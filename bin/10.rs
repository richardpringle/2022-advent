use std::{fmt::Debug, fs::read_to_string, ops::Deref, str::FromStr};

const PROBLEM: u8 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1(&parsed));
    println!("part-2: {:?}", part_2(&parsed));

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Cmd {
    Noop,
    Addx(isize),
}

impl Cmd {
    fn apply(self, x: isize) -> isize {
        match self {
            Cmd::Noop => x,
            Cmd::Addx(y) => x + y,
        }
    }
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.starts_with("noop") => Ok(Self::Noop),
            s => match s.strip_prefix("addx ") {
                Some(s) => s.parse().map_or(Err(()), |num| Ok(Self::Addx(num))),
                None => Err(()),
            },
        }
    }
}

#[derive(Debug, Default)]
struct CycleCount(u8);

impl CycleCount {
    fn is_done(&self) -> bool {
        self.0 == 0
    }

    fn decr(&mut self) {
        self.0 = self.0.checked_sub(1).unwrap_or_default();
    }
}

impl Deref for CycleCount {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Cmd> for CycleCount {
    fn from(cmd: Cmd) -> Self {
        match cmd {
            Cmd::Noop => Self(1),
            Cmd::Addx(_) => Self(2),
        }
    }
}

struct Cpu<T> {
    cmds: T,
    x: isize,
}

impl<T: Debug> Debug for Cpu<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("cmds", &self.cmds)
            .field("x", &self.x)
            .finish()
    }
}

impl Cpu<Vec<Cmd>> {
    fn new() -> Self {
        Self { cmds: vec![], x: 1 }
    }

    fn load(&mut self, cmds: Vec<Cmd>) {
        self.cmds = cmds;
    }

    fn into_iter(self) -> CpuIter<impl Iterator<Item = Cmd> + Debug> {
        let mut cmds = self.cmds.into_iter();
        let cmd = cmds.next();

        CpuIter {
            cpu: Cpu {
                cmds,
                x: self.x,
            },
            cmd,
            counter: cmd.map(CycleCount::from).unwrap_or_default(),
        }
    }
}

struct CpuIter<T> {
    cpu: Cpu<T>,
    cmd: Option<Cmd>,
    counter: CycleCount,
}

impl<T: Debug> Debug for CpuIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuIter")
            .field("cpu", &self.cpu)
            .field("cmd", &self.cmd)
            .field("counter", &self.counter)
            .finish()
    }
}

impl<T: Iterator<Item = Cmd>> Iterator for CpuIter<T> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.is_done() {
            let cmd = self.cmd.take();

            self.cmd = self.cpu.cmds.next();
            self.counter = self.cmd.map(CycleCount::from).unwrap_or_default();
            self.counter.decr();

            cmd.map(|cmd| {
                self.cpu.x = cmd.apply(self.cpu.x);
                self.cpu.x
            })
        } else {
            self.counter.decr();
            Some(self.cpu.x)
        }
    }
}

struct Printer {
    cycle: isize,
}

impl Printer {
    fn new() -> Self {
        Self { cycle: 0 }
    }

    fn print(&mut self, x: isize) {
        let pos = self.cycle % 40;

        let c = if pos.abs_diff(x) <= 1 { '#' } else { '.' };

        if pos == 39 {
            println!("{c}");
        } else {
            print!("{c}");
        }

        self.cycle += 1;
    }
}

type Parsed = Vec<Cmd>;

type Part1 = isize;
type Part2 = ();

fn parse_input(input: &str) -> Parsed {
    input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Cmd>, ()>>()
        .unwrap()
}

fn part_1(parsed: &Parsed) -> Part1 {
    let mut cpu = Cpu::new();

    cpu.load(parsed.clone());
    cpu.into_iter()
        .enumerate()
        .map(|(a, b)| (a + 1, b))
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(a, b)| a as isize * b)
        .sum()
}

fn part_2(parsed: &Parsed) -> Part2 {
    let mut cpu = Cpu::new();
    let mut printer = Printer::new();

    cpu.load(parsed.clone());
    cpu.into_iter().take(240).for_each(|x| printer.print(x))
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    "#;

    const PART_1_TEST_ANS: Part1 = 13140;
    const PART_2_TEST_ANS: Part2 = ();

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
