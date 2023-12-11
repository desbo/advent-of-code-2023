use std::iter;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{digit1, line_ending, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<u32>,
    mappings: Vec<Vec<Mapping>>,
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    source_start: u32,
    dest_start: u32,
    range_length: u32,
}

impl Almanac {
    fn seed_location(&self, seed: u32) -> u32 {
        self.mappings.iter().fold(seed, |id, mappings| {
            mappings
                .iter()
                .find_map(|mapping| mapping.translate(id))
                .unwrap_or(id)
        })
    }
}

impl Mapping {
    fn covers(&self, id: u32) -> bool {
        id >= self.source_start && id <= self.source_start + self.range_length
    }

    fn translate(&self, id: u32) -> Option<u32> {
        if self.covers(id) {
            Some(((self.dest_start as i32 - self.source_start as i32) + id as i32) as u32)
        } else {
            None
        }
    }
}

fn parse_almanac(s: &str) -> IResult<&str, Almanac> {
    let digits = |s| separated_list1(space1, digit1)(s);

    let mapping = preceded(
        terminated(take_till(|c| c == ':'), tag(":\n")),
        separated_list1(line_ending, digits),
    );

    let (s, _) = tag("seeds: ")(s)?;
    let (s, seed_id_strings) = terminated(digits, line_ending)(s)?;

    let (s, mapping_strings) = many1(mapping)(s)?;

    let seed_ids = seed_id_strings.into_iter().flat_map(|id| id.parse());
    let mappings = mapping_strings.into_iter().map(|mappings| {
        mappings
            .into_iter()
            .flat_map(|ids| match ids[..] {
                [dst, src, len] => Ok(Mapping {
                    source_start: u32::from_str(src).unwrap(),
                    dest_start: u32::from_str(dst).unwrap(),
                    range_length: u32::from_str(len).unwrap(),
                }),
                _ => Err("BooM"),
            })
            .collect::<Vec<_>>()
    });

    Ok((
        s,
        Almanac {
            seeds: seed_ids.collect(),
            mappings: mappings.collect(),
        },
    ))
}

#[aoc(day5, part1)]
pub fn solve1(input: &str) -> u32 {
    let (_, almanac) = parse_almanac(input).unwrap();

    almanac
        .seeds
        .iter()
        .map(|&s| almanac.seed_location(s))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve2(input: &str) -> u32 {
    let (_, almanac) = parse_almanac(input).unwrap();

    let all_seeds = almanac.seeds.chunks(2).flat_map(|c| match *c {
        [s, size] => {
            let mut curr = s;

            iter::from_fn(move || {
                let tmp = curr;
                curr += 1;

                if curr < s + size {
                    Some(tmp)
                } else {
                    None
                }
            })
        }
        _ => panic!("a"),
    });

    all_seeds.map(|s| almanac.seed_location(s)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_example() {
        let (_, alm) = parse_almanac(SAMPLE).unwrap();
        assert_eq!(alm.seed_location(79), 82);
        assert_eq!(alm.seed_location(14), 43);
    }
}
