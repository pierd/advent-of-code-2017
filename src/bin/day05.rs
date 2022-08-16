use aoc_helpers::prelude::*;

struct Day05;

fn execute(instrs: &mut [isize]) -> usize {
    let mut idx = 0isize;
    let mut steps = 0;
    while let Some(offset) = usize::try_from(idx)
        .ok()
        .and_then(|idx| instrs.get(idx))
        .copied()
    {
        steps += 1;
        instrs[usize::try_from(idx).expect("already handled in the condition above")] += 1;
        idx += offset;
    }
    steps
}

fn execute2(instrs: &mut [isize]) -> usize {
    let mut idx = 0isize;
    let mut steps = 0;
    while let Some(offset) = usize::try_from(idx)
        .ok()
        .and_then(|idx| instrs.get(idx))
        .copied()
    {
        steps += 1;
        instrs[usize::try_from(idx).expect("already handled in the condition above")] +=
            if offset >= 3 { -1 } else { 1 };
        idx += offset;
    }
    steps
}

impl Problem for Day05 {
    type Input = VecFromLines<TrimAndParse<isize>>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut instrs = input.to_vec();
        execute(&mut instrs)
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut instrs = input.to_vec();
        execute2(&mut instrs)
    }
}

fn main() {
    solve::<Day05>(include_str!("../../inputs/day05.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(execute(&mut [0, 3, 0, 1, -3]), 5);
        assert_eq!(execute2(&mut [0, 3, 0, 1, -3]), 10);
    }
}
