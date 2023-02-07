use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day15;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Kind {
    #[rematch(r"A")]
    A,
    #[rematch(r"B")]
    B,
}

impl Kind {
    const fn factor(&self) -> u64 {
        match self {
            Kind::A => 16807,
            Kind::B => 48271,
        }
    }

    const fn criteria(&self) -> u32 {
        match self {
            Kind::A => 4,
            Kind::B => 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"Generator (A|B) starts with (\d+)")]
struct Generator {
    kind: Kind,
    v: u32,
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.v = (((self.v as u64) * self.kind.factor()) % 2147483647) as u32;
        Some(self.v)
    }
}

impl Problem for Day15 {
    type Input = VecFromLines<Generator>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input[0]
            .zip(input[1])
            .take(40_000_000)
            .filter(|(a, b)| *a & 0x0000ffff == *b & 0x0000ffff)
            .count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input[0]
            .filter(|x| x % input[0].kind.criteria() == 0)
            .zip(input[1].filter(|x| x % input[1].kind.criteria() == 0))
            .take(5_000_000)
            .filter(|(a, b)| *a & 0x0000ffff == *b & 0x0000ffff)
            .count()
    }
}

fn main() {
    solve::<Day15>(include_str!("../../inputs/day15.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "Generator A starts with 65\n",
        "Generator B starts with 8921",
    );

    #[test]
    #[ignore = "slow"]
    fn test_sample() {
        assert_eq!(solve_part1::<Day15>(SAMPLE), 588);
        assert_eq!(solve_part2::<Day15>(SAMPLE), 309);
    }
}
