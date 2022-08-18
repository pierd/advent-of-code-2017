use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day13;

#[derive(Clone, Copy, Debug)]
#[rematch(r"(\d+): (\d+)")]
struct Layer {
    depth: usize,
    range: usize,
}

impl Problem for Day13 {
    type Input = VecFromLines<Layer>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .iter()
            .map(|&Layer { depth, range }| {
                if depth % (range * 2 - 2) == 0 {
                    depth * range
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        for delay in 1.. {
            if !input
                .iter()
                .any(|&Layer { depth, range }| (delay + depth) % (range * 2 - 2) == 0)
            {
                return delay;
            }
        }
        unreachable!()
    }
}

fn main() {
    solve::<Day13>(include_str!("../../inputs/day13.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!("0: 3\n", "1: 2\n", "4: 4\n", "6: 4",);

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day13>(SAMPLE), 24);
        assert_eq!(solve_part2::<Day13>(SAMPLE), 10);
    }
}
