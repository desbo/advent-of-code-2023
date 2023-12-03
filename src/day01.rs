use std::num::ParseIntError;

#[aoc(day1, part1)]
pub fn solve1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)))
        .map(|digits| int_from_first_and_last_digits(digits))
        .sum::<Result<_, _>>()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve2(input: &str) -> i32 {
    fn num_char_at_index(line: &str, i: usize) -> Option<char> {
        fn word_num(substring: &str) -> Option<char> {
            let words: Vec<&str> = vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];

            words.iter().enumerate().find_map(|(i, word)| {
                if substring.starts_with(word) {
                    char::from_digit((i + 1) as u32, 10)
                } else {
                    None
                }
            })
        }

        line.chars()
            .nth(i)
            .filter(|c| c.is_digit(10))
            .or(line.get(i..).and_then(|sub| word_num(sub)))
    }

    input
        .lines()
        .map(|line| {
            line.char_indices()
                .filter_map(|(i, _)| num_char_at_index(line, i))
        })
        .map(|digits| int_from_first_and_last_digits(digits))
        .sum::<Result<_, _>>()
        .unwrap()
}

fn int_from_first_and_last_digits<I>(digits: I) -> Result<i32, ParseIntError>
where
    I: Iterator<Item = char>,
{
    let all_ints = digits.collect::<Vec<_>>();
    let ints = vec![
        all_ints.first().unwrap().to_ascii_lowercase(),
        all_ints.last().unwrap().to_ascii_lowercase(),
    ];

    ints.into_iter().collect::<String>().parse::<i32>()
}
