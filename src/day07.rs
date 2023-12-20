use std::cmp::{min, Ordering};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};

use nom::bytes::complete::take;
use nom::character::complete::{digit1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

#[derive(Hash, Eq, Debug, PartialEq)]
struct Cards(String);

impl Cards {
    fn value_ord(&self, other: &Self) -> Ordering {
        let strength: HashMap<char, usize, RandomState> = HashMap::from_iter(
            [
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ]
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (c, i + 1)),
        );

        self.0
            .chars()
            .zip(other.0.chars())
            .find_map(|(a, b)| {
                if a != b {
                    Some(strength.get(&a).unwrap().cmp(strength.get(&b).unwrap()))
                } else {
                    None
                }
            })
            .unwrap_or(Ordering::Equal)
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_type = self.r#type().cmp(&other.r#type());

        if by_type == Ordering::Equal {
            Cards::value_ord(self, other)
        } else {
            by_type
        }
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Ranking(Vec<(u32, Hand)>);

impl Ranking {
    fn from(hands: &[Hand]) -> Ranking {
        let bids = Cards::bids_by_hand(hands);
        let mut cards: Vec<Cards> = Vec::new();

        hands.iter().for_each(|h| cards.push(h.cards.clone()));

        cards.sort();

        Ranking(
            cards
                .into_iter()
                .enumerate()
                .map(|(i, cards)| {
                    (
                        (i + 1) as u32,
                        Hand {
                            cards: cards.clone(),
                            bid: *bids.get(&cards).unwrap(),
                        },
                    )
                })
                .collect(),
        )
    }

    fn winnings(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |sum, (rank, hand)| sum + (rank * hand.bid))
    }
}

impl Clone for Cards {
    fn clone(&self) -> Self {
        Cards(self.0.clone())
    }
}

#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub struct Hand {
    cards: Cards,
    bid: u32,
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Hand {
        Hand {
            cards: Cards::new(cards),
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        Cards::cmp(&self.cards, &other.cards)
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    separated_pair(take(5usize), space1, digit1)
        .parse(input)
        .map(|(s, (cards, bid))| {
            (
                s,
                Hand {
                    cards: Cards(cards.to_string()),
                    bid: bid.parse().unwrap(),
                },
            )
        })
}

impl Cards {
    fn new(s: &str) -> Self {
        Cards(s.to_string())
    }

    fn bids_by_hand(hands: &[Hand]) -> HashMap<Cards, u32> {
        let mut m = HashMap::new();
        for hand in hands {
            m.insert(hand.cards.clone(), hand.bid);
        }
        m
    }

    fn r#type(&self) -> u8 {
        let mut counts = HashMap::new();

        self.0
            .chars()
            .for_each(|c| *counts.entry(c).or_insert(0) += 1);

        let (&max_dupe_card, max_dupes) = counts
            .iter()
            .filter(|&(&c, _)| c != 'J')
            .max_by_key(|&(_, count)| count)
            .unwrap_or((&'X', &0));

        let &jokers = counts.get(&'J').unwrap_or(&0);

        match min(max_dupes + jokers, 5) {
            //Five of a kind, where all five cards have the same label: AAAAA
            5 => 7,

            // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
            4 => 6,

            // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
            3 if counts
                .iter()
                .any(|(&card, &count)| card != max_dupe_card && card != 'J' && count == 2) =>
            {
                5
            }

            // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            3 => 4,

            // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
            2 if counts
                .iter()
                .filter(|&(_, &c)| c == 2)
                .collect::<Vec<_>>()
                .len()
                == 2 =>
            {
                3
            }

            // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            2 => 2,

            // High card, where all cards' labels are distinct: 23456
            _ => 1,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (s, hands) = separated_list1(newline, parse_hand)(input)?;
    Ok((s, hands))
}

#[aoc(day7, part1)]
pub fn solve1(input: &str) -> u32 {
    let (_, h) = parse_input(input).unwrap();
    let ranking = Ranking::from(&h);

    ranking.winnings()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        // 1. Five of a kind, where all five cards have the same label: AAAAA
        assert_eq!(Cards::new("AAAAA").r#type(), 7);
        assert_eq!(Cards::new("JJJJJ").r#type(), 7);

        // 2. Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        assert_eq!(Cards::new("AA8AA").r#type(), 6);

        // 3. Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        assert_eq!(Cards::new("23332").r#type(), 5);

        // 4. Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        assert_eq!(Cards::new("TTT98").r#type(), 4);

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        assert_eq!(Cards::new("23432").r#type(), 3);

        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        assert_eq!(Cards::new("A23A4").r#type(), 2);

        // High card, where all cards' labels are distinct: 23456
        assert_eq!(Cards::new("23456").r#type(), 1);
    }

    #[test]
    fn test_sort() {
        let mut cc = vec![
            Cards::new("32T3K"),
            Cards::new("T55J5"),
            Cards::new("KK677"),
            Cards::new("KTJJT"),
            Cards::new("QQQJA"),
        ];

        cc.sort();

        assert_eq!(
            cc,
            vec![
                Cards::new("32T3K"),
                Cards::new("KK677"),
                Cards::new("T55J5"),
                Cards::new("QQQJA"),
                Cards::new("KTJJT"),
            ]
        )
    }

    #[test]
    fn test_sort_joker_full() {
        let mut cc = vec![
            Cards::new("AJJJJ"),
            Cards::new("JJJJJ"),
            Cards::new("AAJJJ"),
            Cards::new("AAAAJ"),
            Cards::new("AAAJJ"),
            Cards::new("AAAAA"),
        ];

        cc.sort();

        assert_eq!(
            cc,
            vec![
                Cards::new("JJJJJ"),
                Cards::new("AJJJJ"),
                Cards::new("AAJJJ"),
                Cards::new("AAAJJ"),
                Cards::new("AAAAJ"),
                Cards::new("AAAAA"),
            ]
        )
    }

    #[test]
    fn test_sort_joker_3() {
        let mut cc = vec![
            Cards::new("222JJ"), // 5  of a kind
            Cards::new("22233"), // full house
            Cards::new("222J3"), // 4 of a kind
        ];

        cc.sort();

        assert_eq!(
            cc,
            vec![
                Cards::new("22233"),
                Cards::new("222J3"),
                Cards::new("222JJ")
            ]
        )
    }
    #[test]
    fn test_full_house() {
        assert_eq!(Cards::new("2233J").r#type(), 5);
        assert_eq!(Cards::new("JJJ34").r#type(), 6);
        assert_eq!(Cards::new("J2345").r#type(), 2);
    }

    #[test]
    fn test_reddit() {
        let hands = vec![
            Hand::new("2345A", 1),
            Hand::new("Q2KJJ", 13),
            Hand::new("Q2Q2Q", 19),
            Hand::new("T3T3J", 17),
            Hand::new("T3Q33", 11),
            Hand::new("2345J", 3),
            Hand::new("J345A", 2),
            Hand::new("32T3K", 5),
            Hand::new("T55J5", 29),
            Hand::new("KK677", 7),
            Hand::new("KTJJT", 34),
            Hand::new("QQQJA", 31),
            Hand::new("JJJJJ", 37),
            Hand::new("JAAAA", 43),
            Hand::new("AAAAJ", 59),
            Hand::new("AAAAA", 61),
            Hand::new("2AAAA", 23),
            Hand::new("2JJJJ", 53),
            Hand::new("JJJJ2", 41),
        ];

        let r = Ranking::from(&hands);

        assert_eq!(r.winnings(), 6839)
    }

    #[test]
    fn test_winnings() {
        let cc = vec![
            Hand {
                cards: Cards::new("32T3K"),
                bid: 765,
            },
            Hand {
                cards: Cards::new("T55J5"),
                bid: 684,
            },
            Hand {
                cards: Cards::new("KK677"),
                bid: 28,
            },
            Hand {
                cards: Cards::new("KTJJT"),
                bid: 220,
            },
            Hand {
                cards: Cards::new("QQQJA"),
                bid: 483,
            },
        ];

        assert_eq!(Ranking::from(&cc).winnings(), 5905)
    }
}
