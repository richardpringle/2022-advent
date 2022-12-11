use std::{collections::HashSet, fs::read_to_string};

const PROBLEM: u8 = 3;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Item(char);
struct Priority(usize);

impl From<&Item> for Priority {
    fn from(item: &Item) -> Self {
        let inner = match item.0 {
            c @ 'a'..='z' => (u8::try_from(c).unwrap() - u8::try_from('a').unwrap()) + 1,
            c @ 'A'..='Z' => (u8::try_from(c).unwrap() - u8::try_from('A').unwrap()) + 27,
            _ => unreachable!(),
        } as usize;

        Self(inner)
    }
}

type Parsed = Vec<(HashSet<Item>, HashSet<Item>)>;

type Part1 = usize;
type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            (
                left.chars().map(Item).collect(),
                right.chars().map(Item).collect(),
            )
        })
        .collect()
}

fn part_1(parsed: &Parsed) -> Part1 {
    parsed
        .iter()
        .flat_map(|(left, right)| left.intersection(right))
        .map(Priority::from)
        .map(|p| p.0)
        .sum()
}

fn part_2(parsed: &Parsed) -> Part2 {
    let lines: Vec<HashSet<&Item>> = parsed
        .iter()
        .map(|(left, right)| HashSet::from_iter(left.union(right)))
        .collect();

    lines
        .chunks(3)
        .flat_map(|chunks| {
            HashSet::from_iter(chunks[0].intersection(&chunks[1]).copied())
                .intersection(&chunks[2])
                .copied()
                .collect::<Vec<&Item>>()
        })
        .map(Priority::from)
        .map(|p| p.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    const PART_1_TEST_ANS: Part1 = 157;
    const PART_2_TEST_ANS: Part2 = 70;

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
