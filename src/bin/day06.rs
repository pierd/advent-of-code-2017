use std::collections::{HashMap, HashSet};

use aoc_helpers::{prelude::*, scaffold::Parse};

struct Day06;

struct SixteenInts;

impl Parse for SixteenInts {
    type Parsed = [usize; 16];

    fn parse(raw_input: &str) -> anyhow::Result<Self::Parsed> {
        let mut parts = raw_input.split_ascii_whitespace().map(|s| {
            s.parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Int parsing failed: {}", e))
        });
        let mut get_next = move || {
            parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Not enough numbers"))
        };
        Ok([
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
            get_next()??,
        ])
    }
}

fn reallocate<const N: usize>(blocks: &[usize; N]) -> [usize; N] {
    let (max_idx, max_val) = blocks
        .iter()
        .enumerate()
        .max_by_key(|(idx, val)| (*val, -(*idx as isize)))
        .expect("there should be at least one element");
    let mut new_blocks = *blocks;
    new_blocks[max_idx] = 0;
    for x in new_blocks.iter_mut() {
        *x += max_val / N;
    }
    for offset in 0..(max_val % N) {
        new_blocks[(max_idx + 1 + offset) % N] += 1;
    }
    new_blocks
}

fn find_reallocation_loop<const N: usize>(blocks: &[usize; N]) -> usize {
    let mut seen = HashSet::new();
    let mut steps = 0;
    let mut blocks = *blocks;
    seen.insert(blocks);
    loop {
        blocks = reallocate(&blocks);
        steps += 1;
        if !seen.insert(blocks) {
            break;
        }
    }
    steps
}

fn measure_reallocation_loop<const N: usize>(blocks: &[usize; N]) -> usize {
    let mut seen = HashMap::new();
    let mut steps = 0;
    let mut blocks = *blocks;
    seen.insert(blocks, 0);
    loop {
        blocks = reallocate(&blocks);
        steps += 1;
        if let Some(old_steps) = seen.insert(blocks, steps) {
            return steps - old_steps;
        }
    }
}

impl Problem for Day06 {
    type Input = SixteenInts;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        find_reallocation_loop(input)
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        measure_reallocation_loop(input)
    }
}

fn main() {
    solve::<Day06>(include_str!("../../inputs/day06.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reallocate() {
        assert_eq!(reallocate(&[0, 2, 7, 0]), [2, 4, 1, 2]);
        assert_eq!(reallocate(&[2, 4, 1, 2]), [3, 1, 2, 3]);
        assert_eq!(reallocate(&[3, 1, 2, 3]), [0, 2, 3, 4]);
        assert_eq!(reallocate(&[0, 2, 3, 4]), [1, 3, 4, 1]);
        assert_eq!(reallocate(&[1, 3, 4, 1]), [2, 4, 1, 2]);
    }

    #[test]
    fn test_sample() {
        assert_eq!(find_reallocation_loop(&[0, 2, 7, 0]), 5);
        assert_eq!(measure_reallocation_loop(&[0, 2, 7, 0]), 4);
    }
}
