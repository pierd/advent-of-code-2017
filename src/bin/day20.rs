use std::collections::HashMap;

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day20;

#[derive(Clone, Copy, Debug)]
#[rematch(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>")]
struct Particle {
    px: isize,
    py: isize,
    pz: isize,
    vx: isize,
    vy: isize,
    vz: isize,
    ax: isize,
    ay: isize,
    az: isize,
}

impl Particle {
    fn update(&mut self) {
        self.vx = self.vx.checked_add(self.ax).unwrap_or(self.vx);
        self.vy = self.vy.checked_add(self.ay).unwrap_or(self.vy);
        self.vz = self.vz.checked_add(self.az).unwrap_or(self.vz);
        self.px = self.px.checked_add(self.vx).unwrap_or(self.px);
        self.py = self.py.checked_add(self.vy).unwrap_or(self.py);
        self.pz = self.pz.checked_add(self.vz).unwrap_or(self.pz);
    }

    fn dist(&self) -> usize {
        self.vx
            .unsigned_abs()
            .checked_add(self.vy.unsigned_abs())
            .and_then(|x| x.checked_add(self.vz.unsigned_abs()))
            .unwrap_or(usize::MAX)
    }

    const fn position(&self) -> (isize, isize, isize) {
        (self.px, self.py, self.pz)
    }
}

impl Problem for Day20 {
    type Input = VecFromLines<Particle>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut particles = input.clone();
        for _ in 0..1000 {
            for p in particles.iter_mut() {
                p.update();
            }
        }
        particles
            .into_iter()
            .enumerate()
            .min_by_key(|(_, p)| p.dist())
            .expect("there should be particles")
            .0
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut destroyed = vec![false; input.len()];
        let mut particles = input.clone();
        for _ in 0..1000 {
            let mut positions = HashMap::new();
            for (idx, p) in particles.iter_mut().enumerate() {
                if !destroyed[idx] {
                    p.update();
                    if let Some(other_idx) = positions.insert(p.position(), idx) {
                        destroyed[idx] = true;
                        destroyed[other_idx] = true;
                    }
                }
            }
        }
        destroyed.into_iter().filter(|b| !*b).count()
    }
}

fn main() {
    solve::<Day20>(include_str!("../../inputs/day20.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE1: &str = concat!(
        "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n",
        "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n",
    );

    const SAMPLE2: &str = concat!(
        "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n",
        "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n",
        "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n",
        "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\n",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day20>(SAMPLE1), 0);
        assert_eq!(solve_part2::<Day20>(SAMPLE2), 1);
    }
}
