use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;

/// A playing card.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    pub value: u8,
}

impl Card {
    /// Returns the Card's color.
    pub fn color(&self) -> u8 {
        self.value / 13
    }

    /// Returns the Card's value.
    pub fn value(&self) -> u8 {
        self.value % 13
    }
}

impl From<(u8, u8)> for Card {
    fn from(value: (u8, u8)) -> Self {
        Self {
            value: value.0 * 13 + value.1,
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c: Vec<_> = value.chars().collect();
        if c.len() != 2 {
            Err(c.len().to_string())
        } else {
            let col = match c[0] {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                p => return Err(format!("No color: {}", p).to_string()),
            };

            let val = u8::from_str_radix(c[1].to_string().as_str(), 13);
            match val {
                Ok(val) => Ok(Card::from((col, val))),
                Err(_) => Err("Conversion failed".into()),
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const SUITS: [&str; 4] = ["♣︎", "♦︎", "♥︎", "♠︎"];
        const VALUES: [&str; 13] = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];
        write!(
            f,
            "{}{}",
            VALUES[self.value() as usize],
            SUITS[self.color() as usize]
        )
    }
}

/// Returns the value of the hand.
pub fn value_of_hand(cards: &mut Vec<(u8, u8)>) -> usize {
    assert_eq!(cards.len(), 7);

    cards.sort_by(|card1, card2| card1.1.cmp(&card2.1));

    let mut hand_value = 0;
    hand_value = max(calculate_high_card(cards), hand_value);
    hand_value = max(calculate_pair(cards), hand_value);
    hand_value = max(calculate_two_pair(cards), hand_value);
    hand_value = max(calculate_three(cards), hand_value);
    hand_value = max(calculate_straight(cards), hand_value);
    hand_value = max(calculate_flush(cards), hand_value);
    hand_value = max(calculate_full_house(cards), hand_value);
    hand_value = max(calculate_four(cards), hand_value);
    hand_value = max(calculate_straight_flush(cards), hand_value);
    hand_value
}

/// Checks whether the provided card stack is a straight flush.
fn calculate_straight_flush(cards: &Vec<(u8, u8)>) -> usize {
    for i in (4..=6).rev() {
        let color = cards[i].0;
        let value = cards[i].1;

        if (color, value - 1) == cards[i - 1]
            && (color, value - 2) == cards[i - 2]
            && (color, value - 3) == cards[i - 3]
            && (color, value - 4) == cards[i - 4]
        {
            return calculate_value(STRAIGHT_FLUSH_OFFSET, 0, 0, 0, 0, value);
        }
    }
    0
}

/// Checks whether the provided card stack is a four-of-a-kind.
fn calculate_four(cards: &Vec<(u8, u8)>) -> usize {
    for i in (3..=6).rev() {
        if cards[i].1 == cards[i - 1].1
            && cards[i].1 == cards[i - 2].1
            && cards[i].1 == cards[i - 3].1
        {
            let kicker = if i == 6 { cards[2].1 } else { cards[6].1 };
            return calculate_value(FOUR_OFFSET, 0, 0, 0, cards[i].1, kicker);
        }
    }
    0
}

/// Checks whether the provided card stack is a full house.
fn calculate_full_house(cards: &Vec<(u8, u8)>) -> usize {
    let mut three_index = 7;
    let mut pair_index = 7;

    let mut i = 6;
    while i > 0 {
        if i > 1 && cards[i].1 == cards[i - 1].1 && cards[i].1 == cards[i - 2].1 {
            three_index = i;
            i -= 2;
        } else if cards[i].1 == cards[i - 1].1 {
            pair_index = i;
            i -= 1;
        }
        if i != 0 {
            i -= 1;
        }
    }

    if three_index == 7 || pair_index == 7 {
        return 0;
    }

    return calculate_value(
        FULL_HOUSE_OFFSET,
        0,
        0,
        0,
        cards[three_index].1,
        cards[pair_index].1,
    );
}

/// Checks whether the provided card stack is a flush.
fn calculate_flush(cards: &Vec<(u8, u8)>) -> usize {
    let mut occurrences: [u8; 4] = [0, 0, 0, 0];
    cards
        .iter()
        .for_each(|&card| occurrences[card.0 as usize] += 1);
    let max = occurrences
        .iter()
        .enumerate()
        .max_by(|&tuple1, &tuple2| (tuple1.1).cmp(tuple2.1))
        .unwrap();

    if *(max.1) < 5 {
        return 0;
    }

    let flush = cards
        .clone()
        .iter()
        .filter(|&&card| card.0 == max.0 as u8)
        .map(|&card| card.1)
        .rev()
        .collect::<Vec<u8>>();

    calculate_value(
        FLUSH_OFFSET,
        flush[0],
        flush[1],
        flush[2],
        flush[3],
        flush[4],
    )
}

/// Checks whether the provided card stack is a straight.
fn calculate_straight(cards: &Vec<(u8, u8)>) -> usize {
    let values = cards.iter().map(|card| card.1).collect::<Vec<u8>>();
    for i in [values[6], values[5], values[4]] {
        if i > 3
            && values.contains(&(i - 1))
            && values.contains(&(i - 2))
            && values.contains(&(i - 3))
            && values.contains(&(i - 4))
        {
            return calculate_value(STRAIGHT_OFFSET, 0, 0, 0, 0, i);
        }
    }
    if values.contains(&(3))
        && values.contains(&(2))
        && values.contains(&(1))
        && values.contains(&(0))
        && values.contains(&(12))
    {
        return calculate_value(STRAIGHT_OFFSET, 0, 0, 0, 0, 3);
    }
    0
}

/// Checks whether the provided card stack is a three-of-a-kind.
fn calculate_three(cards: &Vec<(u8, u8)>) -> usize {
    for i in (2..=6).rev() {
        if cards[i].1 == cards[i - 1].1 && cards[i].1 == cards[i - 2].1 {
            let kicker1 = if i == 6 { cards[3].1 } else { cards[6].1 };
            let kicker2 = if i == 6 || i == 5 {
                cards[2].1
            } else {
                cards[5].1
            };
            return calculate_value(THREE_OFFSET, 0, 0, cards[i].1, kicker1, kicker2);
        }
    }
    0
}

/// Checks whether the provided card stack is a two-pair.
fn calculate_two_pair(cards: &Vec<(u8, u8)>) -> usize {
    let mut pairs = [7, 7];

    let mut i = 6;
    while i > 0 {
        if cards[i].1 == cards[i - 1].1 {
            if pairs[0] == 7 {
                pairs[0] = i;
                i -= 1;
                continue;
            } else {
                pairs[1] = i;
                break;
            }
        }
        i -= 1;
    }

    if pairs[1] == 7 {
        return 0;
    }

    let kicker = if pairs[0] != 6 {
        cards[6].1
    } else if pairs[1] != 4 {
        cards[4].1
    } else {
        cards[2].1
    };
    return calculate_value(
        TWO_PAIR_OFFSET,
        0,
        0,
        cards[pairs[0]].1,
        cards[pairs[1]].1,
        kicker,
    );
}

/// Checks whether the provided card stack is a pair.
fn calculate_pair(cards: &Vec<(u8, u8)>) -> usize {
    for i in (1..=6).rev() {
        if cards[i].1 == cards[i - 1].1 {
            let kicker1 = if i == 6 { cards[4].1 } else { cards[6].1 };
            let kicker2 = if i == 6 || i == 5 {
                cards[3].1
            } else {
                cards[5].1
            };
            let kicker3 = if i == 6 || i == 5 || i == 4 {
                cards[2].1
            } else {
                cards[4].1
            };
            return calculate_value(PAIR_OFFSET, 0, cards[i].1, kicker1, kicker2, kicker3);
        }
    }
    0
}

/// Checks whether the provided card stack is a high-card.
fn calculate_high_card(cards: &Vec<(u8, u8)>) -> usize {
    calculate_value(
        HIGH_CARD_OFFSET,
        cards[6].1,
        cards[5].1,
        cards[4].1,
        cards[3].1,
        cards[2].1,
    )
}

/// Calculates the value for internal representation.
fn calculate_value(
    classification: usize,
    order4: u8,
    order3: u8,
    order2: u8,
    order1: u8,
    order0: u8,
) -> usize {
    classification
        + (order4 as usize) * (13 * 13 * 13 * 13)
        + (order3 as usize) * (13 * 13 * 13)
        + (order2 as usize) * (13 * 13)
        + (order1 as usize) * (13)
        + (order0 as usize)
}

/// The base offset for values of hands with type high card.
/// It represents the general base offset as it is the smallest hand.
pub const HIGH_CARD_OFFSET: usize = 0;

/// The base offset for values of hands with type pair.
/// It represents the next number after all possible high card values.
pub const PAIR_OFFSET: usize = HIGH_CARD_OFFSET + 13_usize.pow(5);

/// The base offset for values of hands with type two pair.
/// It represents the next number after all possible pair values.
pub const TWO_PAIR_OFFSET: usize = PAIR_OFFSET + 13_usize.pow(4);

/// The base offset for values of hands with type three.
/// It represents the next number after all possible two pair values.
pub const THREE_OFFSET: usize = TWO_PAIR_OFFSET + 13_usize.pow(3);

/// The base offset for values of hands with type straight.
/// It represents the next number after all possible three values.
pub const STRAIGHT_OFFSET: usize = THREE_OFFSET + 13_usize.pow(3);

/// The base offset for values of hands with type flush.
/// It represents the next number after all possible straight values.
pub const FLUSH_OFFSET: usize = STRAIGHT_OFFSET + 13_usize.pow(1);

/// The base offset for values of hands with type full house.
/// It represents the next number after all possible flush values.
pub const FULL_HOUSE_OFFSET: usize = FLUSH_OFFSET + 13_usize.pow(5);

/// The base offset for values of hands with type four.
/// It represents the next number after all possible flush values.
pub const FOUR_OFFSET: usize = FULL_HOUSE_OFFSET + 13_usize.pow(2);

/// The base offset for values of hands with type straight flush.
/// It represents the next number after all possible four values.
pub const STRAIGHT_FLUSH_OFFSET: usize = FOUR_OFFSET + 13_usize.pow(2);

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    /// Utility function for testing a string of a card combination.
    fn test_combination(s: &str, expected: usize) {
        let iter = s
            .split(" ")
            .map(Card::try_from)
            .map(Result::unwrap)
            .map(|card| (card.color(), card.value()));
        for mut per in iter.permutations(7) {
            let value = value_of_hand(&mut per);
            assert_eq!(
                expected, value,
                "Failed: {:?}. Expected {}, got {}",
                per, expected, value
            );
        }
    }

    /// Testing allowed card combinations.
    #[test]
    fn test_possible_card_combinations() {
        let test_cases = vec![
            // Normal cases
            (
                "A0 B1 C3 D6 A7 B9 CA",
                HIGH_CARD_OFFSET + (28561 * 10) + (2197 * 9) + (169 * 7) + (13 * 6) + 3,
            ),
            (
                "A9 B1 C3 D6 A7 B9 CA",
                PAIR_OFFSET + (2197 * 9) + (169 * 10) + (13 * 7) + 6,
            ),
            (
                "A9 B1 C3 D3 A7 B9 CA",
                TWO_PAIR_OFFSET + (169 * 9) + (13 * 3) + 10,
            ),
            (
                "A0 BB CB D6 A7 BB CA",
                THREE_OFFSET + (169 * 11) + (13 * 10) + 7,
            ),
            ("AC B1 C2 D3 A4 B5 CA", STRAIGHT_OFFSET + 5),
            (
                "A0 B1 B3 B6 B7 B0 CA",
                FLUSH_OFFSET + (28561 * 7) + (2197 * 6) + (169 * 3) + (13 * 1) + 0,
            ),
            ("A0 B0 C0 D6 A7 B9 C9", FULL_HOUSE_OFFSET + (13 * 0) + 9),
            ("A0 B1 C6 D6 A7 A6 B6", FOUR_OFFSET + (13 * 6) + 7),
            ("BB B1 C6 C7 C8 C9 CA", STRAIGHT_FLUSH_OFFSET + 10),
            // Special cases
            ("AC C0 A1 D2 A3 CA B1", STRAIGHT_OFFSET + 3), // also called bicycle or wheel
        ];
        test_cases
            .iter()
            .for_each(|tuple| test_combination(tuple.0, tuple.1));
    }

    /// Testing failure of illegal cards.
    #[test]
    fn test_invalid_cards() {
        let test_cases = vec!["A23", "A", "AD", "12", "  "];
        for case in test_cases {
            assert!(
                Card::try_from(case).is_err(),
                "Failed: {}. Expected Error, got {}",
                case,
                Card::try_from(case).unwrap()
            );
        }
    }
}
