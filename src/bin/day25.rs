use std::collections::{HashMap, HashSet};

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day25;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Direction {
    #[rematch(r"left")]
    Left,
    #[rematch(r"right")]
    Right,
}

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Config {
    #[rematch(r"Begin in state (.).\nPerform a diagnostic checksum after (\d+) steps.")]
    Starting { starting_state: char, steps: usize },
    #[rematch(r"In state (.):\n\s*If the current value is 0:\n\s*- Write the value (\d).\n\s*- Move one slot to the (right|left).\n\s*- Continue with state (.).\n\s*If the current value is 1:\n\s*- Write the value (\d).\n\s*- Move one slot to the (right|left).\n\s*- Continue with state (.).")]
    State {
        name: char,
        value_for_0: usize,
        direction_for_0: Direction,
        state_for_0: char,
        value_for_1: usize,
        direction_for_1: Direction,
        state_for_1: char,
    },
}

#[derive(Clone, Debug, Default)]
struct Tape {
    tape: HashSet<isize>,
    idx: isize,
}

impl Tape {
    fn ones(&self) -> usize {
        self.tape.len()
    }

    fn read(&self) -> bool {
        self.tape.contains(&self.idx)
    }

    fn write(&mut self, v: bool) {
        if v {
            self.tape.insert(self.idx);
        } else {
            self.tape.remove(&self.idx);
        }
    }

    fn advance(&mut self, d: Direction) {
        self.idx += match d {
            Direction::Left => -1,
            Direction::Right => 1,
        };
    }
}

struct Machine {
    target_step: usize,
    state: char,
    transitions: HashMap<char, [(bool, Direction, char); 2]>,
    tape: Tape,
}

impl From<&[Config]> for Machine {
    fn from(configs: &[Config]) -> Self {
        let mut state = 'A';
        let mut target_step = 0;
        let mut transitions: HashMap<char, [(bool, Direction, char); 2]> = Default::default();
        for config in configs {
            match config {
                Config::Starting {
                    starting_state,
                    steps,
                } => {
                    state = *starting_state;
                    target_step = *steps;
                }
                Config::State {
                    name,
                    value_for_0,
                    direction_for_0,
                    state_for_0,
                    value_for_1,
                    direction_for_1,
                    state_for_1,
                } => {
                    transitions.insert(
                        *name,
                        [
                            (*value_for_0 == 1, *direction_for_0, *state_for_0),
                            (*value_for_1 == 1, *direction_for_1, *state_for_1),
                        ],
                    );
                }
            }
        }
        Self {
            target_step,
            state,
            transitions,
            tape: Default::default(),
        }
    }
}

impl Machine {
    fn perform_step(&mut self) {
        let transition_idx = if self.tape.read() { 1 } else { 0 };
        let (to_write, direction, new_state) =
            self.transitions.get(&self.state).unwrap()[transition_idx];
        self.tape.write(to_write);
        self.tape.advance(direction);
        self.state = new_state;
    }

    fn run(&mut self) {
        for _ in 0..self.target_step {
            self.perform_step();
        }
    }
}

impl Problem for Day25 {
    type Input = VecFromMultiLines<Config>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut machine = Machine::from(input.as_slice());
        machine.run();
        machine.tape.ones()
    }

    fn solve_part2(_: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        Default::default()
    }
}

fn main() {
    solve::<Day25>(include_str!("../../inputs/day25.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    const SAMPLE: &str = include_str!("../../inputs/day25-sample.txt");

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day25>(SAMPLE), 3);
    }
}
