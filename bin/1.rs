use std::{collections::BinaryHeap, fs::read_to_string};

const PROBLEM: u8 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let elves = parse_elves_input(&input);

    let largest = find_largest(&elves);
    let largest_three = find_largest_three(&elves);

    println!("Problem {PROBLEM}");
    println!("part-1: {largest}");
    println!("part-2: {largest_three}");

    Ok(())
}

fn parse_elves_input(input: &str) -> Vec<usize> {
    input.trim().lines().fold(vec![0], |mut elves, line| {
        let calories: usize = line.trim().parse().unwrap_or_default();

        if calories > 0 {
            // elves is never empty
            *elves.last_mut().unwrap() += calories;
        } else {
            elves.push(0);
        }

        elves
    })
}

fn find_largest(elves: &[usize]) -> usize {
    *elves.iter().max().unwrap()
}

fn find_largest_three(elves: &[usize]) -> usize {
    elves
        .iter()
        .collect::<BinaryHeap<_>>()
        .into_iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "#;

    #[test]
    fn part_1() {
        let elves = parse_elves_input(INPUT);

        let largest = find_largest(&elves);

        assert_eq!(largest, 24000);
    }

    #[test]
    fn part_2() {
        let elves = parse_elves_input(INPUT);

        let largest = find_largest_three(&elves);

        assert_eq!(largest, 45000);
    }
}
