use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::{Finish, IResult};

#[derive(Debug, PartialEq)]
pub struct Card {
    id: u8,
    winning: HashSet<u8>,
    chosen: HashSet<u8>,
}

impl Card {
    fn chosen_winning_numbers(&self) -> Vec<&u8> {
        self.winning.intersection(&self.chosen).collect::<Vec<_>>()
    }
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, _) = tag("Card")(s)?;
    let (s, _) = space1(s)?;
    let (s, card_id) = map_res(digit1, u8::from_str)(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space1(s)?;
    let (s, winning) = terminated(separated_list0(space1, digit1), tag(" |"))(s)?;
    let (s, _) = space1(s)?;
    let (s, chosen) = separated_list0(space1, digit1)(s)?;

    let card = Card {
        id: card_id,
        winning: HashSet::from_iter(winning.into_iter().map(|s| u8::from_str(s).unwrap())),
        chosen: HashSet::from_iter(chosen.into_iter().map(|s| u8::from_str(s).unwrap())),
    };

    Ok((s, card))
}

impl FromStr for Card {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card(s).finish() {
            Ok((_, card)) => Ok(card),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    input.lines().map(|s| Card::from_str(s).unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn solve1(input: &[Card]) -> i32 {
    input.iter().fold(0i32, |sum, card| {
        sum + 2i32.pow(card.chosen_winning_numbers().len() as u32 - 1)
    })
}

#[aoc(day4, part2)]
pub fn solve2(input: &[Card]) -> i32 {
    let mut processed: Vec<&Card> = Vec::new();
    let mut to_process: VecDeque<&Card> = VecDeque::from_iter(input.iter());

    while !to_process.is_empty() {
        let card = to_process.pop_front().unwrap();
        let num_wins = card.chosen_winning_numbers().len();

        let start = card.id as usize;
        let end = min(card.id as usize + num_wins, input.len());

        input[start..end]
            .iter()
            .for_each(|card| to_process.push_back(card));

        processed.push(card);
    }

    processed.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert_eq!(
            Card::from_str("Card   1: 79  1  6  9 88 95 84 69 83 97 | 42 95  1  6 71 69 61 99 84 12 32 96  9 82 88 97 53 24 28 65 83 38  8 68 79"),
            Ok(Card {
                id: 1,
                winning: HashSet::from_iter(vec![79, 1  ,6,  9, 88, 95, 84, 69, 83, 97].into_iter()),
                chosen: HashSet::from_iter(vec![42,95,1,6,71,69,61,99,84,12,32,96,9,82,88,97,53,24,28,65,83,38,8,68,79].into_iter()),
            })
        )
    }
}
