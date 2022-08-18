use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day11;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[rematch]
enum Direction {
    #[rematch(r"ne")]
    NorthEast,
    #[rematch(r"se")]
    SouthEast,
    #[rematch(r"sw")]
    SouthWest,
    #[rematch(r"nw")]
    NorthWest,
    #[rematch(r"n")]
    North,
    #[rematch(r"s")]
    South,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::NorthEast => (1, 1),
            Direction::SouthEast => (-1, 1),
            Direction::SouthWest => (-1, -1),
            Direction::NorthWest => (1, -1),
            Direction::North => (2, 0),
            Direction::South => (-2, 0),
        }
    }
}

fn add_points((ay, ax): (isize, isize), (by, bx): (isize, isize)) -> (isize, isize) {
    (ay + by, ax + bx)
}

fn steps((y, x): (isize, isize)) -> usize {
    let mut y = y.unsigned_abs();
    let mut x = x.unsigned_abs();
    let diagonal_steps = [x, y].into_iter().min().unwrap();
    y -= diagonal_steps;
    x -= diagonal_steps;
    let vertical_steps = y / 2;
    y %= 2;
    let horizontal_steps = x;
    assert_eq!(y, 0);
    diagonal_steps + vertical_steps + horizontal_steps
}

impl Problem for Day11 {
    type Input = VecFromCommaSeparated<Direction>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        steps(
            input
                .iter()
                .map(|d| d.offset())
                .reduce(add_points)
                .expect("there should be at least one direction"),
        )
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input
            .iter()
            .map(|d| dbg!(d).offset())
            .fold((0, (0, 0)), |(best_steps, position), p| {
                let new_position = add_points(position, p);
                let new_steps = steps(new_position);
                dbg!((new_position, new_steps));
                (
                    [best_steps, new_steps].into_iter().max().unwrap(),
                    new_position,
                )
            })
            .0
    }
}

fn main() {
    solve::<Day11>(include_str!("../../inputs/day11.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    #[test]
    fn test_part1_sample() {
        assert_eq!(solve_part1::<Day11>("ne,ne,ne"), 3);
        assert_eq!(solve_part1::<Day11>("ne,ne,sw,sw"), 0);
        assert_eq!(solve_part1::<Day11>("ne,ne,s,s"), 2);
        assert_eq!(solve_part1::<Day11>("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_steps() {
        assert_eq!(steps((5, 5)), 5);
        assert_eq!(steps((6, 4)), 5);
        assert_eq!(steps((4, 6)), 6);
    }
}
