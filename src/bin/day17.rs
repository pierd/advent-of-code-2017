use aoc_helpers::prelude::*;

struct Day17;

#[derive(Clone, Debug)]
struct SpinLock {
    v: Vec<usize>,
    pos: usize,
    next: usize,
    steps: usize,
}

impl SpinLock {
    fn new(steps: usize) -> Self {
        Self {
            v: vec![0],
            pos: 0,
            next: 1,
            steps,
        }
    }

    fn insert_next(&mut self) {
        self.pos = (self.pos + self.steps) % self.v.len() + 1;
        self.v.insert(self.pos, self.next);
        self.next += 1;
    }

    fn item_after_last_insert(&self) -> usize {
        self.v[(self.pos + 1) % self.v.len()]
    }

    #[allow(unused)]
    fn item_after_0(&self) -> usize {
        let zero_idx = self.v.iter().enumerate().find(|(_, x)| **x == 0).unwrap().0;
        self.v[(zero_idx + 1) % self.v.len()]
    }
}

#[derive(Clone, Debug)]
struct DummySpinLock {
    after_zero: usize,
    pos: usize,
    next: usize,
    steps: usize,
}

impl DummySpinLock {
    fn new(steps: usize) -> Self {
        Self {
            after_zero: 0,
            pos: 0,
            next: 1,
            steps,
        }
    }

    fn insert_next(&mut self) {
        self.pos = (self.pos + self.steps) % self.next;
        if self.pos == 0 {
            self.after_zero = self.next;
        }
        self.pos += 1;
        self.next += 1;
    }

    fn item_after_0(&self) -> usize {
        self.after_zero
    }
}

impl Problem for Day17 {
    type Input = usize;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut spinlock = SpinLock::new(*input);
        for _ in 1..=2017 {
            spinlock.insert_next();
        }
        spinlock.item_after_last_insert()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut spinlock = DummySpinLock::new(*input);
        for _ in 1..=50_000_000 {
            spinlock.insert_next();
        }
        spinlock.item_after_0()
    }
}

fn main() {
    solve::<Day17>("366");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part1;

    const SAMPLE: &str = "3";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day17>(SAMPLE), 638);
    }

    #[test]
    fn compare_spinlocks() {
        let mut regular = SpinLock::new(3);
        let mut dummy = DummySpinLock::new(3);
        for _ in 1..10_000 {
            assert_eq!(dummy.item_after_0(), regular.item_after_0());
            regular.insert_next();
            dummy.insert_next();
        }
    }
}
