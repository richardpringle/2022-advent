use reqwest::{header::COOKIE, Client};
use std::str::FromStr;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_number = std::env::args()
        .next_back()
        .as_ref()
        .map(String::as_str)
        .map(u8::from_str)
        .map(Result::ok)
        .flatten()
        .expect("Must provide problem number to fetch input");

    let cookie = fs::read_to_string(".cookie").await?;

    let url = format!("https://adventofcode.com/2022/day/{problem_number}/input");
    let res = Client::new().get(url).header(COOKIE, cookie).send().await?;

    fs::write(format!("./inputs/{problem_number}.txt"), res.text().await?).await?;

    Ok(())
}
