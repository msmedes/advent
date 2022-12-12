use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let file = fs::read_to_string("./src/inputs/day01.txt")?;
    let input: Vec<&str> = file.lines().collect();

    let tokens: Vec<&str> = input
        .iter()
        .flat_map(|&line| line.split_whitespace())
        .collect();

    println!("{:?}", tokens);

    Ok(())
}
