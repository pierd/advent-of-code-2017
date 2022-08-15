use aoc_helpers::prelude::*;

struct Day01;

impl Problem for Day01 {
    type Input = RowsOfChars<char>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut sum = 0;
        for digits in input[0].windows(2) {
            if digits[0] == digits[1] {
                sum += (digits[0] as u8 - b'0') as usize;
            }
        }
        if input[0].first().unwrap() == input[0].last().unwrap() {
            sum += (*input[0].first().unwrap() as u8 - b'0') as usize;
        }
        sum
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let digits = &input[0];
        let count = digits.len();

        digits
            .iter()
            .enumerate()
            .filter_map(|(idx, digit)| {
                let other_idx = (idx + count / 2) % count;
                if *digit == digits[other_idx] {
                    Some((*digit as u8 - b'0') as usize)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() {
    solve::<Day01>(include_str!("../../inputs/day01.txt"));
}
