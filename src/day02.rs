use std::collections::HashMap;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::{Finish, IResult};

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u8,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
pub struct Draw {
    red: u8,
    blue: u8,
    green: u8,
}

// e.g. "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, _) = tag("Game ")(s)?;
    let (s, game_id) = map_res(digit1, u8::from_str)(s)?;
    let (s, _) = tag(": ")(s)?;
    let (s, draws) = separated_list0(tag("; "), take_till(|c| c == ';'))(s)?;

    let parsed_draws = draws.into_iter().flat_map(Draw::from_str);

    let game = Game {
        id: game_id,
        draws: parsed_draws.collect(),
    };

    Ok((s, game))
}

// e.g. "3 green, 4 blue, 1 red"
fn parse_draw(s: &str) -> IResult<&str, Draw> {
    let (s, draws) = separated_list1(
        tag(", "),
        separated_pair(map_res(digit1, u8::from_str), space1, alpha1),
    )(s)?;

    let mut counts = HashMap::with_capacity(3);

    draws
        .iter()
        .for_each(|&(count, colour)| *counts.entry(colour).or_insert(0) += count);

    Ok((
        s,
        Draw {
            red: *counts.get("red").unwrap_or(&0u8),
            green: *counts.get("green").unwrap_or(&0u8),
            blue: *counts.get("blue").unwrap_or(&0u8),
        },
    ))
}

impl Game {
    fn valid(&self, limits: &Draw) -> bool {
        self.draws.iter().all(|draw| {
            draw.red <= limits.red && draw.green <= limits.green && draw.blue <= limits.blue
        })
    }
}

impl FromStr for Game {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_game(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Draw {
    fn new(red: u8, green: u8, blue: u8) -> Draw {
        Draw { red, green, blue }
    }
}

impl FromStr for Draw {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_draw(s).finish() {
            Ok((_, draw)) => Ok(draw),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|s| Game::from_str(s).unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn solve1(input: &[Game]) -> i32 {
    let limits = &Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    input.iter().fold(0, |id_sum, game| {
        if game.valid(limits) {
            id_sum + game.id as i32
        } else {
            id_sum
        }
    })
}

#[aoc(day2, part2)]
pub fn solve2(input: &[Game]) -> i32 {
    input.iter().fold(0, |power_sum, game| {
        let mut max_draw = Draw::new(0, 0, 0);

        game.draws.iter().for_each(|draw| {
            if draw.red > max_draw.red {
                max_draw.red = draw.red
            }
            if draw.green > max_draw.green {
                max_draw.green = draw.green
            }
            if draw.blue > max_draw.blue {
                max_draw.blue = draw.blue
            }
        });

        power_sum + (max_draw.red as i32 * max_draw.green as i32 * max_draw.blue as i32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",),
            Ok(Game {
                id: 1,
                draws: vec![Draw::new(4, 0, 3), Draw::new(1, 2, 6), Draw::new(0, 2, 0)]
            })
        )
    }

    #[test]
    fn test_power_sum() {
        let games = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let parsed_games = games
            .into_iter()
            .flat_map(Game::from_str)
            .collect::<Vec<Game>>();

        assert_eq!(solve2(&parsed_games), 2286);
    }
}
