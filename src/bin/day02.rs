use aoc_helpers::prelude::*;

struct Day02;

impl Problem for Day02 {
    type Input = VecFromLines<VecFromWhitespaceSeparated<TrimAndParse<usize>>>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .flat_map(|(idx, x)| {
                        std::iter::repeat(*x).zip(row.split_at(idx + 1).1.iter().copied())
                    })
                    .filter_map(|(x, y)| {
                        let (modulo, div) = if x <= y {
                            (y % x, y / x)
                        } else {
                            (x % y, x / y)
                        };
                        if modulo == 0 {
                            Some(div)
                        } else {
                            None
                        }
                    })
                    .next()
                    .expect("there should be a pair")
            })
            .sum()
    }
}

fn main() {
    solve::<Day02>(include_str!("../../inputs/day02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::solve_part2;

    const SAMPLE: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part2::<Day02>(SAMPLE), 9);
    }
}
