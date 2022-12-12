use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Pair {
    left: Half,
    right: Half,
    depth: usize,
    parent: Box<Pair>,
}

impl Pair {
    fn split(&self, side: Half) -> Half {
        match side {
            Half::Literal(_) => side.split(self.depth, self.parent.clone()),
            _ => side,
        }
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cursor = 0;
        while s.chars().nth(cursor).unwrap() != '[' {
            cursor += 1;
            let left = Half::Literal(
                s.chars()
                    .nth(cursor)
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
            );
            cursor += 2;
        }

        Ok(());
    }
}

#[derive(Debug, Clone)]
enum Half {
    Literal(usize),
    Pair(Box<Pair>),
}

impl Half {
    fn split(&self, depth: usize, parent: Box<Pair>) -> Self {
        match self {
            Half::Literal(n) => {
                let left = n / 2;
                let right = (n + (2 - 1)) / 2;
                Half::Pair(Box::new(Pair {
                    left: Half::Literal(left),
                    right: Half::Literal(right),
                    depth,
                    parent,
                }))
            }
            _ => self.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
