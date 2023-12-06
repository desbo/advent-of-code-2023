use std::collections::HashSet;

const LINE_LENGTH: usize = 140;

#[aoc(day3, part1)]
pub fn solve1(input: &str) -> i32 {
    let mut sum = 0;
    let mut current_number: String = String::new();
    let mut is_part_number: bool = false;

    let chars = input.chars().enumerate().collect::<Vec<_>>();

    for &(i, c) in &chars {
        if !c.is_digit(10) {
            if is_part_number {
                sum += current_number.parse::<i32>().unwrap()
            }

            current_number = String::new();
            is_part_number = false;
            continue;
        } else {
            current_number.push(c);

            if neighbours(&chars, i, LINE_LENGTH)
                .iter()
                .any(|&(_, c)| is_symbol(c))
            {
                is_part_number = true;
            }
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn solve2(input: &str) -> i32 {
    let mut sum = 0;

    let chars = input.chars().enumerate().collect::<Vec<_>>();

    for &(i, c) in &chars {
        if c == '*' {
            let ns = neighbours(&chars, i, LINE_LENGTH);

            let numeric_neighbour_cells = ns
                .iter()
                .filter(|(_, c)| c.is_digit(10))
                .collect::<Vec<_>>();

            let mut nums = HashSet::new();

            for &(num_idx, _) in numeric_neighbour_cells {
                let num = read_number_at(&chars, num_idx).unwrap();
                nums.insert(num);
            }

            if nums.len() == 2 {
                sum += nums.iter().product::<i32>();
            }
        }
    }

    sum
}

fn read_number_at(indexed_chars: &[(usize, char)], num_idx: usize) -> Option<i32> {
    let mut num_str = String::new();
    let mut i = num_idx;

    loop {
        if i == 0 || !indexed_chars[i - 1].1.is_digit(10) {
            break;
        }
        i -= 1;
    }

    loop {
        if !indexed_chars[i].1.is_digit(10) {
            break;
        }
        num_str.push(indexed_chars[i].1);
        i += 1
    }

    num_str.parse().ok()
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10) && c != '\n'
}

fn neighbours(
    indexed_chars: &[(usize, char)],
    index_to_check: usize,
    line_length: usize,
) -> Vec<(usize, char)> {
    let mut result = vec![];

    for y in 0i32..3 {
        for x in 0i32..3 {
            let idx = (index_to_check as i32 + x - 1) + ((y - 1) * (line_length + 1) as i32);

            indexed_chars
                .get(clamp(0, indexed_chars.len() as i32, idx) as usize)
                .into_iter()
                .for_each(|&(i, c)| result.push((i, c)));
        }
    }

    result
}

fn clamp<T: PartialOrd>(min: T, max: T, x: T) -> T {
    return if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    };
}
