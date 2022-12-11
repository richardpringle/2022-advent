use std::{fs::read_to_string, ops::Range};

const PROBLEM: u8 = 8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1(&parsed));
    println!("part-2: {:?}", part_2(&parsed));

    Ok(())
}

struct Forest(Vec<Vec<u8>>);

impl std::fmt::Debug for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Forest")?;

        for row in self.0.iter() {
            for val in row.iter() {
                write!(f, "{} ", val)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Forest {
    fn tree(&self, i: usize, j: usize) -> u8 {
        self.0[i][j]
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        let len = self.0.len();
        let tree = self.tree(i, j);

        let left = maybe_add_one(0..j, |range| {
            range.rev().take_while(|&j| tree > self.tree(i, j)).count()
        });

        let right = maybe_add_one(j + 1..len, |range| {
            range.take_while(|&j| tree > self.tree(i, j)).count()
        });

        let up = maybe_add_one(0..i, |range| {
            range.rev().take_while(|&i| tree > self.tree(i, j)).count()
        });
        let down = maybe_add_one(i + 1..len, |range| {
            range.take_while(|&i| tree > self.tree(i, j)).count()
        });

        left * right * up * down
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn maybe_add_one(range: Range<usize>, f: impl FnOnce(Range<usize>) -> usize) -> usize {
    let len = range.end - range.start;

    let val = f(range);

    if val == len {
        val
    } else {
        val + 1
    }
}

struct VisibilityMap<'a>(Vec<Vec<bool>>, &'a Forest);

impl std::fmt::Debug for VisibilityMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VisibilityMap")?;

        for row in self.0.iter() {
            for val in row.iter() {
                write!(f, "{} ", i32::from(*val))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<'a> VisibilityMap<'a> {
    fn from_forest(forest: &'a Forest) -> Self {
        let len = forest.len();

        let mut this = Self(vec![vec![false; len]; len], forest);

        // top
        this.0[0].iter_mut().for_each(|val| {
            *val = true;
        });

        // bottom
        this.0[len - 1].iter_mut().for_each(|val| {
            *val = true;
        });

        // sides
        this.0.iter_mut().skip(1).take(len - 2).for_each(|row| {
            row[0] = true;
            row[len - 1] = true;
        });

        this.check_visibility();

        this
    }

    fn tree_mut(&mut self, i: usize, j: usize) -> &mut bool {
        &mut self.0[i][j]
    }

    fn check_visibility(&mut self) {
        let forest = self.1;

        let len = forest.len();

        // check rows
        for i in 1..(len - 1) {
            let mut max = forest.tree(i, 0);

            // from the front
            for j in 1..(len - 1) {
                let curr = forest.tree(i, j);

                if curr > max {
                    *self.tree_mut(i, j) = true;
                    max = curr;
                }
            }

            max = forest.tree(i, len - 1);
            // from the back
            for j in (1..(len - 1)).rev() {
                let curr = forest.tree(i, j);

                if curr > max {
                    *self.tree_mut(i, j) = true;
                    max = curr;
                }
            }
        }

        for j in 1..(len - 1) {
            let mut max = forest.tree(0, j);

            // from the top
            for i in 1..(len - 1) {
                let curr = forest.tree(i, j);

                if curr > max {
                    *self.tree_mut(i, j) = true;
                    max = curr;
                }
            }

            max = forest.tree(len - 1, j);

            // from the bottom
            for i in (1..(len - 1)).rev() {
                let curr = forest.tree(i, j);

                if curr > max {
                    *self.tree_mut(i, j) = true;
                    max = curr;
                }
            }
        }
    }

    fn into_iter(self) -> impl Iterator<Item = bool> {
        self.0.into_iter().flat_map(|row| row.into_iter())
    }

    fn into_indexed_iter(self) -> impl Iterator<Item = ((usize, usize), bool)> {
        let len = self.0.len();

        self.0
            .into_iter()
            .enumerate()
            .skip(1)
            .take(len - 2)
            .flat_map(move |(i, row)| {
                row.into_iter()
                    .enumerate()
                    .skip(1)
                    .take(len - 2)
                    .map(move |(j, visibility)| ((i, j), visibility))
            })
    }
}

type Parsed = Forest;

type Part1 = usize;
type Part2 = usize;

fn parse_input(input: &str) -> Parsed {
    let forest = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Forest(forest)
}

fn part_1(parsed: &Parsed) -> Part1 {
    VisibilityMap::from_forest(parsed)
        .into_iter()
        .filter(|x| *x)
        .count()
}

fn part_2(parsed: &Parsed) -> Part2 {
    VisibilityMap::from_forest(parsed)
        .into_indexed_iter()
        .filter(|(_, x)| *x)
        .map(|((i, j), _)| parsed.scenic_score(i, j))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
30373
25512
65332
33549
35390
    "#;

    const PART_1_TEST_ANS: Part1 = 21;
    const PART_2_TEST_ANS: Part2 = 8;

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
