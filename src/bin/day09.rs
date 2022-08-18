use aoc_helpers::prelude::*;

struct Day09;

impl Problem for Day09 {
    type Input = String;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut depth = 0;
        let mut score = 0;
        let mut ignore_next = false;
        let mut in_garbage = false;
        for c in input.chars() {
            if ignore_next {
                ignore_next = false;
            } else if c == '!' {
                ignore_next = true;
            } else if in_garbage {
                if c == '>' {
                    in_garbage = false;
                }
            } else if c == '<' {
                in_garbage = true;
            } else if c == '}' {
                score += depth;
                if depth > 0 {
                    depth -= 1;
                }
            } else if c == '{' {
                depth += 1;
            }
        }
        score
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut score = 0;
        let mut ignore_next = false;
        let mut in_garbage = false;
        for c in input.chars() {
            if ignore_next {
                ignore_next = false;
            } else if c == '!' {
                ignore_next = true;
            } else if in_garbage {
                if c == '>' {
                    in_garbage = false;
                } else {
                    score += 1
                }
            } else if c == '<' {
                in_garbage = true;
            }
        }
        score
    }
}

fn main() {
    solve::<Day09>(include_str!("../../inputs/day09.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_part1_samples() {
        assert_eq!(solve_part1::<Day09>("{}"), 1);
        assert_eq!(solve_part1::<Day09>("{{{}}}"), 6);
        assert_eq!(solve_part1::<Day09>("{{},{}}"), 5);
        assert_eq!(solve_part1::<Day09>("{{{},{},{{}}}}"), 16);
        assert_eq!(solve_part1::<Day09>("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(solve_part1::<Day09>("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(solve_part1::<Day09>("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(solve_part1::<Day09>("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part2_samples() {
        assert_eq!(solve_part2::<Day09>("<>"), 0);
        assert_eq!(solve_part2::<Day09>("<random characters>"), 17);
        assert_eq!(solve_part2::<Day09>("<<<<>"), 3);
        assert_eq!(solve_part2::<Day09>("<{!>}>"), 2);
        assert_eq!(solve_part2::<Day09>("<!!>"), 0);
        assert_eq!(solve_part2::<Day09>("<!!!>>"), 0);
        assert_eq!(solve_part2::<Day09>("<{o'i!a,<{i<a>"), 10);
    }
}
