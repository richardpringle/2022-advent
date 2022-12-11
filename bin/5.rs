use std::{convert::Infallible, fs::read_to_string, str::FromStr};

const PROBLEM: u8 = 5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    let part_1 = part_1(parsed.clone());
    let part_2 = part_2(parsed);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1);
    println!("part-2: {:?}", part_2);

    Ok(())
}

#[derive(Debug)]
struct Crate(Option<char>);

impl FromStr for Crate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err("pass in the proper size string");
        }

        let mut chars = s.chars();

        chars.next();

        let inner = chars.next().filter(|c| *c != ' ');

        Ok(Self(inner))
    }
}

impl Crate {
    fn take_from_str(s: &str) -> (Self, &str) {
        if s.len() > 3 {
            (s[0..3].parse().unwrap(), &s[4..])
        } else {
            (s[0..3].parse().unwrap(), "")
        }
    }
}

struct Crates(Vec<Crate>);

impl FromStr for Crates {
    type Err = Infallible;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut crates = vec![];

        while !s.is_empty() {
            let (one_crate, rest) = Crate::take_from_str(s);
            crates.push(one_crate);
            s = rest;
        }

        Ok(Self(crates))
    }
}

#[derive(Clone, Debug)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Self {
        Self(vec![])
    }

    fn push(&mut self, c: char) {
        self.0.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn peak(&self) -> Option<&char> {
        self.0.last()
    }
}

#[derive(Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn move_crates_one_at_a_time(&mut self, a_move: Move) {
        for _ in 0..a_move.count {
            let a_crate = self.0[a_move.from - 1].pop().unwrap();
            self.0[a_move.to - 1].push(a_crate)
        }
    }

    fn move_crates_together(&mut self, a_move: Move) {
        let mut crates = vec![];

        for _ in 0..a_move.count {
            let a_crate = self.0[a_move.from - 1].pop().unwrap();
            crates.push(a_crate);
        }

        crates
            .into_iter()
            .rev()
            .for_each(|a_crate| self.0[a_move.to - 1].push(a_crate));
    }
}

impl std::fmt::Debug for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (i, stack) in self.0.iter().enumerate() {
            writeln!(f, "{} {:?}", i + 1, stack.0)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[derive(Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Move {} from {} to {}", self.count, self.from, self.to)
    }
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ').filter_map(|c| usize::from_str(c).ok());

        Ok(Self {
            count: iter.next().unwrap(),
            from: iter.next().unwrap(),
            to: iter.next().unwrap(),
        })
    }
}

type Parsed = (Stacks, Vec<Move>);

type Part1 = String;
type Part2 = String;

fn parse_input(input: &str) -> Parsed {
    let (stacks_raw, moves_raw) = input.split_once("\n\n").unwrap();

    let stacks = {
        let mut lines: Vec<&str> = stacks_raw.lines().filter(|line| !line.is_empty()).collect();
        let col_count = lines
            .pop()
            .unwrap()
            .trim()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap() as usize;

        let rows: Vec<Crates> = lines
            .into_iter()
            .map(|line| Crates::from_str(line).unwrap())
            .collect();

        let mut stacks = vec![Stack::new(); col_count];

        rows.into_iter().rev().for_each(|row| {
            row.0.into_iter().enumerate().for_each(|(col, val)| {
                if let Crate(Some(c)) = val {
                    stacks[col].push(c)
                }
            })
        });

        Stacks(stacks)
    };

    let moves = moves_raw
        .trim()
        .lines()
        .map(|line| Move::from_str(line).unwrap())
        .collect();

    (stacks, moves)
}

fn part_1(parsed: Parsed) -> Part1 {
    let (mut stacks, moves) = parsed;

    for a_move in moves {
        stacks.move_crates_one_at_a_time(a_move);
    }

    stacks.0.iter().filter_map(|stack| stack.peak()).collect()
}

fn part_2(parsed: Parsed) -> Part2 {
    let (mut stacks, moves) = parsed;

    for a_move in moves {
        stacks.move_crates_together(a_move);
    }

    stacks.0.iter().filter_map(|stack| stack.peak()).collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    #[test]
    fn part_1() {
        let parsed = super::parse_input(INPUT);
        let part_1_ans = super::part_1(parsed);

        assert_eq!(part_1_ans, String::from("CMZ"));
    }

    #[test]
    fn part_2() {
        let parsed = super::parse_input(INPUT);
        let part_2_ans = super::part_2(parsed);

        assert_eq!(part_2_ans, String::from("MCD"));
    }
}
