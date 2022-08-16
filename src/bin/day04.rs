use std::collections::HashSet;

use aoc_helpers::prelude::*;

struct Day04;

fn is_valid(p: &str) -> bool {
    let mut set = HashSet::new();
    for part in p.split_ascii_whitespace() {
        if !set.insert(part) {
            return false;
        }
    }
    true
}

fn validation_key(s: &str) -> String {
    let mut chrs = s.chars().collect::<Vec<_>>();
    chrs.sort_unstable();
    chrs.into_iter().collect()
}

fn is_really_valid(p: &str) -> bool {
    let mut set = HashSet::new();
    for part in p.split_ascii_whitespace() {
        if !set.insert(validation_key(part)) {
            return false;
        }
    }
    true
}

impl Problem for Day04 {
    type Input = VecFromLines<String>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().filter(|s| is_valid(*s)).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input.iter().filter(|s| is_really_valid(*s)).count()
    }
}

fn main() {
    solve::<Day04>(include_str!("../../inputs/day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_really_valid() {
        assert!(is_really_valid("abcde fghij"));
        assert!(!is_really_valid("abcde xyz ecdab"));
        assert!(is_really_valid("a ab abc abd abf abj"));
        assert!(is_really_valid("iiii oiii ooii oooi oooo"));
        assert!(!is_really_valid("oiii ioii iioi iiio"));
    }
}
