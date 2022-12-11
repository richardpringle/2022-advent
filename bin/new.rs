use std::{fs::OpenOptions, io::Write, str::FromStr};

const TOML_TEMPLATE: &str = r#"
[[bin]]
name = "{}"
path = "bin/{}.rs"
"#;

const CODE_TEMPLATE: &str = r##"
use std::fs::read_to_string;

const PROBLEM: u8 = {};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(format!("inputs/{PROBLEM}.txt"))?;

    let parsed = parse_input(&input);

    println!("Problem {PROBLEM}");
    println!("part-1: {:?}", part_1(&parsed));
    // println!("part-2: {:?}", part_2(&parsed));

    Ok(())
}

struct Parsed;

#[derive(Debug, PartialEq)]
struct Part1;

#[derive(Debug, PartialEq)]
struct Part2;

fn parse_input(input: &str) -> Parsed {
    todo!()
}

fn part_1(parsed: &Parsed) -> Part1 {
    todo!()
}

fn part_2(parsed: &Parsed) -> Part2 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{Part1, Part2};

    const INPUT: &str = r#"
        <paste test input here>
    "#;

    const PART_1_TEST_ANS: Part1 = Part1;
    const PART_2_TEST_ANS: Part2 = Part2;

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
"##;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_number = std::env::args()
        .next_back().as_deref()
        .map(u8::from_str)
        .and_then(Result::ok)
        .expect("Must provide problem number to create file");

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("bin/{problem_number}.rs"))
        .unwrap_or_else(|_| panic!("code for problem-{problem_number} already exits"));

    file.write_all(
        CODE_TEMPLATE
            .replace("{}", &problem_number.to_string())
            .as_bytes(),
    )?;

    let mut file = OpenOptions::new().append(true).open("Cargo.toml")?;

    file.write_all(
        TOML_TEMPLATE
            .replace("{}", &problem_number.to_string())
            .as_bytes(),
    )?;

    Ok(())
}
