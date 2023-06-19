use crate::cards::{self, Card};
use crate::raw_message::RawMessage;
use async_std::channel::Sender;
use async_std::net::TcpStream;
use async_std::prelude::FutureExt;
use async_std::sync::Arc;
use std::error::Error;
use std::net::Shutdown;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Associates a `RawMessage` to a player id of the `Player` who sent it.
pub struct PlayerCommunication {
    /// The sender of the raw message.
    pub sender: usize,

    /// The raw message that the player sent.
    pub message: RawMessage,
}

/// A player associated with the server.
pub struct Player {
    socket: TcpStream,
    player_id: usize,
    pub begin_game: bool,
    pub display_name: String,
    pub cards: Vec<Card>,
    connection_status: Arc<AtomicBool>,
    pub has_folded: bool,
    coins: usize,
    pub is_out: bool,
    current_betting_amount: usize,

    /// The value of the cards when the player wants to show his cards.
    pub end_of_round_values: Option<usize>,
}

impl Player {
    /// Creates a new player and spawns a corresponding reader task.
    pub fn new(
        socket: TcpStream,
        display_name: String,
        player_id: usize,
        sender: Sender<PlayerCommunication>,
    ) -> Result<Self, Box<dyn Error>> {
        let me = Self {
            socket,
            player_id,
            display_name,
            begin_game: false,
            cards: vec![],
            connection_status: Arc::new(AtomicBool::new(true)),
            has_folded: false,
            coins: 0,
            is_out: false,
            current_betting_amount: 0,
            end_of_round_values: None,
        };

        me.spawn_reader_task(player_id, sender)?;
        Ok(me)
    }

    /// Writes a message to the socket.
    pub(crate) async fn write_message(&mut self, msg: RawMessage) {
        if self.is_connected() {
            if let Err(e) = msg.to_stream(&mut self.socket).await {
                println!(
                    "[SERVER] Client {} was unable to be written to {:?}",
                    self.player_id, e
                );
                self.shutdown();
            }
        }
    }

    /// Spawns the reader task.
    fn spawn_reader_task(
        &self,
        player_id: usize,
        sender: Sender<PlayerCommunication>,
    ) -> Result<(), Box<dyn Error>> {
        let connection_status = self.connection_status.clone();
        let socket = self.socket.clone();

        // Task handle is dropped
        // Task is executing on its own
        async_std::task::spawn(async move {
            let res = Self::reader(socket, sender, connection_status, player_id).await;
            match res {
                _ => {}
            }
        });

        Ok(())
    }

    /// Asynchronous reader function of the reader task.
    async fn reader(
        mut socket: TcpStream,
        sender: Sender<PlayerCommunication>,
        connection_status: Arc<AtomicBool>,
        player_id: usize,
    ) {
        match Self::message_loop(&mut socket, &sender, player_id).await {
            Ok(()) => {
                println!("[SERVER] id: {}: Disconnected gracefully", player_id)
            }
            Err(e) => println!("[SERVER] id: {}. Oh no, {:?}", player_id, e),
        }

        Self::shutdown_internal(&socket, &connection_status);

        // Doesn't matter if receiver has been dropped
        let _ = sender
            .send(PlayerCommunication {
                sender: player_id,
                message: RawMessage::ConnectionEnded,
            })
            .await;
    }

    /// Asynchronous message loop of the `Player`.
    async fn message_loop(
        socket: &mut TcpStream,
        sender: &Sender<PlayerCommunication>,
        player_id: usize,
    ) -> Result<(), Box<dyn Error>> {
        // Can be used to determine if a TcpConnection is dead
        // Currently unused, but the heartbeat message can be sent by the client to keep the
        // connection alive, if the value is lowered
        const TIMEOUT: u64 = 3000;
        loop {
            let computed_msg = RawMessage::read_from_stream(socket)
                .timeout(Duration::from_secs(TIMEOUT))
                .await??;
            println!("[SERVER] Received {} {:?}", player_id, computed_msg);
            match computed_msg {
                RawMessage::Heartbeat => {}
                a => {
                    sender
                        .send(PlayerCommunication {
                            sender: player_id,
                            message: a,
                        })
                        .await?;
                }
            }
        }
    }

    /// Determines the value of the hand.
    pub fn determine_card_value(&self, table_cards: &Vec<Card>) -> usize {
        assert_eq!(table_cards.len(), 5);
        let mut tmp = self.cards.clone();
        for x in table_cards {
            tmp.push(x.clone());
        }
        let mut cards: Vec<(u8, u8)> = tmp.into_iter().map(|c| (c.color(), c.value())).collect();
        cards::value_of_hand(&mut cards)
    }

    /// Internal socket shutdown routine.
    fn shutdown_internal(socket: &TcpStream, connection_status: &Arc<AtomicBool>) {
        println!("[SERVER] Shutting down socket");

        let _ = socket.shutdown(Shutdown::Both);
        connection_status.store(false, Ordering::Relaxed)
    }

    /// Shuts down the socket related to the `Player`.
    pub fn shutdown(&self) {
        Self::shutdown_internal(&self.socket, &self.connection_status)
    }

    /// Obtain this `Player`'s identifier.
    pub fn player_id(&self) -> usize {
        self.player_id
    }

    /// Obtain whether this `Player` is connected or not.
    pub fn is_connected(&self) -> bool {
        self.connection_status.load(Ordering::Relaxed)
    }

    /// Obtain this `Player`'s amount of coins.
    pub fn coins(&self) -> usize {
        self.coins
    }

    /// Obtain this `Player`'s current betting amount.
    pub fn current_betting_amount(&self) -> usize {
        self.current_betting_amount
    }

    /// Subtracts the betting amount (but not more than max_amount) and returns it.
    /// If any money is left over it is added back to the `Player`'s coins.
    /// Resets the betting amount to zero.
    pub fn take_betting_amount(&mut self, max_amount: usize) -> usize {
        let amount_deduced = std::cmp::min(self.current_betting_amount, max_amount);
        self.current_betting_amount -= amount_deduced;
        self.coins += self.current_betting_amount;
        self.current_betting_amount = 0;
        amount_deduced
    }

    /// Add a specified amount of coins to the `Player`'s balance.
    pub fn add_coins(&mut self, amount: usize) {
        self.coins += amount;
    }

    /// Set the `Player`'s amount of coins.
    pub fn set_coins(&mut self, coins: usize) {
        self.coins = coins;
    }

    /// Deduct money from the `Player`'s balance for betting.
    pub fn deduct_from_money_to_bet(&mut self, amount: usize) {
        self.coins += self.current_betting_amount;
        assert!(self.coins >= amount);
        self.coins -= amount;
        self.current_betting_amount = amount;
    }
}
