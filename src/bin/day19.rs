use aoc_helpers::{prelude::*, tile_map::TileMap};

struct Day19;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Horizontal,
    Vertical,
    Both,
    Letter(char),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Self::Empty),
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            '+' => Ok(Self::Both),
            'A'..='Z' | 'a'..='z' => Ok(Self::Letter(value)),
            _ => Err(anyhow::anyhow!("Can't parse {:?}", value)),
        }
    }
}

fn solve_both(tiles: &Vec<Vec<Tile>>) -> (String, usize) {
    let map = TileMap::from(tiles);
    let mut letters = String::new();
    let mut steps = 0;
    let (mut row, mut col) = (0isize, 0isize);
    let (mut drow, mut dcol) = (1isize, 0isize);

    for (i, t) in tiles[0].iter().enumerate() {
        if *t != Tile::Empty {
            assert_eq!(*t, Tile::Vertical);
            col = i as isize;
            break;
        }
    }

    loop {
        match map.get(row, col) {
            Some(Tile::Both) => {
                std::mem::swap(&mut drow, &mut dcol);
                if matches!(map.get(row + drow, col + dcol), None | Some(Tile::Empty)) {
                    drow *= -1;
                    dcol *= -1;
                }
            }
            Some(Tile::Horizontal | Tile::Vertical) => {}
            Some(Tile::Letter(c)) => letters.push(c),
            Some(Tile::Empty) | None => break,
        }
        row += drow;
        col += dcol;
        steps += 1;
    }

    (letters, steps)
}

impl Problem for Day19 {
    type Input = RowsOfChars<Tile>;
    type Part1 = String;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        solve_both(input).0
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        solve_both(input).1
    }
}

fn main() {
    solve::<Day19>(include_str!("../../inputs/day19.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "        |          \n",
        "        |  +--+    \n",
        "        A  |  C    \n",
        "    F---|----E|--+ \n",
        "        |  |  |  D \n",
        "        +B-+  +--+ \n",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day19>(SAMPLE), "ABCDEF".to_owned());
        assert_eq!(solve_part2::<Day19>(SAMPLE), 38);
    }
}
