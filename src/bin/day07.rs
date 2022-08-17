use std::collections::HashMap;

use aoc_helpers::prelude::*;

struct Day07;

#[derive(Clone, Debug)]
struct Program<'a> {
    name: &'a str,
    weight: usize,
    subs: Vec<&'a str>,
}

impl<'a> Program<'a> {
    fn parse(s: &'a str) -> Result<Self, anyhow::Error> {
        lazy_static::lazy_static! {
            // vpbdpfm (74) -> ndegtj, wnwxs
            static ref RE: regex::Regex = regex::Regex::new(r"(\w+) \((\d+)\)( -> ([\w, ]+))?").unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            Ok(Self {
                name: caps
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Getting name failed"))?
                    .as_str(),
                weight: caps
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("Getting weight failed"))?
                    .as_str()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Field 'weight' parsing error: {}", e))?,
                subs: caps
                    .get(4)
                    .map(|m| m.as_str().split(", ").collect::<Vec<_>>())
                    .unwrap_or_default(),
            })
        } else {
            Err(anyhow::anyhow!("Regex didn't match"))
        }
    }

    fn parse_all(s: &'a str) -> Result<Vec<Self>, anyhow::Error> {
        s.split('\n')
            .map(Program::parse)
            .collect::<Result<Vec<_>, _>>()
    }
}

struct Programs<'a> {
    programs: Vec<Program<'a>>,
    name_to_idx: HashMap<&'a str, usize>,
    name_to_parent_name: HashMap<&'a str, &'a str>,
    name_to_tree_weight: HashMap<&'a str, usize>,
}

impl<'a> Programs<'a> {
    fn new(programs: Vec<Program<'a>>) -> Self {
        let mut name_to_idx: HashMap<&'a str, usize> = Default::default();
        let mut name_to_parent_name: HashMap<&'a str, &'a str> = Default::default();
        for (idx, program) in programs.iter().enumerate() {
            name_to_idx.insert(program.name, idx);
            for sub in &program.subs {
                assert!(name_to_parent_name.insert(sub, program.name).is_none());
            }
        }

        Self {
            programs,
            name_to_idx,
            name_to_parent_name,
            name_to_tree_weight: Default::default(),
        }
    }

    fn find_bottom(&self) -> Option<&str> {
        for program in &self.programs {
            if !self.name_to_parent_name.contains_key(program.name) {
                return Some(program.name);
            }
        }
        None
    }

    fn fill_tree_weight(&mut self, bottom_tree_name: &str) {
        let mut stack = Vec::new();
        stack.push(
            *self
                .name_to_idx
                .get(bottom_tree_name)
                .expect("bottom_tree_name should be in the tree"),
        );
        while let Some(idx) = stack.pop() {
            let program = self.programs.get(idx).unwrap();
            let sub_weight = program
                .subs
                .iter()
                .map(|sub_name| self.name_to_tree_weight.get(sub_name).copied())
                .reduce(|accum, item| {
                    if let (Some(a), Some(i)) = (accum, item) {
                        Some(a + i)
                    } else {
                        None
                    }
                })
                // first level option being None means there are no sub programs
                .unwrap_or(Some(0));
            if let Some(sub_weight) = sub_weight {
                // we have weights of all the sub trees
                self.name_to_tree_weight
                    .insert(program.name, program.weight + sub_weight);
            } else {
                // something is missing -> we'll have to come back to this node so push it onto stack
                stack.push(idx);
                // push all the missing nodes
                stack.extend(program.subs.iter().filter_map(|sub_name| {
                    if self.name_to_tree_weight.contains_key(sub_name) {
                        None
                    } else {
                        Some(self.name_to_idx.get(sub_name).unwrap())
                    }
                }));
            }
        }
    }

    fn find_unbalanced_weight(&self) -> usize {
        for program in &self.programs {
            if !program.subs.is_empty() {
                let mut weights: HashMap<usize, usize> = Default::default();
                for sub in &program.subs {
                    *weights
                        .entry(
                            *self
                                .name_to_tree_weight
                                .get(sub)
                                .expect("weights should be filled"),
                        )
                        .or_default() += 1;
                }
                if weights.len() > 1 {
                    let weight = *weights
                        .iter()
                        .find(|(_, v)| **v == 1)
                        .expect("there should be a node with an odd weight")
                        .0;
                    let other_weight = weights
                        .into_iter()
                        .find(|(_, v)| *v > 1)
                        .expect("there should be a node with a common weight")
                        .0;
                    let sub_name = program
                        .subs
                        .iter()
                        .find_map(|sub_name| {
                            if *self.name_to_tree_weight.get(sub_name).unwrap() == weight {
                                Some(*sub_name)
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    let sub = self
                        .programs
                        .get(*self.name_to_idx.get(sub_name).unwrap())
                        .unwrap();
                    return other_weight + sub.weight - weight;
                }
            }
        }
        unreachable!("there should be an unbalanced node")
    }
}

impl Problem for Day07 {
    type Input = String;
    type Part1 = String;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let programs = Programs::new(Program::parse_all(input).expect("input should parse"));
        programs
            .find_bottom()
            .expect("there should be a bottom program")
            .to_owned()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut programs = Programs::new(Program::parse_all(input).expect("input should parse"));
        let bottom = programs
            .find_bottom()
            .expect("there should be a bottom program")
            .to_owned();
        programs.fill_tree_weight(&bottom);
        programs.find_unbalanced_weight()
    }
}

fn main() {
    solve::<Day07>(include_str!("../../inputs/day07.txt"));
}
