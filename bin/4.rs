use std::{convert::Infallible, fs::read_to_string, str::FromStr};

const PROBLEM: u8 = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    let part_1 = part_1(&parsed);
    let part_2 = part_2(&parsed);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1);
    println!("part-2: {:?}", part_2);

    Ok(())
}

#[derive(Debug)]
struct Range(usize, usize);

impl FromStr for Range {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.trim().split_once('-').unwrap();
        Ok(Self(left.parse().unwrap(), right.parse().unwrap()))
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.0 <= self.0 && self.0 <= other.1
            || other.0 <= self.1 && self.1 <= other.1
            || self.0 < other.0 && self.1 > other.1
    }
}

type Parsed = Vec<(Range, Range)>;

type Part1 = usize;
type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(left, right)| {
            (
                Range::from_str(left).unwrap(),
                Range::from_str(right).unwrap(),
            )
        })
        .collect()
}

fn part_1(parsed: &Parsed) -> Part1 {
    parsed
        .iter()
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .count()
}

fn part_2(parsed: &Parsed) -> Part2 {
    parsed
        .iter()
        .filter(|(left, right)| left.overlaps(right))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;

    const PART_1_TEST_ANS: Part1 = 2;
    const PART_2_TEST_ANS: Part2 = 4;

    #[test]
    fn part_1() {
        let parsed = super::parse_input(INPUT);
        let part_1_ans = super::part_1(&parsed);

        assert_eq!(part_1_ans, PART_1_TEST_ANS);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse_input(INPUT);
        let part_2_ans = super::part_2(&parsed);

        assert_eq!(part_2_ans, PART_2_TEST_ANS);
    }
}
