use std::collections::HashMap;

use aoc_helpers::prelude::*;

struct Day22;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InfectionStatus {
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WrappedInfectionStatus(Option<InfectionStatus>);

impl TryFrom<char> for WrappedInfectionStatus {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self(Some(InfectionStatus::Infected))),
            '.' => Ok(Self(None)),
            _ => Err(anyhow::anyhow!("Unknown infection status: {:?}", value)),
        }
    }
}

fn run<const SIMPLE_MODE: bool>(
    starting_map: &Vec<Vec<WrappedInfectionStatus>>,
    steps: usize,
) -> usize {
    let mut map: HashMap<(isize, isize), InfectionStatus> = Default::default();
    for (row_idx, row) in starting_map.iter().enumerate() {
        for (col_idx, status) in row.iter().enumerate() {
            if let WrappedInfectionStatus(Some(status)) = *status {
                map.insert((row_idx as isize, col_idx as isize), status);
            }
        }
    }

    let mut direction = (-1isize, 0);
    let (mut row, mut col) = (
        (starting_map.len() / 2) as isize,
        (starting_map[0].len() / 2) as isize,
    );
    let mut infections = 0;
    for _ in 0..steps {
        let infected = map.get(&(row, col)).copied();
        if SIMPLE_MODE {
            match infected {
                None => {
                    direction = (-direction.1, direction.0);
                    infections += 1;
                    map.insert((row, col), InfectionStatus::Infected);
                }
                Some(InfectionStatus::Infected) => {
                    direction = (direction.1, -direction.0);
                    map.remove(&(row, col));
                }
                Some(InfectionStatus::Weakened | InfectionStatus::Flagged) => panic!(),
            }
        } else {
            match infected {
                None => {
                    direction = (-direction.1, direction.0);
                    map.insert((row, col), InfectionStatus::Weakened);
                }
                Some(InfectionStatus::Weakened) => {
                    infections += 1;
                    map.insert((row, col), InfectionStatus::Infected);
                }
                Some(InfectionStatus::Infected) => {
                    direction = (direction.1, -direction.0);
                    map.insert((row, col), InfectionStatus::Flagged);
                }
                Some(InfectionStatus::Flagged) => {
                    direction = (-direction.0, -direction.1);
                    map.remove(&(row, col));
                }
            }
        }
        row += direction.0;
        col += direction.1;
    }
    infections
}

impl Problem for Day22 {
    type Input = RowsOfChars<WrappedInfectionStatus>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        run::<true>(input, 10_000)
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        run::<false>(input, 10_000_000)
    }
}

fn main() {
    solve::<Day22>(include_str!("../../inputs/day22.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::Parse;

    const SAMPLE: &str = concat!("..#\n", "#..\n", "...\n",);

    #[test]
    fn test_sample() {
        let map = RowsOfChars::<WrappedInfectionStatus>::parse(SAMPLE).unwrap();
        assert_eq!(run::<true>(&map, 7), 5);
        assert_eq!(run::<false>(&map, 100), 26);
    }
}
