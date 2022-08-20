use std::collections::{HashMap, VecDeque};

use aoc_helpers::{interpret::Execute, prelude::*};
use rematch::rematch;

struct Day18;

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
    #[rematch(r"snd (.*)")]
    Snd(Arg),
    #[rematch(r"set (.) (.*)")]
    Set(Arg, Arg),
    #[rematch(r"add (.*) (.*)")]
    Add(Arg, Arg),
    #[rematch(r"mul (.*) (.*)")]
    Mul(Arg, Arg),
    #[rematch(r"mod (.*) (.*)")]
    Mod(Arg, Arg),
    #[rematch(r"rcv (.*)")]
    Rcv(Arg),
    #[rematch(r"jgz (.*) (.*)")]
    Jgz(Arg, Arg),
}

#[derive(Clone, Debug, Default)]
struct State {
    regs: HashMap<char, isize>,
    last_played: Option<isize>,
    last_received: Option<isize>,
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
            Instr::Snd(x) => {
                state.last_played = Some(x.eval(&state));
                Default::default()
            }
            Instr::Set(x, y) => {
                *x.eval_mut(&mut state) = y.eval(&state);
                Default::default()
            }
            Instr::Add(x, y) => {
                *x.eval_mut(&mut state) += y.eval(&state);
                Default::default()
            }
            Instr::Mul(x, y) => {
                *x.eval_mut(&mut state) *= y.eval(&state);
                Default::default()
            }
            Instr::Mod(x, y) => {
                *x.eval_mut(&mut state) %= y.eval(&state);
                Default::default()
            }
            Instr::Rcv(x) => {
                if x.eval(&state) != 0 {
                    state.last_received = state.last_played;
                    interpret::Jump::Stop
                } else {
                    Default::default()
                }
            }
            Instr::Jgz(x, y) => {
                if x.eval(&state) > 0 {
                    interpret::Jump::Relative(y.eval(&state))
                } else {
                    Default::default()
                }
            }
        };
        (state, jump)
    }
}

struct ProgramState {
    local_state: State,
    instr_idx: usize,
    sends: usize,
    send_queue: VecDeque<isize>,
    execution_state: ExecutionState,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ExecutionState {
    #[default]
    NotStarted,
    WaitingForVal,
    Ended,
}

impl ProgramState {
    fn new(id: usize) -> Self {
        let mut local_state = State::default();
        *local_state.get_mut('p') = id as isize;
        Self {
            local_state,
            instr_idx: 0,
            sends: 0,
            send_queue: Default::default(),
            execution_state: Default::default(),
        }
    }

    fn execute(&mut self, instrs: &[Instr], mut received: Option<isize>) {
        while let Some(instr) = instrs.get(self.instr_idx) {
            let jump = match instr {
                Instr::Snd(x) => {
                    self.send_queue.push_back(x.eval(&self.local_state));
                    self.sends += 1;
                    Default::default()
                }
                Instr::Rcv(x) => {
                    if let Some(rcv) = received {
                        *x.eval_mut(&mut self.local_state) = rcv;
                        received = None;
                        Default::default()
                    } else {
                        self.execution_state = ExecutionState::WaitingForVal;
                        return;
                    }
                }
                _ => {
                    let mut temp = Default::default();
                    std::mem::swap(&mut temp, &mut self.local_state);
                    let (new_state, jump) = instr.execute(temp);
                    self.local_state = new_state;
                    jump
                }
            };
            match jump {
                interpret::Jump::Absolute(idx) => self.instr_idx = idx,
                interpret::Jump::Relative(d) => {
                    if let Ok(new_idx) = usize::try_from(self.instr_idx as isize + d) {
                        self.instr_idx = new_idx
                    } else {
                        break;
                    }
                }
                interpret::Jump::Stop => break,
            }
        }
        self.execution_state = ExecutionState::Ended;
    }
}

impl Problem for Day18 {
    type Input = VecFromLines<Instr>;
    type Part1 = isize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .execute(State::default())
            .0
            .last_received
            .expect("something should have been received")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut programs = [ProgramState::new(0), ProgramState::new(1)];
        while programs[0].execution_state != ExecutionState::WaitingForVal
            || programs[1].execution_state != ExecutionState::WaitingForVal
            || !programs[0].send_queue.is_empty()
            || !programs[1].send_queue.is_empty()
        {
            if programs[0].execution_state != ExecutionState::WaitingForVal
                || !programs[1].send_queue.is_empty()
            {
                let received = programs[1].send_queue.pop_front();
                programs[0].execute(input, received);
            } else {
                let received = programs[0].send_queue.pop_front();
                programs[1].execute(input, received);
            }
        }
        programs[1].sends
    }
}

fn main() {
    solve::<Day18>(include_str!("../../inputs/day18.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE1: &str = concat!(
        "set a 1\n",
        "add a 2\n",
        "mul a a\n",
        "mod a 5\n",
        "snd a\n",
        "set a 0\n",
        "rcv a\n",
        "jgz a -1\n",
        "set a 1\n",
        "jgz a -2\n",
    );

    const SAMPLE2: &str =
        concat!("snd 1\n", "snd 2\n", "snd p\n", "rcv a\n", "rcv b\n", "rcv c\n", "rcv d\n",);

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day18>(SAMPLE1), 4);
        assert_eq!(solve_part2::<Day18>(SAMPLE2), 3);
    }
}
