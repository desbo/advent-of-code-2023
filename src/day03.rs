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

            if adjacent_to_symbol(&chars, i, LINE_LENGTH) {
                is_part_number = true;
            }
        }
    }

    sum
}

fn adjacent_to_symbol(
    indexed_chars: &[(usize, char)],
    index_to_check: usize,
    line_length: usize,
) -> bool {
    fn is_symbol(c: char) -> bool {
        c != '.' && !c.is_digit(10) && c != '\n'
    }

    for y in 0i32..3 {
        for x in 0i32..3 {
            let idx = (index_to_check as i32 + x - 1) + ((y - 1) * (line_length + 1) as i32);

            if indexed_chars
                .get(clamp(0, indexed_chars.len() as i32, idx) as usize)
                .map(|&(_, c)| is_symbol(c))
                .unwrap_or(false)
            {
                return true;
            }
        }
    }

    false
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
