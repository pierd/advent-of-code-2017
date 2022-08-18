use advent_of_code_2017::knot_hash::{dense_to_sparse, knot_hash_list};
use aoc_helpers::{prelude::*, scaffold::Parse};

struct Day10;

fn vec_to_hex(v: Vec<usize>) -> String {
    v.into_iter().map(|n| format!("{:02x}", n)).collect()
}

impl Problem for Day10 {
    type Input = String;
    type Part1 = usize;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let lengths = VecFromCommaSeparated::<usize>::parse(input)
            .expect("input should parse as a list of ints");
        let lst = knot_hash_list::<256, 1>(&lengths);
        lst[0] * lst[1]
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut lengths: Vec<usize> = input.chars().map(|c| c as u8 as usize).collect();
        lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
        let lst = knot_hash_list::<256, 64>(&lengths);
        vec_to_hex(dense_to_sparse(&lst))
    }
}

fn main() {
    solve::<Day10>(include_str!("../../inputs/day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_part1() {
        assert_eq!(
            solve_part1::<Day10>(include_str!("../../inputs/day10.txt")),
            1980
        );
    }

    #[test]
    fn test_vec_to_hex() {
        assert_eq!(vec_to_hex(vec![64, 7, 255]), "4007ff".to_owned());
    }
}
