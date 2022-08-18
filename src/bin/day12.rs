use std::{collections::HashSet, str::FromStr};

use aoc_helpers::prelude::*;

struct Day12;

#[derive(Clone, Debug)]
pub struct Node {
    pub idx: usize,
    vertices: Vec<usize>,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" <-> ");
        let idx: usize = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing id"))?
            .parse()
            .map_err(|e| anyhow::anyhow!("Error parsing id: {}", e))?;
        let vertices: Vec<usize> = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing vertices"))?
            .split(", ")
            .map(|s| {
                s.parse()
                    .map_err(|e| anyhow::anyhow!("Error parsing vertex id: {}", e))
            })
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Self { idx, vertices })
    }
}

fn find_group(nodes: &[Node], starting_idx: usize) -> HashSet<usize> {
    let mut visited: HashSet<usize> = Default::default();
    let mut stack: Vec<usize> = Default::default();
    stack.push(starting_idx);
    while let Some(idx) = stack.pop() {
        if visited.insert(idx) {
            stack.extend_from_slice(&nodes[idx].vertices);
        }
    }
    visited
}

impl Problem for Day12 {
    type Input = VecFromLines<Node>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        find_group(input, 0).len()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut all_visited: HashSet<usize> = Default::default();
        let mut groups_count = 0;
        for idx in 0..input.len() {
            if !all_visited.contains(&idx) {
                groups_count += 1;
                all_visited.extend(find_group(input, idx).into_iter());
            }
        }
        groups_count
    }
}

fn main() {
    solve::<Day12>(include_str!("../../inputs/day12.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day12>(SAMPLE), Default::default());
        assert_eq!(solve_part2::<Day12>(SAMPLE), Default::default());
    }
}
