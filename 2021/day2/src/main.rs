use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    delta: i64,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<direction>\D*) (?P<delta>\d*)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let direction = caps[1].parse::<Direction>()?;
        let delta = caps[2].parse::<i64>()?;

        Ok(Self { direction, delta })
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _idk => panic!("This didn't work {}", _idk),
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect()
}

fn execute_instructions(instructions: &[Instruction]) -> i64 {
    let (mut x, mut y) = (0, 0);
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => x += instruction.delta,
            Direction::Up => y -= instruction.delta,
            Direction::Down => y += instruction.delta,
        }
    }
    x * y
}

fn execute_instructions_with_aim(instructions: &[Instruction]) -> i64 {
    let (mut position, mut depth, mut aim) = (0, 0, 0);
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                position += instruction.delta;
                depth += aim * instruction.delta;
            }
            Direction::Up => aim -= instruction.delta,
            Direction::Down => aim += instruction.delta,
        }
    }
    position * depth
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let instructions = get_instructions(input);
    let product = execute_instructions_with_aim(&instructions);
    dbg!(product);
    Ok(())
}
