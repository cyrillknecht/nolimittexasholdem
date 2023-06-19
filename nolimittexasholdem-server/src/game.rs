use crate::cards::Card;
use crate::player::{Player, PlayerCommunication};
use crate::raw_message::{PlayerChoice, RawMessage};
use async_std::channel::{self, Receiver, Sender};
use async_std::net::TcpListener;
use async_std::prelude::FutureExt;
use async_std::sync::{Mutex, MutexGuard};
use rand::prelude::SliceRandom;
use std::error::Error;
use std::future::Future;
use std::time::{Duration, Instant};

/// Creates an instance of the game and runs it asynchronously.
pub(crate) async fn start(
    stop_condition: impl Future<Output = Result<(), Box<dyn Error>>>,
    port: u16,
    small_blind: usize,
    start_money: usize,
) -> Result<(), Box<dyn Error>> {
    let game = async {
        let game = create_game(port).await?;
        game.run(small_blind, start_money).await?;
        Result::<(), Box<dyn Error>>::Ok(())
    };

    game.race(stop_condition).await?;
    Ok(())
}



/// A game of No Limit Texas Hold'em Poker.
///
/// Round numbers as following:
/// 
/// * Pre round 0: give cards, prepare round
/// 
/// * Round 0: betting
/// 
/// * Round 1: adjust bets
/// 
/// * End of round 1: show 3 cards
/// 
/// * Round 2: betting
/// 
/// * Round 3: adjust betting
/// 
/// * End of round 3: show 4 cards
/// 
/// * Round 4: betting
/// 
/// * Round 5: adjust betting
/// 
/// * End of round 5: show 5 cards
/// 
/// * Round 6: betting
/// 
/// * Round 7: adjust betting
/// 
/// * End of round 7: calculate winner and set round to 0
pub(crate) struct Game {
    players: Vec<Player>,
    open_middle_cards: Vec<Card>,
    card_stack: Vec<Card>,
    receiver: Receiver<PlayerCommunication>,
    round_number: usize,

    /// Players (in order) who play in this round.
    /// Ideally should be connected (but may be not) and should not have folded or be out.
    players_in_round: Vec<usize>,

    /// Starting position is one after the dealer.
    dealer: usize,

    small_blind: usize,
}

impl Game {

    /// Runs the instance of the game.
    async fn run(mut self, small_blind: usize, start_money: usize) -> Result<(), Box<dyn Error>> {
        assert!(!self.players.is_empty());
        self.init(small_blind, start_money).await;
        let winner = loop {
            match self.determine_end_and_game_winner().await {
                None => { self.play_round().await?; }
                Some(w) => break w,
            }
        };

        for p in &mut self.players {
            p.write_message(RawMessage::GameEnd(winner)).await;
        }

        println!("[SERVER] Winner: {:?}", winner);
        Ok(())
    }

    /// Inits the game once per instance by giving the players coins.
    async fn init(&mut self, small_blind: usize, start_money: usize) {
        self.small_blind = small_blind;
        for p in &mut self.players {
            p.set_coins(start_money);
        }
    }

    /// Determines if the round ends and if it ends the winner of the round.
    /// 
    /// Return values:
    /// * None => Game not finished
    /// * Some(None) => Game finished, no winner
    /// * Some(Some(w)) => Game finished, w won
    async fn determine_end_and_game_winner(&self) -> Option<Option<usize>> {
        // Game finished with no winner unless someone is still playing
        let mut game_finished = Some(None);

        for i in 0..self.players.len() {
            let p = &self.players[i];
            if !p.is_out && p.is_connected() {
                match game_finished {
                    // If no other winner yet, set winner
                    // None case should never happen because game_finished = None is never set
                    None | Some(None) => game_finished = Some(Some(i)),
                    // Someone else is also still in
                    Some(Some(_)) => return None,
                }
            }
        }
        game_finished
    }

    /// Inits the first round after starting the server or showdown.
    /// Sets all player cards, empties open middle cards and sets blinds.
    async fn init_round_0(&mut self) {
        self.card_stack.clear();
        self.open_middle_cards.clear();
        // There are no more than 6 players, cards always suffice
        let card_amount = 52;
        for i in 0..card_amount {
            self.card_stack.push(Card { value: i });
        }
        self.card_stack.shuffle(&mut rand::thread_rng());
        for i in &self.players_in_round {
            let p = &mut self.players[*i];
            // Assert clean for each player
            assert_eq!(p.cards.len(), 0);
            assert_eq!(p.current_betting_amount(), 0);
            for _ in 0..2 {
                p.cards.push(self.card_stack.pop().unwrap());
            }
        }

        // First player after dealer pays blind
        if let Some(index) = self.players_in_round.get(0) {
            let p = &mut self.players[*index];
            Self::try_set_player_bet(self.small_blind, p, self.small_blind, self.small_blind);
        }
        // Second player after dealer pays blind * 2
        if let Some(index) = self.players_in_round.get(1) {
            let p = &mut self.players[*index];
            Self::try_set_player_bet(self.small_blind, p, 2 * self.small_blind, self.small_blind);
        }
    }

    /// The general structure of one poker round.
    async fn play_round(&mut self) -> Result<(), Box<dyn Error>> {
        self.get_qualified_players();
        self.foreplay().await;
        self.main_play().await?;
        self.afterplay().await;
        Ok(())
    }

    /// The part before the actual playing.
    async fn foreplay(&mut self) {
        match self.round_number {
            0 => { self.init_round_0().await; }
            _ => {}
        }
        self.broadcast().await;
    }

    /// The actual part of playing (raise, check, etc.).
    async fn main_play(&mut self) -> Result<(), Box<dyn Error>> {
        // Assert that no player in the round has folded
        // players_in_round only contains players which have not folded before playing
        assert!({
            self.players_in_round
                .iter()
                .map(|i| !self.players[*i].has_folded)
                .fold(true, |a, b| (a && b))
        });
        if self.players_in_round.len() <= 1 {
            println!("[SERVER] Not enough players in, skipping round {}", self.round_number);
            // Not enough players, skip
            return Ok(());
        }

        match self.round_number {
            0 | 2 | 4 | 6 => {
                // Let players bet
                for i in 0..self.players_in_round.len() {
                    let player_id = self.players_in_round[i];
                    let highest_bet = self.get_highest_bet_in_round();
                    let p = &mut self.players[player_id];
                    let message = Self::await_player_response(&self.receiver, p, player_id).await?;
                    println!("[SERVER] game.rs: id: {}, Got {:?}", player_id, message);
                    match message {
                        RawMessage::PlayerChoice(PlayerChoice::RaiseTo(amount)) => {
                            Self::try_set_player_bet(self.small_blind, p, amount, highest_bet);
                        }
                        RawMessage::PlayerChoice(PlayerChoice::Pass) => {
                            Self::try_set_player_bet( self.small_blind,  p, p.current_betting_amount(), highest_bet);
                        }
                        _ => {
                            // All action which is not raise is considered fold!
                            // So that players who do not play are automatically excluded from the rounds
                            p.has_folded = true;
                        }
                    };
                    self.broadcast().await;
                }
            }
            1 | 3 | 5 | 7 => {
                // Let players adjust bets
                for i in 0..self.players_in_round.len() {
                    let player_id = self.players_in_round[i];
                    let highest_bet = self.get_highest_bet_in_round();
                    let p = &mut self.players[player_id];
                    // Only necessary if bets are unequally high
                    if p.current_betting_amount() != highest_bet {
                        let message =
                            Self::await_player_response(&self.receiver, p, player_id).await?;
                        match message {
                            RawMessage::PlayerChoice(PlayerChoice::RaiseTo(_) | PlayerChoice::Pass) => {
                                Self::try_set_player_bet(self.small_blind, p, highest_bet, highest_bet);
                            }
                            RawMessage::PlayerChoice(PlayerChoice::Fold) | _ => {
                                // All action which is not raise is considered fold!
                                // So that players who do not play are automatically excluded from the rounds
                                p.has_folded = true;
                            }
                        };
                    }
                    self.broadcast().await;
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Tries to set a players bet to a specified amount.
    /// Bet may not exceed players coin amount.
    /// Bet should be higher or equal than highest bet of others (minimal_amount).
    /// Bet should be multiple of small blind.
    fn try_set_player_bet(current_blind: usize, p: &mut Player, mut amount: usize, minimal_amount: usize) {
        // Must raise to at least amount of all previous players
        // Or all in

        // Amount must be higher or equal than previous amount
        amount = std::cmp::max(amount, p.current_betting_amount());

        // Amount must be higher or equal than all other players' bets
        amount = std::cmp::max(amount, minimal_amount);

        // Amount must be multiple of small blind
        // Round down if too big
        let remainder = amount % current_blind;
        if amount > current_blind && usize::MAX - current_blind < amount {
            amount -= remainder;
        } else {
            if remainder != 0 {
                amount += current_blind - remainder;
            }
        }

        // Amount may not be higher than current owned amount
        amount = std::cmp::min(amount, p.coins() + p.current_betting_amount());

        // Make player bet
        p.deduct_from_money_to_bet(amount);
    }

    /// Represents the game path after every round played.
    async fn afterplay(&mut self) {
        // If last_player is None, then there are more than two players left
        // If last_player is Some(i), the i is the last player left
        // If last_player is Some(None), then there are no players left
        let mut last_player = Some(None);
        for i in &self.players_in_round {
            if !self.players[*i].has_folded {
                match last_player {
                    None => break,
                    Some(t) => match t {
                        Some(_) => {
                            last_player = None;
                            break;
                        }
                        None => last_player = Some(Some(*i)),
                    },
                }
            }
        }

        if let Some(maybe_player) = last_player {
            println!("[SERVER] Less than two players left, making fast afterplay. Round {}", self.round_number);
            self.early_end(maybe_player).await;
            self.broadcast_intern(maybe_player, true).await;
            self.unfold_and_out_players();
            self.round_number = 0;
            self.move_dealer_and_increase_blind();
            for p in &mut self.players {
                p.cards.clear();
            }
            // Let everyone look at stats
            async_std::task::sleep(Duration::from_secs(7)).await;
            self.broadcast().await;
        } else {
            // Enough players remain to continue normally
            match self.round_number {
                1 => {
                    // Show 3 cards
                    self.open_middle_cards.push(self.card_stack.pop().unwrap());
                    self.open_middle_cards.push(self.card_stack.pop().unwrap());
                    self.open_middle_cards.push(self.card_stack.pop().unwrap());
                }
                3 | 5 => {
                    self.open_middle_cards.push(self.card_stack.pop().unwrap());
                }
                0 | 2 | 4 | 6 => { /* Do nothing */ }
                // Everything else symbolises game end
                7 | _ => {
                    self.determine_winner_and_payout().await;

                    for p in &mut self.players {
                        p.cards.clear();
                    }
                    self.unfold_and_out_players();
                }
            }
            self.determine_next_round_and_move_dealer().await;
            self.broadcast().await;
        }
    }

    /// Represents the game path if there are not enough players during a round.
    async fn early_end(&mut self, maybe_last: Option<usize>) {
        if let Some(winner) = maybe_last {
            println!("[SERVER] Only one player remained. id: {}", winner);
            let mut pot = 0;
            let max = self.players[winner].current_betting_amount();
            for p in &mut self.players {
                pot += p.take_betting_amount(max);
            }
            self.players[winner].add_coins(pot);
        } else {
            println!("[SERVER] No player remained");
            for p in &mut self.players {
                // Return all investments
                p.take_betting_amount(0);
            }
        }
    }

    /// Unfolds and outs all players.
    fn unfold_and_out_players(&mut self) {
        for p in &mut self.players {
            if p.coins() == 0 || !p.is_connected() {
                p.is_out = true;
            }
            p.has_folded = false;
        }
    }

    /// Determines the winner and the corresponding payout.
    async fn determine_winner_and_payout(&mut self) {
        // There is only one winner
        // If multiple people win, no one wins!
        let mut winner = None;
        let mut card_combo = 0;
        let mut too_many_winner = false;
        for i in &self.players_in_round {
            let p = &mut self.players[*i];
            // Player might have folded
            if !p.has_folded {
                match winner {
                    None => {
                        winner = Some(*i);
                        card_combo = p.determine_card_value(&self.open_middle_cards);
                        p.end_of_round_values = Some(card_combo);
                    }
                    Some(_) => {
                        // If the current val is not more than the previous players,
                        // the player has no incentive to show what cards he had
                        let curr_val = p.determine_card_value(&self.open_middle_cards);
                        if card_combo == curr_val {
                            p.end_of_round_values = Some(curr_val);
                            too_many_winner = true;
                        } else if curr_val > card_combo {
                            too_many_winner = false;
                            winner = Some(*i);
                            card_combo = curr_val;
                            p.end_of_round_values = Some(curr_val);
                        }
                    }
                }
            }
        }
        println!("Too many winners: {}", too_many_winner);
        self.broadcast_intern(winner, true).await;
        // Wait 4 seconds for everyone to see the cards before moving money
        async_std::task::sleep(Duration::from_secs(10)).await;
        for p in &mut self.players {
            p.end_of_round_values = None;
        }

        if let Some(winner) = winner {
            if !too_many_winner {
                let mut money_won = 0;
                // Win no more than player has bet
                // Might bet less than others in "all in" case
                // Most cases won't be affected by this
                let maximal_win = self.players[winner].current_betting_amount();
                for p in &mut self.players {
                    let stolen_money = p.take_betting_amount(maximal_win);
                    money_won += stolen_money;
                }
                self.players[winner].add_coins(money_won);
            }
        } else {
            for p in &mut self.players {
                // Return coins to players
                p.take_betting_amount(0);
            }
        }
        self.broadcast().await;
    }

    /// Retrieves the highest bet in the round.
    fn get_highest_bet_in_round(&self) -> usize {
        self.players
            .iter()
            .map(|p| p.current_betting_amount())
            .fold(0, std::cmp::max)
    }

    /// Determines the next round and moves the dealer.
    async fn determine_next_round_and_move_dealer(&mut self) {
        self.round_number += 1;
        // There are only eight rounds
        if self.round_number >= 8 {
            // If round has ended, move dealer
            self.move_dealer_and_increase_blind();
            self.round_number = 0;
            // Reset fold state
            for p in &mut self.players {
                if !p.is_out {
                    p.has_folded = false;
                }
            }
        }
    }

    /// Moves the dealer to the next player and increases the blind.
    fn move_dealer_and_increase_blind(&mut self) {
        // If no one remains, this will not be an infinity loop
        for _ in 0..self.players.len() {
            self.dealer += 1;
            self.dealer %= self.players.len();
            if self.dealer == 0 {
                // Increase blind if dealer passes position 0
                self.small_blind *= 2;
            }
            if !self.players[self.dealer].is_out {
                // If person which can deal is found, stop
                break;
            }
        }
    }

    /// Internal function for broadcasting the game state.
    async fn broadcast_intern(&mut self, hand_winner: Option<usize>, is_showdown: bool) {
        let middle_cards: Vec<Card> = self.open_middle_cards.clone();
        let player_names: Vec<String> = self
            .players
            .iter()
            .map(|p| p.display_name.to_string())
            .collect();
        let mut player_cards: Vec<Option<[Card; 2]>> = vec![None; self.players.len()];
        for i in 0..self.players.len() {
            if let Some(_) = self.players[i].end_of_round_values {
                player_cards[i] = Some([self.players[i].cards[0], self.players[i].cards[1]]);
            }
        }
        let player_betting_amount: Vec<usize> = self
            .players
            .iter()
            .map(Player::current_betting_amount)
            .collect();
        let player_money: Vec<usize> = self.players.iter().map(|p| p.coins()).collect();
        let player_has_folded: Vec<bool> = self.players.iter().map(|p| p.has_folded).collect();
        let player_is_out: Vec<bool> = self.players.iter().map(|p| p.is_out).collect();

        for p in &mut self.players {
            let personal_cards = if p.cards.len() >= 2 {
                [p.cards[0], p.cards[1]]
            } else {
                [Card { value: 0 }, Card { value: 0 }]
            };
            let msg = RawMessage::GameStatus {
                personal_cards,
                personal_id: p.player_id(),
                middle_cards: middle_cards.clone(),
                player_names: player_names.clone(),
                player_cards: player_cards.clone(),
                player_betting_amount: player_betting_amount.clone(),
                player_money: player_money.clone(),
                player_has_folded: player_has_folded.clone(),
                player_is_out: player_is_out.clone(),
                round_number: self.round_number,
                is_started: true,
                hand_winner: if let Some(t) = hand_winner {
                    t as i8
                } else {
                    -1 as i8
                },
                is_showdown
            };

            p.write_message(msg).await;
        }
        // TODO: change ResponseListenerThread.cpp to accept message faster (it overreads)
        // This is an issue with the Lama Game, it's not really ours to fix...
        async_std::task::sleep(Duration::from_millis(100)).await;
    }

    /// Sends the game state to all connected players.
    async fn broadcast(&mut self) {
        self.broadcast_intern(None, false).await;
    }

    /// Queries all players who can play the current round in a list.
    /// The queried list is in correct playing order.
    fn get_qualified_players(&mut self) {
        self.players_in_round.clear();
        let mut cur = self.dealer;
        // Dealer is the last one in round
        for _ in 0..self.players.len() {
            cur += 1;
            cur %= self.players.len();

            // If player has neither folded nor is out, player is in
            if !self.players[cur].is_out && !self.players[cur].has_folded {
                self.players_in_round.push(cur);
            }
        }
    }

    /// Awaits for player response.
    async fn await_player_response(
        receiver: &Receiver<PlayerCommunication>,
        curr_player: &mut Player,
        current_index: usize,
    ) -> Result<RawMessage, Box<dyn Error>> {
        curr_player.write_message(RawMessage::AwaitingPlayer).await;
        let start_turn_time = Instant::now();
        // Can be used to limit the time a player has to play
        // Currently very high, to facilitate testing
        const MAX_TIME: u64 = 10000;
        let max_time = Duration::from_secs(MAX_TIME);

        // If disconnected, we immediately have a response
        if !curr_player.is_connected() {
            // Always go all in if not connected
            return Ok(RawMessage::PlayerChoice(PlayerChoice::RaiseTo(usize::MAX)));
        }
        let mut got_response = false;

        let mut response = RawMessage::ConnectionEnded;
        while !got_response {
            let now = Instant::now();
            if start_turn_time + max_time <= now {
                // Time out, player either disconnected or not responding
                got_response = true;
            } else {
                let try_time = max_time - (now - start_turn_time);
                let maybe_timeout = receiver.recv().timeout(try_time).await;
                match maybe_timeout {
                    Ok(info) => {
                        match info {
                            Ok(com) => {
                                let sender = com.sender;
                                // Ignore all other messages
                                if sender == current_index {
                                    match com.message {
                                        RawMessage::PlayerChoice(_) => {
                                            response = com.message;
                                            got_response = true;
                                        }
                                        RawMessage::ConnectionEnded => {
                                            // Wait no longer
                                            got_response = true;
                                        }
                                        _ => { /* Ignore all other messages */ }
                                    }
                                }
                            }
                            Err(_) => {
                                // Error on receiving from channel
                                panic!("Error on receiving from stream");
                            }
                        }
                    }
                    Err(_) => {
                        // Timed out
                        got_response = true;
                    }
                }
            }
        }
        Ok(response)
    }
}

/// Waits for all players to connect and sets their status to ready. Afterwards creates a game and return it.
async fn create_game(port: u16) -> Result<Game, Box<dyn Error>> {
    let (sender, receiver) = channel::unbounded();
    let players = Mutex::new(vec![]);
    let accept_players = accept_players(sender, &players, port);
    let wait_for_ready = wait_for_ready(&receiver, &players);

    let lock = accept_players.race(wait_for_ready).await.unwrap();
    // Lock must be kept until the TcpListener is closed!
    drop(lock);

    Ok(Game {
        players: players.into_inner(),
        open_middle_cards: vec![],
        card_stack: vec![],
        receiver,
        round_number: 0,
        players_in_round: vec![],
        dealer: 0,
        small_blind: 0,
    })
}

/// Awaits for connected to set their status to ready.
async fn wait_for_ready<'a>(
    receiver: &Receiver<PlayerCommunication>,
    players: &'a Mutex<Vec<Player>>,
) -> Result<MutexGuard<'a, Vec<Player>>, Box<dyn Error>> {
    loop {
        let PlayerCommunication { message, sender } = receiver.recv().await?;
        match message {
            RawMessage::IsReady => {
                let mut lock = players.lock().await;
                lock[sender].begin_game = true;

                // If more than one player, and everyone is ready, start game
                if lock.len() > 1 {
                    let mut begin = true;
                    for p in lock.iter() {
                        if !p.begin_game {
                            begin = false;
                        }
                    }

                    if begin {
                        return Ok(lock);
                    }
                }
            }
            RawMessage::SetDisplayName(name) => {
                let mut lock = players.lock().await;
                lock[sender].display_name = name;
            }
            _ => { /* Ignore other unknown messages */ }
        }
    }
}

/// Awaits for new players to connect.
async fn accept_players<T>(
    sender: Sender<PlayerCommunication>,
    players: &Mutex<Vec<Player>>,
    port: u16,
) -> Result<T, Box<dyn Error>> {
    println!("Listening on: {}:{}", "0.0.0.0", port);
    let incoming = TcpListener::bind("0.0.0.0:".to_string() + &port.to_string()).await?;
    loop {
        let (new_one, address) = incoming.accept().await.unwrap();

        let mut lock = players.lock().await;
        println!("[SERVER] New player arrived at {}", address);
        let player_id = lock.len();

        let mut player = Player::new(new_one, "".to_string(), player_id, sender.clone()).unwrap();

        player
            .write_message(RawMessage::GameStatus {
                personal_cards: [Card { value: 0 }, Card { value: 0 }],
                personal_id: player_id,
                middle_cards: vec![Card { value: 0 }, Card { value: 0 }, Card { value: 0 }],
                player_names: vec!["".to_string(); player_id + 1],
                player_cards: vec![None; player_id + 1],
                player_betting_amount: vec![0; player_id + 1],
                player_money: vec![0; player_id + 1],
                player_has_folded: vec![false; player_id + 1],
                player_is_out: vec![false; player_id + 1],
                round_number: 0,
                is_started: false,
                hand_winner: -1,
                is_showdown: false
            })
            .await;

        lock.push(player);
        drop(lock);
        //No more than 6 players
        if player_id == 5 {
            async_std::future::pending::<()>().await;
        }
    }
}
