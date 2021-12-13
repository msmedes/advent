use anyhow::{Error, Result};
use hashbrown::HashSet;

use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    value: i64,
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let splits: Vec<&str> = s.split('=').collect();
        let axis = match splits[0] {
            "y" => Axis::Y,
            "x" => Axis::X,
            _ => panic!("invalid axis value"),
        };
        let value = splits[1].parse::<i64>().unwrap();
        Ok(Fold { axis, value })
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values: Vec<i64> = s
            .split(',')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect();

        Ok(Point {
            x: values[0],
            y: values[1],
        })
    }
}

#[derive(Debug)]
struct Manual {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

impl Manual {
    fn apply_folds(&mut self) {
        for fold_index in 0..self.folds.len() {
            self.apply_fold(fold_index);
        }
    }

    fn apply_fold(&mut self, index: usize) {
        let fold = &self.folds[index];
        let new_points = self
            .points
            .iter()
            .map(|point| match fold.axis {
                Axis::X => {
                    if point.x > fold.value {
                        Point {
                            x: point.x - (2 * (point.x - fold.value)),
                            y: point.y,
                        }
                    } else {
                        *point
                    }
                }
                Axis::Y => {
                    if point.y > fold.value {
                        Point {
                            x: point.x,
                            y: point.y - (2 * (point.y - fold.value)),
                        }
                    } else {
                        *point
                    }
                }
            })
            .collect();

        self.points = new_points;
    }

    fn count_dots(&self) {
        let num_dots = self.points.len();
        dbg!(num_dots);
    }
}

impl fmt::Display for Manual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_y = self.points.iter().map(|point| point.y).min().unwrap();
        let max_y = self.points.iter().map(|point| point.y).max().unwrap() + 1;
        let min_x = self.points.iter().map(|point| point.x).min().unwrap();
        let max_x = self.points.iter().map(|point| point.x).max().unwrap() + 1;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let dot = match self.points.contains(&Point { x, y }) {
                    true => "#",
                    false => ".",
                };
                write!(f, "{}", dot)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Manual {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let input: Vec<&str> = s.split("\n\n").collect();
        let points = input[0]
            .lines()
            .map(Point::from_str)
            .map(Result::unwrap)
            .collect();
        let folds = input[1]
            .lines()
            .map(Fold::from_str)
            .map(Result::unwrap)
            .collect();
        Ok(Self { points, folds })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut manual = input.parse::<Manual>().unwrap();
    part2(&mut manual);
}

fn part1(manual: &mut Manual) {
    manual.apply_fold(0);
    manual.count_dots();
}

fn part2(manual: &mut Manual) {
    manual.apply_folds();
    println!("{}", manual);
}
