use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use aoc_helpers::matrix::*;
use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day21;

#[derive(Clone, Debug, PartialEq, Eq)]
struct BitMap {
    map: Vec<Vec<bool>>,
}

impl From<Vec<Vec<bool>>> for BitMap {
    fn from(map: Vec<Vec<bool>>) -> Self {
        Self { map }
    }
}

impl FromStr for BitMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<bool>> = s
            .split('/')
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Self::from(map))
    }
}

impl Display for BitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for b in row {
                f.write_char(if *b { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn bool_iter_to_int<'a, I: Iterator<Item = &'a bool>>(iter: I) -> usize {
    iter.fold(0, |acc, item| (acc << 1) | if *item { 1 } else { 0 })
}

impl BitMap {
    fn starting_pattern() -> Self {
        Self::from(vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ])
    }

    fn size(&self) -> usize {
        self.map.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ChecksummedBitMap {
    size: usize,
    checksum: usize,
}

impl From<&Vec<Vec<bool>>> for ChecksummedBitMap {
    fn from(m: &Vec<Vec<bool>>) -> Self {
        let size = m.rows();
        let mut checksum = bool_iter_to_int(m.iter_by_rows());
        let mut try_checksum = |candidate: usize| {
            if checksum < candidate {
                checksum = candidate;
            }
        };

        let temp1 = m.flip_horizontally();
        let temp = temp1.iter_by_rows();
        try_checksum(bool_iter_to_int(temp));
        try_checksum(bool_iter_to_int(m.flip_vertically().iter_by_rows()));
        try_checksum(bool_iter_to_int(
            m.flip_horizontally().flip_vertically().iter_by_rows(),
        ));

        try_checksum(bool_iter_to_int(m.rotate().iter_by_rows()));
        try_checksum(bool_iter_to_int(
            m.rotate().flip_horizontally().iter_by_rows(),
        ));
        try_checksum(bool_iter_to_int(
            m.rotate().flip_vertically().iter_by_rows(),
        ));
        try_checksum(bool_iter_to_int(
            m.rotate()
                .flip_horizontally()
                .flip_vertically()
                .iter_by_rows(),
        ));

        Self { size, checksum }
    }
}

#[derive(Clone, Debug)]
#[rematch(r"([.#/]+) => ([.#/]+)")]
struct Rule {
    pattern: BitMap,
    output: BitMap,
}

#[derive(Debug)]
struct RuleSet {
    rules: HashMap<ChecksummedBitMap, Rule>,
}

impl From<&[Rule]> for RuleSet {
    fn from(rules: &[Rule]) -> Self {
        Self {
            rules: rules
                .iter()
                .map(|rule| (ChecksummedBitMap::from(&rule.pattern.map), rule.clone()))
                .collect(),
        }
    }
}

impl RuleSet {
    fn iterate(&self, image: BitMap) -> BitMap {
        let output_size = if image.size() % 2 == 0 {
            image.size() / 2 * 3
        } else {
            image.size() / 3 * 4
        };
        let mut map = vec![vec![false; output_size]; output_size];
        let (rule_size, output_tile_size) = if image.size() % 2 == 0 {
            (2, 3)
        } else {
            (3, 4)
        };

        for row in (0..image.size()).step_by(rule_size) {
            for col in (0..image.size()).step_by(rule_size) {
                let slice = image
                    .map
                    .slice(row..(row + rule_size), col..(col + rule_size))
                    .to_vec_vec();
                let checksummed = ChecksummedBitMap::from(&slice);
                let output = self.rules.get(&checksummed).unwrap();
                for (row_idx, output_row) in output.output.map.iter().enumerate() {
                    for (col_idx, b) in output_row.iter().enumerate() {
                        map[row / rule_size * output_tile_size + row_idx]
                            [col / rule_size * output_tile_size + col_idx] = *b;
                    }
                }
            }
        }

        BitMap { map }
    }
}

impl Problem for Day21 {
    type Input = VecFromLines<Rule>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let rules = RuleSet::from(input.as_slice());
        let mut image = BitMap::starting_pattern();
        for _ in 0..5 {
            image = rules.iterate(image);
        }
        image.map.iter_by_rows().filter(|b| **b).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let rules = RuleSet::from(input.as_slice());
        let mut image = BitMap::starting_pattern();
        for _ in 0..18 {
            image = rules.iterate(image);
        }
        image.map.iter_by_rows().filter(|b| **b).count()
    }
}

fn main() {
    solve::<Day21>(include_str!("../../inputs/day21.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::Parse;

    const SAMPLE: &str = concat!(
        "../.# => ##./#../...\n",
        ".#./..#/### => #..#/..../..../#..#\n",
    );

    #[test]
    fn test_sample() {
        let rules = RuleSet::from(VecFromLines::<Rule>::parse(SAMPLE).unwrap().as_slice());
        let mut image = BitMap::starting_pattern();
        image = rules.iterate(image);
        assert_eq!(image.map.iter_by_rows().filter(|b| **b).count(), 4);
        image = rules.iterate(image);
        assert_eq!(image.map.iter_by_rows().filter(|b| **b).count(), 12);
    }

    #[test]
    fn test_parsing_starting_pattern() {
        assert_eq!(BitMap::starting_pattern(), ".#./..#/###".parse().unwrap());
    }
}
