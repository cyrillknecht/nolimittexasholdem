use crate::cards::Card;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;
use serde_json::{json, Value};
use std::error::Error;
use std::str::FromStr;

/// An enumeration type for representing the types of messages.
#[derive(Debug, PartialEq, Clone)]
pub enum RawMessage {
    SetDisplayName(String),
    IsReady,
    Heartbeat,
    PlayerChoice(PlayerChoice),
    ConnectionEnded,
    AwaitingPlayer,
    GameStatus {
        personal_cards: [Card; 2],
        personal_id: usize,
        middle_cards: Vec<Card>,
        player_names: Vec<String>,
        player_cards: Vec<Option<[Card; 2]>>,
        player_betting_amount: Vec<usize>,
        player_money: Vec<usize>,
        player_has_folded: Vec<bool>,
        player_is_out: Vec<bool>,
        round_number: usize,
        is_started: bool,
        hand_winner: i8,
        is_showdown: bool
    },
    GameEnd(Option<usize>),
}

/// An enumeration type for representing the choices of the player during a poker round.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PlayerChoice {
    RaiseTo(usize),
    Fold,
    Pass,
}

impl Into<Value> for RawMessage {
    fn into(self) -> Value {
        match self {
            Self::SetDisplayName(s) => json!({"type": "set_display_name", "player_name" : s}),
            Self::IsReady => json!({"type": "is_ready"}),
            Self::Heartbeat => json!({"type": "heartbeat"}),
            Self::PlayerChoice(s) => match s {
                PlayerChoice::RaiseTo(val) => {
                    json!({"type": "response",
                        "action" : "raise_to",
                        "amount" : val
                    })
                }
                PlayerChoice::Fold => {
                    json!({"type": "response",
                        "action" : "fold"
                    })
                }
                PlayerChoice::Pass => {
                    json!({"type": "response",
                        "action" : "pass",
                    })
                }
            },
            Self::ConnectionEnded => json!({"type": "connection_ended"}),
            Self::AwaitingPlayer => json!({"type": "awaiting_player"}),
            Self::GameStatus {
                personal_cards,
                personal_id,
                middle_cards,
                player_names,
                player_cards,
                player_betting_amount,
                player_money,
                player_has_folded,
                player_is_out,
                round_number,
                is_started,
                hand_winner,
                is_showdown
            } => {
                let personal_cards = (personal_cards[0].value, personal_cards[1].value);
                let middle_cards: Vec<u8> = middle_cards.into_iter().map(|c| c.value).collect();
                let player_cards: Vec<Option<(u8, u8)>> = player_cards
                    .into_iter()
                    .map(|c| {
                        if let Some(t) = c {
                            Some((t[0].value, t[1].value))
                        } else {
                            None
                        }
                    })
                    .collect();
                json!({
                    "type": "game_state",
                    "personal_cards": personal_cards,
                    "personal_id": personal_id,
                    "middle_cards": middle_cards,
                    "player_names": player_names,
                    "player_cards": player_cards,
                    "player_betting_amount": player_betting_amount,
                    "player_money": player_money,
                    "player_has_folded": player_has_folded,
                    "player_is_out": player_is_out,
                    "round_number": round_number,
                    "is_started": is_started,
                    "hand_winner": hand_winner,
                    "is_showdown": is_showdown
                })
            }
            Self::GameEnd(winner) => {
                json!({"type": "game_end", "winner":winner})
            }
        }
    }
}

impl TryFrom<Value> for RawMessage {
    type Error = serde_json::Error;

    fn try_from(mut value: Value) -> Result<RawMessage, Self::Error> {
        use serde_json::value::from_value;

        match from_value::<String>(value["type"].take())?.as_str() {
            "is_ready" => Ok(Self::IsReady),
            "heartbeat" => Ok(Self::Heartbeat),
            "awaiting_player" => Ok(Self::AwaitingPlayer),
            "game_end" => Ok(Self::GameEnd(from_value(value["winner"].take())?)),
            "set_display_name" => Ok(Self::SetDisplayName(from_value(
                value["player_name"].take(),
            )?)),
            "connection_ended" => Ok(Self::ConnectionEnded),
            "response" => {
                match from_value::<String>(value["action"].take())?.as_str() {
                    "raise_to" => Ok(Self::PlayerChoice(PlayerChoice::RaiseTo(
                        from_value::<u64>(value["amount"].take())? as usize,
                    ))),
                    "fold" => Ok(Self::PlayerChoice(PlayerChoice::Fold)),
                    "pass" => Ok(Self::PlayerChoice(PlayerChoice::Pass)),
                    _ => {
                        // Will return an Err
                        // Must use weird logic here, as constructing an error is not possible
                        // serde_json has made error creation private
                        println!("Error at parsing {:?}", value);
                        from_value::<String>(Value::Null)?;
                        panic!("Should not reach this code");
                    }
                }
            }
            "game_state" => {
                let u64_to_card: fn(&u64) -> Card = |c| Card { value: *c as u8 };

                let personal_cards = from_value::<[u64; 2]>(value["personal_cards"].take())?;
                let personal_cards = [
                    u64_to_card(&personal_cards[0]),
                    u64_to_card(&personal_cards[1]),
                ];

                let personal_id: usize = from_value(value["personal_id"].take())?;

                let middle_cards: Vec<Card> = from_value::<Vec<u64>>(value["middle_cards"].take())?
                    .iter()
                    .map(u64_to_card)
                    .collect();

                let player_names = from_value::<Vec<String>>(value["player_names"].take())?;

                let player_cards: Vec<Option<[Card; 2]>> =
                    from_value::<Vec<Option<[u64; 2]>>>(value["player_cards"].take())?
                        .into_iter()
                        .map(|v| match v {
                            None => None,
                            Some(t) => Some([u64_to_card(&t[0]), u64_to_card(&t[1])]),
                        })
                        .collect();

                let player_betting_amount =
                    from_value::<Vec<u64>>(value["player_betting_amount"].take())?
                        .into_iter()
                        .map(|a| a as usize)
                        .collect();
                let player_money = from_value::<Vec<u64>>(value["player_money"].take())?
                    .into_iter()
                    .map(|a| a as usize)
                    .collect();
                let player_has_folded = from_value::<Vec<bool>>(value["player_has_folded"].take())?;
                let player_is_out = from_value::<Vec<bool>>(value["player_is_out"].take())?;

                let round_number: usize = from_value(value["round_number"].take())?;
                let is_started: bool = from_value(value["is_started"].take())?;
                let hand_winner: i8 = from_value(value["hand_winner"].take())?;
                let is_showdown: bool = from_value(value["is_showdown"].take())?;

                Ok(Self::GameStatus {
                    personal_cards,
                    personal_id,
                    middle_cards,
                    player_names,
                    player_cards,
                    player_betting_amount,
                    player_money,
                    player_has_folded,
                    player_is_out,
                    round_number,
                    is_started,
                    hand_winner,
                    is_showdown
                })
            }
            _ => {
                // Will return an Err
                // Must use weird logic here, as constructing an error is not possible
                // serde_json has made error creation private
                println!("Error at parsing {:?}", value);
                from_value::<String>(Value::Null)?;
                panic!("Should not reach this code");
            }
        }
    }
}

impl RawMessage {
    /// Reads the size of an incoming byte stream of a TcpStream.
    async fn read_size_of_incoming(socket: &mut TcpStream) -> Result<usize, Box<dyn Error>> {
        let mut all_buff = Vec::<u8>::new();
        let mut buff = [0u8; 1024];

        loop {
            let read_amount = socket.peek(&mut buff).await?;
            let mut ter = None;
            for i in 0..read_amount {
                if buff[i] == b':' {
                    ter = Some(i);
                    break;
                }
            }
            if let Some(i) = ter {
                // Null terminator is read
                // But not attached to string!
                socket.read_exact(&mut buff[0..(i + 1)]).await?;
                all_buff.extend_from_slice(&buff[0..i]);
                let string: String = String::from_utf8_lossy(&*all_buff).into();
                return Ok(usize::from_str(string.as_str())?);
            } else {
                // Read the bytes again until terminator found
                // Maybe skip instead?
                socket.read_exact(&mut buff[0..read_amount]).await?;
                all_buff.extend_from_slice(&buff[0..read_amount]);
            }
        }
    }

    /// Utility function for reading from a TcpStream until we reach a zero terminator.
    async fn read_till_terminator(socket: &mut TcpStream) -> Result<String, Box<dyn Error>> {
        let size = Self::read_size_of_incoming(socket).await?;
        let mut buff: Vec<u8> = vec![0; size];
        socket.read_exact(&mut buff[0..size]).await?;
        Ok(String::from_utf8_lossy(&*buff).into())
    }

    /// Writes the content of the RawMessage to a TcpStream.
    pub async fn to_stream(self, socket: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        let t: Value = self.into();
        let s = serde_json::to_string(&t)?;
        // Prepend the length of the message

        let mut message = s.len().to_string();

        message.push(':');
        message.push_str(&s);
        println!("[SERVER]ToStream {}", message);
        socket.write_all(message.as_bytes()).await?;
        socket.flush().await?;
        Ok(())
    }

    /// Creates a RawMessage by reading from a TcpStream.
    pub async fn read_from_stream(socket: &mut TcpStream) -> Result<Self, Box<dyn Error>> {
        let raw_string = Self::read_till_terminator(socket).await?;
        println!("Read: {:?}", raw_string);
        let value_msg: Value = serde_json::from_str(&raw_string)?;
        Ok(RawMessage::try_from(value_msg)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Testing RawMessage to JSON string conversion and vice versa.
    #[test]
    fn test_raw_message_conversion() {
        let messages = vec![
            (
                RawMessage::SetDisplayName("TestUsername1$".to_string()),
                r#"{"type":"set_display_name", "player_name": "TestUsername1$"}"#,
            ),
            (RawMessage::IsReady, r#"{"type":"is_ready"}"#),
            (RawMessage::Heartbeat, r#"{"type":"heartbeat"}"#),
            (
                RawMessage::PlayerChoice(PlayerChoice::RaiseTo(3)),
                r#"{"type": "response", "action" : "raise_to", "amount" : 3}"#,
            ),
            (
                RawMessage::PlayerChoice(PlayerChoice::Fold),
                r#"{"type": "response", "action" : "fold"}"#,
            ),
            (
                RawMessage::PlayerChoice(PlayerChoice::Pass),
                r#"{"type": "response", "action" : "pass"}"#,
            ),
            (
                RawMessage::ConnectionEnded,
                r#"{"type": "connection_ended"}"#,
            ),
            (RawMessage::AwaitingPlayer, r#"{"type": "awaiting_player"}"#),
            (
                RawMessage::GameStatus {
                    personal_cards: [Card::try_from("CA").unwrap(), Card::try_from("D4").unwrap()],
                    personal_id: 1234,
                    middle_cards: vec![
                        Card::try_from("A0").unwrap(),
                        Card::try_from("B1").unwrap(),
                        Card::try_from("A3").unwrap(),
                    ],
                    player_names: vec![
                        "user1".to_string(),
                        "User2".to_string(),
                        "User3$".to_string(),
                    ],
                    player_cards: vec![
                        Some([Card::try_from("BB").unwrap(), Card::try_from("DB").unwrap()]),
                        None,
                        None,
                    ],
                    player_betting_amount: vec![2, 3, 4],
                    player_money: vec![100, 100, 200],
                    player_has_folded: vec![true, false, true],
                    player_is_out: vec![true, true, false],
                    round_number: 2,
                    is_started: true,
                    hand_winner: -1,
                    is_showdown: false
                },
                r#"{
                "type": "game_state",
                "personal_cards": [36, 43],
                "personal_id": 1234,
                "middle_cards": [0, 14, 3],
                "player_names": ["user1", "User2", "User3$"],
                "player_cards": [[24,50], null, null],
                "player_betting_amount": [2, 3, 4],
                "player_money": [100, 100, 200],
                "player_has_folded": [true, false, true],
                "player_is_out": [true, true, false],
                "round_number": 2,
                "is_started": true,
                "hand_winner": -1,
                "is_showdown": false
             }"#,
            ),
            (
                RawMessage::GameEnd(Some(1)),
                r#"{"type": "game_end", "winner": 1}"#,
            ),
        ];

        for (raw_message, json_message) in messages {
            let json: Value = raw_message.clone().into();
            let validation_json: Value = serde_json::from_str(json_message).unwrap();
            assert_eq!(
                json, validation_json,
                "Failed: {:?}. Expected {}, got {}",
                raw_message, json_message, json
            );
            let converted_to_raw_message = RawMessage::try_from(json.clone()).unwrap();
            assert_eq!(
                converted_to_raw_message, raw_message,
                "Failed: {}. Expected {:?}, got {:?}",
                json, raw_message, converted_to_raw_message
            );
        }
    }
}
