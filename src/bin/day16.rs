use std::collections::{HashMap, VecDeque};

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day16;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Move {
    #[rematch(r"s(\d+)")]
    Spin(usize),
    #[rematch(r"x(\d+)/(\d+)")]
    Exchange(usize, usize),
    #[rematch(r"p(.)/(.)")]
    Partner(char, char),
}

impl Move {
    fn eval(&self, buffer: &mut VecDeque<char>) {
        match self {
            Move::Spin(n) => {
                for _ in 0..*n {
                    let x = buffer.pop_back().unwrap();
                    buffer.push_front(x)
                }
            }
            Move::Exchange(i, j) => buffer.swap(*i, *j),
            Move::Partner(x, y) => {
                let i = buffer
                    .iter()
                    .enumerate()
                    .find(|(_, c)| **c == *x)
                    .unwrap()
                    .0;
                let j = buffer
                    .iter()
                    .enumerate()
                    .find(|(_, c)| **c == *y)
                    .unwrap()
                    .0;
                buffer.swap(i, j);
            }
        }
    }
}

fn create_entry_position(upto_letter: char) -> VecDeque<char> {
    let mut buffer = VecDeque::with_capacity(16);
    for c in 'a'..=upto_letter {
        buffer.push_back(c);
    }
    buffer
}

fn perform_dance(mut buffer: VecDeque<char>, moves: &[Move]) -> VecDeque<char> {
    for m in moves {
        m.eval(&mut buffer);
    }
    buffer
}

impl Problem for Day16 {
    type Input = VecFromCommaSeparated<Move>;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let buffer = create_entry_position('p');
        perform_dance(buffer, input).into_iter().collect()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut buffer = create_entry_position('p');
        let mut seen: HashMap<String, usize> = Default::default();
        let mut iteration = 0;
        while iteration < 1_000_000_000 {
            let key: String = buffer.iter().copied().collect();
            if let Some(already_seen) = seen.insert(key, iteration) {
                let cycle_length = iteration - already_seen;
                let cycles_left = (1_000_000_000 - iteration) / cycle_length;
                iteration += cycles_left * cycle_length;
            }
            buffer = perform_dance(buffer, input);
            iteration += 1;
        }
        buffer.into_iter().collect()
    }
}

fn main() {
    solve::<Day16>(include_str!("../../inputs/day16.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::parse::Parse;

    const SAMPLE: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_sample() {
        let buffer = create_entry_position('e');
        let moves = <Day16 as Problem>::Input::parse(SAMPLE).unwrap();
        assert_eq!(
            perform_dance(buffer.clone(), &moves)
                .into_iter()
                .collect::<String>(),
            "baedc".to_owned()
        );
        assert_eq!(
            perform_dance(perform_dance(buffer.clone(), &moves), &moves)
                .into_iter()
                .collect::<String>(),
            "ceadb".to_owned()
        );
    }
}
