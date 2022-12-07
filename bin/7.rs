use std::{cell::RefCell, fs::read_to_string, iter::once, rc::Rc};

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
    Dir(String, Vec<File>),
    File(usize),
}

impl File {
    fn new_file(size: usize) -> Self {
        Self::File(size)
    }

    fn new_dir(name: String) -> Self {
        Self::Dir(name, vec![])
    }

    fn insert(&mut self, file: Self) {
        if let Self::Dir(_, contents) = self {
            contents.push(file)
        }
    }

    fn size(&self) -> usize {
        match self {
            File::File(size) => *size,
            File::Dir(_, files) => files.iter().map(|file| file.size()).sum(),
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        match self {
            File::Dir(_, contents) => {
                let children = contents.into_iter().flat_map(|child| child.iter());

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

#[derive(Debug)]
enum TreeNode {
    Leaf(File),
    Node {
        name: String,
        children: Vec<Rc<RefCell<TreeNode>>>,
    },
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Node {
            name: Default::default(),
            children: Default::default(),
        }
    }
}

impl TreeNode {
    fn new(name: String) -> Self {
        Self::Node {
            name,
            children: vec![],
        }
    }

    fn from_file(file: File) -> Self {
        Self::Leaf(file)
    }

    fn name(&self) -> &str {
        match self {
            TreeNode::Node { name, .. } => name.as_str(),
            _ => "",
        }
    }

    fn children(&self) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> + '_ {
        match self {
            TreeNode::Node { children, .. } => children.into_iter().cloned(),
            _ => (&[]).into_iter().cloned(),
        }
    }

    fn insert_child(&mut self, node: TreeNode) {
        match self {
            TreeNode::Node { children, .. } => children.push(Rc::new(RefCell::new(node))),
            _ => (),
        }
    }

    fn into_file(&mut self) -> Option<File> {
        let this = std::mem::take(self);

        match this {
            TreeNode::Leaf(file) => file.into(),
            TreeNode::Node { name, children } => children
                .into_iter()
                .flat_map(|file| {
                    let mut borrowed = file.borrow_mut();
                    borrowed.into_file()
                })
                .fold(File::new_dir(name), |mut dir, file| {
                    dir.insert(file);
                    dir
                })
                .into(),
        }
    }
}

fn parse_input(input: &str) -> Parsed {
    let mut lines = input.trim().lines();
    lines.next(); // skip the first line, it's just creating the initial directory;

    let tree = Rc::new(RefCell::new(TreeNode::new("/".into())));

    let mut current_dir = vec![tree.clone()];

    let mut next = lines.next();

    while let Some(line) = next {
        match &line[0..4] {
            "$ cd" => {
                let name = line[5..].to_string();

                match name.as_str() {
                    ".." => {
                        current_dir.pop();
                    }
                    name => {
                        let child = {
                            let borrowed = current_dir.last().unwrap().borrow();

                            let child = borrowed
                                .children()
                                .find(|child| {
                                    let borrowed = child.borrow();
                                    borrowed.name() == name
                                })
                                .unwrap()
                                .clone();

                            child
                        };

                        current_dir.push(child)
                    }
                };

                next = lines.next();
            }

            "$ ls" => {
                next = loop {
                    match lines.next() {
                        Some(s) if s.starts_with('$') => break Some(s),
                        Some(line) => {
                            let (size, name) = line.split_once(' ').unwrap();

                            let child = if size == "dir" {
                                TreeNode::new(name.to_string())
                            } else {
                                let file = File::new_file(size.parse().unwrap());
                                TreeNode::from_file(file)
                            };

                            let mut borrowed = current_dir.last().unwrap().borrow_mut();
                            borrowed.insert_child(child);
                        }
                        None => break None,
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut borrowed = tree.borrow_mut();
    let file = borrowed.into_file();
    file.unwrap()
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
        let mut d = Parsed::new_dir("d".into());
        d.insert(Parsed::new_file(4060174));
        d.insert(Parsed::new_file(8033020));
        d.insert(Parsed::new_file(5626152));
        d.insert(Parsed::new_file(7214296));

        let mut e = Parsed::new_dir("e".into());
        e.insert(Parsed::new_file(584));
        let mut a = Parsed::new_dir("a".into());
        a.insert(e);
        a.insert(Parsed::new_file(29116));
        a.insert(Parsed::new_file(2557));
        a.insert(Parsed::new_file(62596));

        let mut parsed = Parsed::new_dir("/".into());

        parsed.insert(a);
        parsed.insert(Parsed::new_file(14848514));
        parsed.insert(Parsed::new_file(8504156));
        parsed.insert(d);

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
