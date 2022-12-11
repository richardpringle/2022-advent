use std::fs::read_to_string;

const PROBLEM: u8 = 2;

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

type Parsed = Vec<(Move, Move)>;

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&Move> for usize {
    fn from(a_move: &Move) -> Self {
        match a_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Move {
    fn play(&self, other: &Self) -> usize {
        let score = match (self, other) {
            // tie
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => 3,
            // win
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => 6,
            // loss
            _ => 0,
        };

        score + usize::from(self)
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn get_my_move(&self, oponent: &Move) -> Move {
        match (oponent, self) {
            (Move::Rock, Outcome::Win) | (Move::Scissors, Outcome::Loss) => Move::Paper,
            (Move::Rock, Outcome::Loss) | (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Loss) | (Move::Scissors, Outcome::Win) => Move::Rock,
            (oponent, Outcome::Draw) => *oponent,
        }
    }
}

impl From<&Move> for Outcome {
    fn from(a_move: &Move) -> Self {
        match a_move {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

type Part1 = usize;

type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut moves = line.chars().filter_map(|c| match c {
                'A' | 'X' => Some(Move::Rock),
                'B' | 'Y' => Some(Move::Paper),
                'C' | 'Z' => Some(Move::Scissors),
                _ => None,
            });

            (moves.next().unwrap(), moves.next().unwrap())
        })
        .collect()
}

fn part_1(parsed: &Parsed) -> Part1 {
    parsed.iter().map(|(oponent, me)| me.play(oponent)).sum()
}

fn part_2(parsed: &Parsed) -> Part2 {
    parsed
        .iter()
        .map(|(oponent, me)| (oponent, Outcome::from(me)))
        .map(|(oponent, outcome)| outcome.get_my_move(oponent).play(oponent))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
        A Y
        B X
        C Z
    "#;

    const PART_1_TEST_ANS: Part1 = 15;
    const PART_2_TEST_ANS: Part2 = 12;

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
