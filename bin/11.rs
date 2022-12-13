use std::{
    cell::RefCell,
    cmp::Ordering,
    fs::read_to_string,
    num::ParseIntError,
    str::{FromStr, Lines},
};

const PROBLEM: u8 = 11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1(&parsed));
    // println!("part-2: {:?}", part_2(&parsed));

    Ok(())
}

type Parsed = Monkeys;
type Part1 = usize;
type Part2 = usize;

type Items = Vec<Item>;
type Monkeys = Vec<RefCell<Monkey>>;

#[derive(Clone, Debug)]
struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

impl Monkey {
    fn new(items: Items, operation: Operation, test: Test) -> Self {
        Self {
            items,
            operation,
            test,
            inspection_count: 0,
        }
    }

    fn inspect_and_throw_items<const N: usize>(&mut self, monkeys: &Monkeys) {
        let items = std::mem::take(&mut self.items).into_iter();

        let items = items
            .map(|item| self.operation.operate(item))
            .map(|item| Item(item.0 / N));

        for item in items {
            self.inspection_count += 1;
            let index = self.test(item);
            let mut recipient = monkeys[index].borrow_mut();
            recipient.receive(item);
        }
    }

    fn test(&self, item: Item) -> usize {
        self.test.test(item)
    }

    fn receive(&mut self, item: Item) {
        self.items.push(item)
    }
}

#[derive(Clone, Debug)]
enum Operation {
    SelfMul,
    SelfAdd,
    Mul(usize),
    Add(usize),
}

impl Operation {
    fn operate(&self, item: Item) -> Item {
        match self {
            Operation::SelfMul => Item(item.0 * item.0),
            Operation::SelfAdd => Item(item.0 + item.0),
            Operation::Mul(other) => Item(item.0 * other),
            Operation::Add(other) => Item(item.0 + other),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.split(' ');

        let result = match chars.next() {
            Some("+") => match chars.next().unwrap().parse() {
                Ok(other) => Self::Add(other),
                _ => Self::SelfAdd,
            },
            _ => match chars.next().unwrap().parse() {
                Ok(other) => Self::Mul(other),
                _ => Self::SelfMul,
            },
        };

        Ok(result)
    }
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divisor: usize,
    t: usize,
    f: usize,
}

impl Test {
    fn test(&self, item: Item) -> usize {
        if item.0 % self.divisor == 0 {
            self.t
        } else {
            self.f
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Item(usize);

impl FromStr for Item {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug)]
struct TwoLargest([usize; 2]);

impl TwoLargest {
    fn new() -> Self {
        Self([0; 2])
    }

    fn insert(&mut self, val: usize) -> bool {
        match (self.0[0].cmp(&val), self.0[1].cmp(&val)) {
            (Ordering::Less, Ordering::Less) => {
                self.0[0] = self.0[1];
                self.0[1] = val;
                true
            }
            (Ordering::Less, _) => {
                self.0[0] = val;
                true
            }
            _ => false,
        }
    }
}

impl From<TwoLargest> for usize {
    fn from(two: TwoLargest) -> Self {
        two.0.into_iter().product()
    }
}

fn parse_monkey(mut monkeys: Monkeys, mut lines: Lines) -> Monkeys {
    match lines.next() {
        Some(line) if line.starts_with("Monkey ") => {
            let items = parse_items(lines.next().unwrap());
            let operation = parse_operation(lines.next().unwrap());
            let (lines, test) = parse_test(lines);

            monkeys.push(RefCell::new(Monkey::new(items, operation, test)));

            parse_monkey(monkeys, lines)
        }
        Some(_) => parse_monkey(monkeys, lines),
        None => monkeys,
    }
}

fn parse_items(s: &str) -> Items {
    s.trim()
        .strip_prefix("Starting items: ")
        .and_then(|s| {
            s.split(", ")
                .map(FromStr::from_str)
                .collect::<Result<Items, _>>()
                .ok()
        })
        .unwrap_or_default()
}

fn parse_operation(s: &str) -> Operation {
    s.trim()
        .strip_prefix("Operation: new = old ")
        .and_then(|s| s.parse().ok())
        .unwrap()
}

fn parse_test(mut lines: Lines) -> (Lines, Test) {
    let mut values = [lines.next(), lines.next(), lines.next()]
        .into_iter()
        .flatten()
        .flat_map(|line| line.split(' ').filter_map(|val| usize::from_str(val).ok()));

    let divisor = values.next().unwrap();
    let t = values.next().unwrap();
    let f = values.next().unwrap();

    (lines, Test { divisor, t, f })
}

fn parse_input(input: &str) -> Parsed {
    parse_monkey(vec![], input.trim().lines())
}

fn part_1(parsed: &Parsed) -> Part1 {
    let monkeys = parsed.clone();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            monkey.inspect_and_throw_items::<3>(&monkeys);
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| dbg!(monkey.borrow().inspection_count))
        .fold(TwoLargest::new(), |mut largest, count| {
            largest.insert(count);
            dbg!(largest)
        })
        .into()
}

fn part_2(parsed: &Parsed) -> Part2 {
    let monkeys = parsed.clone();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            monkey.inspect_and_throw_items::<1>(&monkeys);
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| dbg!(monkey.borrow().inspection_count))
        .fold(TwoLargest::new(), |mut largest, count| {
            largest.insert(count);
            dbg!(largest)
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#;

    const PART_1_TEST_ANS: Part1 = 10605;
    const PART_2_TEST_ANS: Part2 = 2713310158;

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
