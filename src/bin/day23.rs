use std::collections::HashMap;

use aoc_helpers::{interpret::Execute, prelude::*};
use rematch::rematch;

struct Day23;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[rematch]
enum Arg {
    #[rematch(r"([a-z])")]
    Reg(char),
    #[rematch(r"(-?\d+)")]
    Const(isize),
}

impl Arg {
    fn eval(&self, state: &State) -> isize {
        match self {
            Arg::Reg(r) => state.get(*r),
            Arg::Const(v) => *v,
        }
    }

    fn eval_mut<'a>(&self, state: &'a mut State) -> &'a mut isize {
        match self {
            Arg::Reg(r) => state.get_mut(*r),
            Arg::Const(_) => panic!("can only be performed on a register"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[rematch]
enum Instr {
    #[rematch(r"set (.) (.*)")]
    Set(Arg, Arg),
    #[rematch(r"sub (.*) (.*)")]
    Sub(Arg, Arg),
    #[rematch(r"mul (.*) (.*)")]
    Mul(Arg, Arg),
    #[rematch(r"jnz (.*) (.*)")]
    Jnz(Arg, Arg),
}

#[derive(Clone, Debug, Default)]
struct State {
    regs: HashMap<char, isize>,
    mul_calls: usize,
}

impl State {
    fn get(&self, reg: char) -> isize {
        self.regs.get(&reg).copied().unwrap_or_default()
    }

    fn get_mut(&mut self, reg: char) -> &mut isize {
        self.regs.entry(reg).or_default()
    }
}

impl interpret::Execute<State> for Instr {
    fn execute(&self, mut state: State) -> (State, interpret::Jump) {
        let jump = match self {
            Instr::Set(x, y) => {
                *x.eval_mut(&mut state) = y.eval(&state);
                Default::default()
            }
            Instr::Sub(x, y) => {
                *x.eval_mut(&mut state) -= y.eval(&state);
                Default::default()
            }
            Instr::Mul(x, y) => {
                state.mul_calls += 1;
                *x.eval_mut(&mut state) *= y.eval(&state);
                Default::default()
            }
            Instr::Jnz(x, y) => {
                if x.eval(&state) != 0 {
                    interpret::Jump::Relative(y.eval(&state))
                } else {
                    Default::default()
                }
            }
        };
        (state, jump)
    }
}

fn is_prime(n: &usize) -> bool {
    if *n < 4 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }
    for i in (3..(*n / 2)).step_by(2) {
        if *n % i == 0 {
            return false;
        }
    }
    true
}

impl Problem for Day23 {
    type Input = VecFromLines<Instr>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.execute(State::default()).0.mul_calls
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut state = State::default();
        *state.get_mut('a') = 1;

        // execute the beginning to get the inputs
        state = (&input[0..=7]).execute(state).0;

        let b = state.get('b').unsigned_abs();
        let c = state.get('c').unsigned_abs();

        (b..=c).step_by(17).filter(|n| !is_prime(n)).count()
    }
}

fn main() {
    solve::<Day23>(include_str!("../../inputs/day23.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day23>(SAMPLE), Default::default());
        assert_eq!(solve_part2::<Day23>(SAMPLE), Default::default());
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(&2));
        assert!(is_prime(&3));
        assert!(!is_prime(&4));
        assert!(is_prime(&5));
        assert!(!is_prime(&6));
        assert!(is_prime(&7));
        assert!(!is_prime(&8));
        assert!(!is_prime(&9));
        assert!(!is_prime(&10));

        assert!(!is_prime(&108105));
    }
}
