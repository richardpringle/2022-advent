use std::fs::read_to_string;

const PROBLEM: u8 = 6;

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
struct Buffer<const N: usize>([u8; N], usize);

impl<const N: usize> Buffer<N> {
    fn new(inner: [u8; N]) -> Self {
        Self(inner, 0)
    }

    fn insert(&mut self, val: u8) {
        let Self(mut inner, i) = self;

        inner[*i] = val;

        *self = Self(inner, (*i + 1) % N);
    }

    fn are_all_unique(&self) -> bool {
        self.0
            .into_iter()
            .map(|val| self.0.into_iter().filter(|other| val == *other).count())
            .all(|val| val == 1)
    }
}

type Parsed = Vec<u8>;

type Part1 = usize;
type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    input.trim().as_bytes().to_vec()
}

fn solve<const N: usize>(parsed: &Parsed) -> usize {
    let mut buffer = Buffer::<N>::new(parsed[0..N].try_into().unwrap());
    let mut index = N;

    if buffer.are_all_unique() {
        return index;
    }

    for val in parsed.into_iter().skip(N) {
        index += 1;
        buffer.insert(*val);

        if buffer.are_all_unique() {
            return index;
        }
    }

    index
}

fn part_1(parsed: &Parsed) -> Part1 {
    solve::<4>(parsed)
}

fn part_2(parsed: &Parsed) -> Part2 {
    solve::<14>(parsed)
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#;

    const PART_1_TEST_ANS: Part1 = 7;
    const PART_2_TEST_ANS: Part2 = 19;

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
