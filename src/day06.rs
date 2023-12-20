use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;

pub struct RaceLog {
    races: Vec<Race>,
}

pub struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    fn num_record_breaks(&self) -> u64 {
        let mut count = 0;

        for hold_time in 0..self.time + 1 {
            if hold_time * (self.time - hold_time) > self.distance_record {
                count += 1
            }
        }

        count
    }
}

fn parse_race_log_1(s: &str) -> IResult<&str, RaceLog> {
    let nums = |s| separated_list1(space1, digit1)(s);

    let (s, _) = pair(tag("Time:"), space1)(s)?;
    let (s, times) = nums(s)?;
    let (s, _) = pair(tag("\nDistance:"), space1)(s)?;
    let (s, dists) = nums(s)?;

    let races = times.into_iter().zip(dists).map(|(a, b)| Race {
        time: a.parse().unwrap(),
        distance_record: b.parse().unwrap(),
    });

    Ok((
        s,
        RaceLog {
            races: races.collect(),
        },
    ))
}

fn parse_race_log_2(s: &str) -> IResult<&str, RaceLog> {
    let nums = |s| map(separated_list1(space1, digit1), |strs| strs.join(""))(s);

    let (s, _) = pair(tag("Time:"), space1)(s)?;
    let (s, times) = nums(s)?;
    let (s, _) = pair(tag("\nDistance:"), space1)(s)?;
    let (s, dists) = nums(s)?;

    Ok((
        s,
        RaceLog {
            races: vec![Race {
                time: times.parse().unwrap(),
                distance_record: dists.parse().unwrap(),
            }],
        },
    ))
}

#[aoc(day6, part1)]
fn solve1(input: &str) -> u64 {
    let (_, log) = parse_race_log_1(input).unwrap();

    log.races
        .iter()
        .fold(1, |wins, race| wins * race.num_record_breaks())
}

#[aoc(day6, part2)]
fn solve2(input: &str) -> u64 {
    let (_, log) = parse_race_log_2(input).unwrap();

    log.races
        .iter()
        .fold(1, |wins, race| wins * race.num_record_breaks())
}
