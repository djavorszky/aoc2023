use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::prelude::*;

pub fn run_day() -> Result<()> {
    let input = include_str!("../input/7.txt");
    println!("Task 1: {}", task1(input)?);
    println!("Task 2: {}", task2(input)?);
    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(Bid::new)
        .sorted()
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid.amount as usize)
        .sum();

    Ok(result)
}

fn task2(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(Bid::new_t2)
        .sorted()
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid.amount as usize)
        .sum();

    Ok(result)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Bid {
    hand: Hand,
    amount: u32,
}

impl Bid {
    fn new(s: &str) -> Self {
        let (first, second) = s.split_once(' ').unwrap();

        Self {
            hand: Hand::new(first),
            amount: second.parse::<u32>().unwrap(),
        }
    }

    fn new_t2(s: &str) -> Self {
        let (first, second) = s.split_once(' ').unwrap();

        Self {
            hand: Hand::new_t2(first),
            amount: second.parse::<u32>().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    hand_type: Type,
    card_values: [u8; 5],
}

fn card_value(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

fn card_value_t2(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

impl Hand {
    fn new(cards: &str) -> Self {
        let mut card_values = [0; 5];

        for (idx, c) in cards.char_indices() {
            card_values[idx] = card_value(c)
        }

        Self {
            hand_type: Type::from_cards(cards),
            card_values,
        }
    }

    fn new_t2(cards: &str) -> Self {
        let mut card_values = [0; 5];

        for (idx, c) in cards.char_indices() {
            card_values[idx] = card_value_t2(c)
        }

        Self {
            hand_type: Type::from_cards_t2(cards),
            card_values,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Type {
    fn from_cards(cards: &str) -> Self {
        let mut map = HashMap::new();

        for c in cards.chars() {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        match *map.values().sorted().as_slice() {
            [1, 1, 1, 2] => Self::Pair,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 3] => Self::ThreeOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 4] => Self::FourOfAKind,
            [5] => Self::FiveOfAKind,
            _ => Self::HighCard,
        }
    }

    fn from_cards_t2(cards: &str) -> Self {
        let to_type = |map: &HashMap<char, i32>| match *map.values().sorted().as_slice() {
            [1, 1, 1, 2] => Self::Pair,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 3] => Self::ThreeOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 4] => Self::FourOfAKind,
            [5] => Self::FiveOfAKind,
            _ => Self::HighCard,
        };

        replace_joker(cards)
            .into_iter()
            .map(|hand| {
                let mut map = HashMap::new();

                for c in hand.chars() {
                    map.entry(c).and_modify(|count| *count += 1).or_insert(1);
                }

                to_type(&map)
            })
            .max()
            .unwrap_or(Type::FiveOfAKind)
    }

    fn value(&self) -> u8 {
        match self {
            Type::FiveOfAKind => 6,
            Type::FourOfAKind => 5,
            Type::FullHouse => 4,
            Type::ThreeOfAKind => 3,
            Type::TwoPair => 2,
            Type::Pair => 1,
            Type::HighCard => 0,
        }
    }
}

fn replace_joker(cards: &str) -> Vec<String> {
    if !cards.contains('J') {
        return vec![cards.to_string()];
    }

    let chars: HashSet<char> = cards.chars().filter(|c| *c != 'J').collect();

    chars
        .iter()
        .map(|c| cards.replace('J', &c.to_string()))
        .collect_vec()
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task1() {
        let res = task1(TEST).unwrap();
        assert_eq!(res, 6440)
    }

    #[test]
    fn test_task2() {
        let res = task2(TEST).unwrap();
        assert_eq!(res, 5905)
    }

    #[test]
    fn test_ordering() {
        assert!(Type::FiveOfAKind > Type::FourOfAKind);
        assert!(Type::Pair < Type::FourOfAKind);

        assert!(Hand::new("55555") > Hand::new("66662"));
        assert!(Hand::new("T222A") > Hand::new("9111B"));
    }

    #[test]
    fn test_parse_hand() {
        let hand = Hand::new("32T3K");

        assert_eq!(hand.hand_type, Type::Pair);
        assert_eq!(hand.card_values, [3, 2, 10, 3, 13]);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(Type::from_cards("32T3K"), Type::Pair);
        assert_eq!(Type::from_cards("T55J5"), Type::ThreeOfAKind);
        assert_eq!(Type::from_cards("KK677"), Type::TwoPair);
        assert_eq!(Type::from_cards("KTJJT"), Type::TwoPair);
        assert_eq!(Type::from_cards("QQQJA"), Type::ThreeOfAKind);
        assert_eq!(Type::from_cards("AAAKK"), Type::FullHouse);
        assert_eq!(Type::from_cards("QQQQ2"), Type::FourOfAKind);
        assert_eq!(Type::from_cards("55555"), Type::FiveOfAKind);
        assert_eq!(Type::from_cards("12345"), Type::HighCard);
    }

    #[test]
    fn test_parse_bid() {
        assert_eq!(
            Bid::new("32T3K 765"),
            Bid {
                hand: Hand::new("32T3K"),
                amount: 765
            }
        );
    }

    #[test]
    fn test_replace_joker() {
        let mut res = replace_joker("KKJTT");
        res.sort();

        assert_eq!(res, vec!["KKKTT", "KKTTT"]);
        let mut res = replace_joker("A9JJ2");
        res.sort();

        assert_eq!(res, vec!["A9222", "A9992", "A9AA2"]);
    }

    const TEST: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
}
