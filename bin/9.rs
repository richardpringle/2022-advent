use std::{collections::HashSet, fs::read_to_string, str::FromStr};

const PROBLEM: u8 = 9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1(&parsed));
    println!("part-2: {:?}", part_2(&parsed));

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ').map(Ok).unwrap_or(Err(()))? {
            ("L", count) => Ok(Self::Left(count.parse().map_err(|_| ())?)),
            ("R", count) => Ok(Self::Right(count.parse().map_err(|_| ())?)),
            ("U", count) => Ok(Self::Up(count.parse().map_err(|_| ())?)),
            ("D", count) => Ok(Self::Down(count.parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [(isize, isize); N],
    tail_visits: HashSet<(isize, isize)>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knots: [(0, 0); N],
            tail_visits: HashSet::from([(0, 0)]),
        }
    }

    fn make_moves(&mut self, moves: Move) {
        let (count, move_fn): (usize, &dyn Fn(&mut (isize, isize)) -> ()) = match moves {
            Move::Left(count) => (count, &|pos| {
                pos.0 -= 1;
            }),
            Move::Right(count) => (count, &|pos| {
                pos.0 += 1;
            }),
            Move::Up(count) => (count, &|pos| {
                pos.1 += 1;
            }),
            Move::Down(count) => (count, &|pos| {
                pos.1 -= 1;
            }),
        };

        for _ in 0..count {
            move_fn(self.head_mut());
            self.catch_up();
            self.record_tail_position();
        }
    }

    fn catch_up(&mut self) {
        for (head, tail) in (0usize..N)
            .map(|i| (i, i + 1))
            .take_while(|(_, second)| *second < N)
        {
            let (head, tail) = (self.knots[head], &mut self.knots[tail]);

            if head.0.abs_diff(tail.0) > 1 {
                if head.1.abs_diff(tail.1) > 0 {
                    Self::catch_up_y(head, tail);
                }

                Self::catch_up_x(head, tail);
            }

            if head.1.abs_diff(tail.1) > 1 {
                if head.0.abs_diff(tail.0) > 0 {
                    Self::catch_up_x(head, tail);
                }

                Self::catch_up_y(head, tail);
            }
        }
    }

    fn catch_up_x(head: (isize, isize), tail: &mut (isize, isize)) {
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
    }

    fn catch_up_y(head: (isize, isize), tail: &mut (isize, isize)) {
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    }

    fn head_mut(&mut self) -> &mut (isize, isize) {
        &mut self.knots[0]
    }

    fn tail(&self) -> (isize, isize) {
        self.knots[N - 1]
    }

    fn record_tail_position(&mut self) {
        self.tail_visits.insert(self.tail());
    }

    fn into_tail_count(self) -> usize {
        self.tail_visits.len()
    }
}

type Parsed = Vec<Move>;

type Part1 = usize;
type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Move>, _>>()
        .unwrap()
}

fn part_1(parsed: &Parsed) -> Part1 {
    parsed
        .into_iter()
        .fold(Rope::<2>::new(), |mut rope, m| {
            rope.make_moves(*m);
            rope
        })
        .into_tail_count()
}

fn part_2(parsed: &Parsed) -> Part2 {
    parsed
        .into_iter()
        .fold(Rope::<10>::new(), |mut rope, m| {
            rope.make_moves(*m);
            rope
        })
        .into_tail_count()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "#;

    const PART_1_TEST_ANS: Part1 = 13;
    const PART_2_TEST_ANS: Part2 = 36;

    #[test]
    fn part_1() {
        let parsed = super::parse_input(INPUT);
        let part_1_ans = super::part_1(&parsed);

        assert_eq!(part_1_ans, PART_1_TEST_ANS);
    }

    #[test]
    fn part_2() {
        let parsed = super::parse_input(
            r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
        "#,
        );
        let part_2_ans = super::part_2(&parsed);

        assert_eq!(part_2_ans, PART_2_TEST_ANS);
    }
}
