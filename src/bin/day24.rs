use std::collections::HashMap;

use aoc_helpers::bitset::*;
use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[rematch(r"(\d+)/(\d+)")]
struct Component {
    port0: usize,
    port1: usize,
}

impl Component {
    fn fit(&self, port: usize) -> Option<usize> {
        if self.port0 == port {
            Some(self.port1)
        } else if self.port1 == port {
            Some(self.port0)
        } else {
            None
        }
    }

    fn strength(&self) -> usize {
        self.port0 + self.port1
    }
}

#[derive(Clone, Copy, Debug)]
struct BridgeState<'c> {
    last_port: usize,
    strength: usize,
    length: usize,
    components_used: u64,
    components: &'c [Component],
}

impl<'c> walk::Generator<Self> for BridgeState<'c> {
    fn generate<F: FnMut(Self)>(&mut self, mut callback: F) {
        for (idx, component) in self.components.iter().enumerate() {
            if !self.components_used.contains(&idx) {
                if let Some(last_port) = component.fit(self.last_port) {
                    let mut components_used = self.components_used;
                    components_used.insert(idx);
                    callback(BridgeState {
                        last_port,
                        strength: self.strength + component.strength(),
                        length: self.length + 1,
                        components_used,
                        components: self.components,
                    })
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct BridgeBuilder<'c> {
    components: &'c [Component],
    ports_to_components: HashMap<usize, u64>,
    best_strength: usize,
    best_length: usize,
    best_strength_for_length: usize,
}

impl<'c> walk::Walker<BridgeState<'c>> for BridgeBuilder<'c> {
    type NextGenerator = BridgeState<'c>;

    type Result = ();

    fn visit(
        &mut self,
        state: &BridgeState<'c>,
    ) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if self.best_strength < state.strength {
            self.best_strength = state.strength;
        }
        if self.best_length < state.length {
            self.best_length = state.length;
            self.best_strength_for_length = state.strength;
        } else if self.best_length == state.length && self.best_strength_for_length < state.strength
        {
            self.best_strength_for_length = state.strength;
        }
        if !self
            .ports_to_components
            .get(&state.last_port)
            .copied()
            .unwrap_or_default()
            .difference(&state.components_used)
            .is_empty()
        {
            // there are some components to use -> generate more states
            walk::VisitDecision::Next(*state)
        } else {
            walk::VisitDecision::Continue
        }
    }
}

impl<'c> BridgeBuilder<'c> {
    fn new(components: &'c [Component]) -> Self {
        let mut ports_to_components: HashMap<usize, u64> = Default::default();
        for (idx, component) in components.iter().enumerate() {
            ports_to_components
                .entry(component.port0)
                .or_default()
                .insert(idx);
            ports_to_components
                .entry(component.port1)
                .or_default()
                .insert(idx);
        }
        Self {
            components,
            ports_to_components,
            best_strength: 0,
            best_length: 0,
            best_strength_for_length: 0,
        }
    }

    fn initial_state(&self) -> BridgeState<'c> {
        BridgeState {
            last_port: 0,
            strength: 0,
            length: 0,
            components_used: 0,
            components: self.components,
        }
    }
}

impl Problem for Day24 {
    type Input = VecFromLines<Component>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut walker = BridgeBuilder::new(input);
        let initial_state = walker.initial_state();
        walk::walk_deep(&mut walker, initial_state);
        walker.best_strength
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut walker = BridgeBuilder::new(input);
        let initial_state = walker.initial_state();
        walk::walk_deep(&mut walker, initial_state);
        walker.best_strength_for_length
    }
}

fn main() {
    solve::<Day24>(include_str!("../../inputs/day24.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str =
        concat!("0/2\n", "2/2\n", "2/3\n", "3/4\n", "3/5\n", "0/1\n", "10/1\n", "9/10\n",);

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day24>(SAMPLE), 31);
        assert_eq!(solve_part2::<Day24>(SAMPLE), 19);
    }
}
