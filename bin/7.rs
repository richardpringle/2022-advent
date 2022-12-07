use std::{collections::HashMap, fs::read_to_string, iter::once};

const PROBLEM: u8 = 7;

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

#[derive(Debug, PartialEq, Eq)]
enum File {
    Dir(HashMap<String, File>),
    File(usize),
}

impl File {
    fn new_file(size: usize) -> Self {
        Self::File(size)
    }

    fn new_dir() -> Self {
        Self::Dir(Default::default())
    }

    fn insert(&mut self, name: String, file: Self) {
        if let Self::Dir(contents) = self {
            contents.insert(name, file);
        }
    }

    fn remove(&mut self, name: &str) -> Option<(String, File)> {
        match self {
            File::Dir(map) => map.remove_entry(name),
            _ => None,
        }
    }

    fn size(&self) -> usize {
        match self {
            File::File(size) => *size,
            File::Dir(files) => files.values().map(|file| file.size()).sum(),
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        match self {
            File::Dir(contents) => {
                let children = contents
                    .into_iter()
                    .map(|(_, file)| file)
                    .flat_map(|child| child.iter());

                Box::new(once(self).chain(children))
            }
            _ => Box::new(once(self)),
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(..))
    }
}

type Parsed = File;

type Part1 = usize;
type Part2 = usize;

struct WorkingDirectory(Vec<(String, File)>);

impl WorkingDirectory {
    fn new(name: String, file: File) -> Self {
        Self(vec![(name, file)])
    }

    fn back(&mut self) {
        if self.0.len() == 1 {
            return;
        }

        let (name, file) = self.0.pop().unwrap();

        self.0
            .last_mut()
            .map(|(_, dir)| dir)
            .unwrap()
            .insert(name.to_string(), file);
    }

    fn enter(&mut self, name: &str) {
        let child = self.0.last_mut().unwrap().1.remove(name).unwrap();
        self.0.push(child);
    }

    fn insert(&mut self, name: String, file: File) {
        let last = self.0.last_mut().map(|file| &mut file.1).unwrap();
        last.insert(name, file);
    }

    fn into_file(mut self) -> File {
        while self.0.len() > 1 {
            self.back();
        }

        self.0.pop().unwrap().1
    }
}

fn parse_input(input: &str) -> Parsed {
    let mut lines = input.trim().lines();
    lines.next(); // skip the first line, it's just creating the initial directory;

    let file = File::new_dir();

    let mut current_dir = WorkingDirectory::new(String::from("/"), file);

    let mut next = lines.next();

    while let Some(line) = next {
        match &line[0..4] {
            "$ cd" => {
                let name = line[5..].to_string();

                match name.as_str() {
                    ".." => current_dir.back(),
                    name => current_dir.enter(name),
                };

                next = lines.next();
            }

            "$ ls" => {
                next = loop {
                    match lines.next() {
                        Some(line) if !line.starts_with('$') => {
                            let (size, name) = line.split_once(' ').unwrap();

                            let child = if size == "dir" {
                                File::new_dir()
                            } else {
                                File::new_file(size.parse().unwrap())
                            };

                            current_dir.insert(name.to_string(), child);
                        }
                        line => break line,
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    current_dir.into_file()
}

fn part_1(parsed: &Parsed) -> Part1 {
    const MAX_SIZE: usize = 100000;

    parsed
        .iter()
        .filter_map(|file| {
            if file.is_dir() {
                Some(file.size()).filter(|size| *size <= MAX_SIZE)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(parsed: &Parsed) -> Part2 {
    const TOTAL_SPACE: usize = 70000000;
    const UPDATE_SIZE: usize = 30000000;

    let space_needed = UPDATE_SIZE - (TOTAL_SPACE - parsed.size());

    parsed
        .iter()
        .filter_map(|file| {
            if file.is_dir() {
                Some(file.size()).filter(|size| *size >= space_needed)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{Parsed, Part1, Part2};

    const INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    "#;

    const PART_1_TEST_ANS: Part1 = 95437;
    const PART_2_TEST_ANS: Part2 = 24933642;

    fn parsed_input() -> Parsed {
        let mut d = Parsed::new_dir();
        d.insert("j".into(), Parsed::new_file(4060174));
        d.insert("d.log".into(), Parsed::new_file(8033020));
        d.insert("d.ext".into(), Parsed::new_file(5626152));
        d.insert("k".into(), Parsed::new_file(7214296));

        let mut e = Parsed::new_dir();
        e.insert("i".into(), Parsed::new_file(584));
        let mut a = Parsed::new_dir();
        a.insert("e".into(), e);
        a.insert("f".into(), Parsed::new_file(29116));
        a.insert("g".into(), Parsed::new_file(2557));
        a.insert("h.lst".into(), Parsed::new_file(62596));

        let mut parsed = Parsed::new_dir();

        parsed.insert("a".into(), a);
        parsed.insert("b.txt".into(), Parsed::new_file(14848514));
        parsed.insert("c.dat".into(), Parsed::new_file(8504156));
        parsed.insert("d".into(), d);

        parsed
    }

    #[test]
    fn parse() {
        assert_eq!(super::parse_input(INPUT), parsed_input());
    }

    #[test]
    fn part_1() {
        let part_1_ans = super::part_1(&parsed_input());

        assert_eq!(part_1_ans, PART_1_TEST_ANS);
    }

    #[test]
    fn part_2() {
        let part_2_ans = super::part_2(&parsed_input());

        assert_eq!(part_2_ans, PART_2_TEST_ANS);
    }
}
