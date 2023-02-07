use advent_of_code_2017::knot_hash::knot_hash;
use aoc_helpers::{prelude::*, scaffold::Parse, tile_map::TileMap};

struct Day14;

struct TileMapHasher;

fn u8_to_bools(b: u8) -> [bool; 8] {
    [
        b & 0b10000000 != 0,
        b & 0b01000000 != 0,
        b & 0b00100000 != 0,
        b & 0b00010000 != 0,
        b & 0b00001000 != 0,
        b & 0b00000100 != 0,
        b & 0b00000010 != 0,
        b & 0b00000001 != 0,
    ]
}

fn u8s_to_bools(u8s: &[u8]) -> Vec<bool> {
    u8s.iter()
        .flat_map(|b| u8_to_bools(*b).into_iter())
        .collect()
}

impl Parse for TileMapHasher {
    type Parsed = TileMap<bool>;

    fn parse(raw_input: &str) -> anyhow::Result<Self::Parsed> {
        let tiles: Vec<Vec<bool>> = (0..128)
            .into_iter()
            .map(|row| u8s_to_bools(&knot_hash(&format!("{raw_input}-{row}"))))
            .collect();
        Ok(TileMap::from(tiles))
    }
}

impl Problem for Day14 {
    type Input = TileMapHasher;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().filter(|b| *b).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; 128]; 128];
        let mut regions_count = 0;
        let mut stack: Vec<(isize, isize)> = Default::default();
        for row in 0..128 {
            for col in 0..128 {
                if input.get(row, col).unwrap() && !visited[row as usize][col as usize] {
                    regions_count += 1;

                    // search + mark as visited
                    stack.push((row, col));
                    visited[row as usize][col as usize] = true;
                    while let Some((row, col)) = stack.pop() {
                        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let crow = row + drow;
                            let ccol = col + dcol;
                            if input.get(crow, ccol).unwrap_or_default()
                                && !visited[crow as usize][ccol as usize]
                            {
                                stack.push((crow, ccol));
                                visited[crow as usize][ccol as usize] = true;
                            }
                        }
                    }
                }
            }
        }
        regions_count
    }
}

fn main() {
    solve::<Day14>("jxqlasbh");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "flqrgnkx";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day14>(SAMPLE), 8108);
        assert_eq!(solve_part2::<Day14>(SAMPLE), 1242);
    }

    #[test]
    fn test_u8s_to_bools() {
        assert_eq!(
            u8s_to_bools(&[0xa0, 0xc2, 0x01, 0x70]),
            vec![
                true, false, true, false, false, false, false, false, true, true, false, false,
                false, false, true, false, false, false, false, false, false, false, false, true,
                false, true, true, true, false, false, false, false,
            ]
        );
    }
}
