use aoc_helpers::{prelude::*, scaffold::Parse};

struct Day10;

fn reverse_circular<T>(slice: &mut [T], mut first: usize, mut last: usize) {
    while first != last {
        slice.swap(first, last);
        first = (first + 1) % slice.len();
        if first == last {
            break;
        }
        last = if last == 0 { slice.len() - 1 } else { last - 1 };
    }
}

fn knot_hash_list<const LIST_LENGTH: usize, const ROUNDS: usize>(lengths: &[usize]) -> Vec<usize> {
    let mut lst: Vec<usize> = (0..LIST_LENGTH).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..ROUNDS {
        for length in lengths {
            if *length > 1 {
                reverse_circular(
                    &mut lst,
                    current_position,
                    (current_position + *length - 1) % LIST_LENGTH,
                );
            }
            current_position = (current_position + *length + skip_size) % LIST_LENGTH;
            skip_size += 1;
        }
    }
    lst
}

fn dense_to_sparse(dense: &[usize]) -> Vec<usize> {
    assert!(
        dense.len() % 16 == 0,
        "Dense hash lenght must be divisible by 16"
    );
    dense
        .chunks(16)
        .map(|chunk| {
            chunk
                .iter()
                .copied()
                .reduce(|acc, item| acc ^ item)
                .unwrap()
        })
        .collect()
}

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
    use std::vec;

    use super::*;

    #[test]
    fn test_reverse_circular() {
        let mut arr = [0, 1, 2, 3];
        reverse_circular(&mut arr, 0, 3);
        assert_eq!(arr, [3, 2, 1, 0]);
        reverse_circular(&mut arr, 0, 2);
        assert_eq!(arr, [1, 2, 3, 0]);
        reverse_circular(&mut arr, 2, 0);
        assert_eq!(arr, [3, 2, 1, 0]);
        reverse_circular(&mut arr, 2, 2);
        assert_eq!(arr, [3, 2, 1, 0]);
    }

    #[test]
    fn test_knot_hash_list() {
        assert_eq!(knot_hash_list::<5, 1>(&[3, 4, 1, 5]), vec![3, 4, 2, 1, 0]);
    }

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
