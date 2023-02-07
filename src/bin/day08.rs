use std::collections::HashMap;

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day08;

#[derive(Copy, Clone, Debug)]
#[rematch]
enum Command {
    #[rematch(r"inc")]
    Inc,
    #[rematch(r"dec")]
    Dec,
}

impl Command {
    fn eval(&self, a: &mut isize, b: isize) {
        match self {
            Command::Inc => *a += b,
            Command::Dec => *a -= b,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[rematch]
enum Operator {
    #[rematch(r"==")]
    Equal,
    #[rematch(r"!=")]
    NotEqual,
    #[rematch(r">=")]
    GreaterOrEqual,
    #[rematch(r"<=")]
    LessOrEqual,
    #[rematch(r">")]
    GreaterThan,
    #[rematch(r"<")]
    LessThan,
}

impl Operator {
    fn eval(&self, a: isize, b: isize) -> bool {
        match self {
            Operator::Equal => a == b,
            Operator::NotEqual => a != b,
            Operator::GreaterThan => a > b,
            Operator::LessThan => a < b,
            Operator::GreaterOrEqual => a >= b,
            Operator::LessOrEqual => a <= b,
        }
    }
}

#[derive(Clone, Debug)]
#[rematch(r"(\w+) (inc|dec) (-?\d+) if (\w+) ([^ ]+) (-?\d+)")]
struct Instr {
    reg: String,
    cmd: Command,
    offset: isize,
    cond_reg: String,
    cond_op: Operator,
    cond_val: isize,
}

impl Instr {
    fn eval(&self, regs: &mut HashMap<String, isize>) {
        if self.cond_op.eval(
            regs.get(&self.cond_reg).copied().unwrap_or_default(),
            self.cond_val,
        ) {
            self.cmd
                .eval(regs.entry(self.reg.clone()).or_default(), self.offset);
        }
    }
}

impl Problem for Day08 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = isize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut regs = Default::default();
        for instr in input {
            instr.eval(&mut regs);
        }
        regs.into_values().max().unwrap_or_default()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut highest_ever = 0;
        let mut regs = Default::default();
        for instr in input {
            instr.eval(&mut regs);
            let current_highest = regs.values().copied().max().unwrap_or_default();
            if highest_ever < current_highest {
                highest_ever = current_highest;
            }
        }
        highest_ever
    }
}

fn main() {
    solve::<Day08>(include_str!("../../inputs/day08.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "b inc 5 if a > 1\n",
        "a inc 1 if b < 5\n",
        "c dec -10 if a >= 1\n",
        "c inc -20 if c == 10",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day08>(SAMPLE), 1);
        assert_eq!(solve_part2::<Day08>(SAMPLE), 10);
    }
}
