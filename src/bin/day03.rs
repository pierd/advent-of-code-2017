use std::collections::HashMap;

use aoc_helpers::prelude::*;

struct Day03;

#[derive(Clone, Copy, Debug, Default)]
enum Direction {
    Right,
    Up,
    Left,
    #[default]
    Down,
}

#[derive(Clone, Copy, Debug)]
struct SpiralIterator {
    x: isize,
    y: isize,
    step: usize,
    round: usize,
    direction: Direction,
}

impl Default for SpiralIterator {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            step: 1,
            round: Default::default(),
            direction: Default::default(),
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (usize, (isize, isize));

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.step, (self.x, self.y));
        let round = self.round as isize;
        if let Some(new_direction) = match self.direction {
            Direction::Right => {
                if self.x == round {
                    self.y += 1;
                    Some(Direction::Up)
                } else {
                    self.x += 1;
                    None
                }
            }
            Direction::Up => {
                if self.y == round {
                    self.x -= 1;
                    Some(Direction::Left)
                } else {
                    self.y += 1;
                    None
                }
            }
            Direction::Left => {
                if self.x == -round {
                    self.y -= 1;
                    Some(Direction::Down)
                } else {
                    self.x -= 1;
                    None
                }
            }
            Direction::Down => {
                if self.y == -round {
                    self.x += 1;
                    self.round += 1;
                    Some(Direction::Right)
                } else {
                    self.y -= 1;
                    None
                }
            }
        } {
            self.direction = new_direction;
        }
        self.step += 1;
        Some(current)
    }
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Problem for Day03 {
    type Input = TrimAndParse<usize>;
    type Part1 = isize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let (_, (x, y)) = SpiralIterator::default()
            .nth(*input - 1)
            .expect("spiral never ends");
        x.abs() + y.abs()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut mem = HashMap::new();
        mem.insert((0, 0), 1);
        for (_, (x, y)) in SpiralIterator::default().skip(1) {
            let new_val = NEIGHBOURS
                .iter()
                .filter_map(|(dx, dy)| mem.get(&(x + *dx, y + *dy)))
                .sum();
            if new_val > *input {
                return new_val;
            }
            mem.insert((x, y), new_val);
        }
        unreachable!()
    }
}

fn main() {
    solve::<Day03>(include_str!("../../inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    #[test]
    fn test_spiral_iterator() {
        let mut spiral = SpiralIterator::default();
        assert_eq!(spiral.next(), Some((1, (0, 0))));
        assert_eq!(spiral.next(), Some((2, (1, 0))));
        assert_eq!(spiral.next(), Some((3, (1, 1))));
        assert_eq!(spiral.next(), Some((4, (0, 1))));
        assert_eq!(spiral.next(), Some((5, (-1, 1))));
        assert_eq!(spiral.next(), Some((6, (-1, 0))));
        assert_eq!(spiral.next(), Some((7, (-1, -1))));
        assert_eq!(spiral.next(), Some((8, (0, -1))));
        assert_eq!(spiral.next(), Some((9, (1, -1))));
        assert_eq!(spiral.next(), Some((10, (2, -1))));
        assert_eq!(spiral.next(), Some((11, (2, 0))));
        assert_eq!(spiral.next(), Some((12, (2, 1))));
    }

    #[test]
    fn test_part1_sample() {
        assert_eq!(solve_part1::<Day03>("1"), 0);
        assert_eq!(solve_part1::<Day03>("12"), 3);
        assert_eq!(solve_part1::<Day03>("23"), 2);
        assert_eq!(solve_part1::<Day03>("1024"), 31);
    }
}
