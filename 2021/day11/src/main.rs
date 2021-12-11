use anyhow::{Error, Result};
use hashbrown::HashSet;

use std::fmt;
use std::str::FromStr;

use std::collections::VecDeque;

type Points = Vec<Point>;
type Point = (usize, usize);
type Flashed = HashSet<Point>;

#[derive(Debug)]
struct Grid {
    dumbos: Vec<usize>,
    height: usize,
    width: usize,
    flashes: usize,
}

impl Grid {
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn is_valid_index(&self, x: i64, y: i64) -> bool {
        0 <= x && x < self.width as i64 && 0 <= y && y < self.height as i64
    }

    fn find_first_simultaneous_flash(&mut self) {
        let mut count = 0;
        let mut flashed = 0;
        while flashed != self.dumbos.len() {
            count += 1;
            flashed = self.step2();
        }

        println!("{}", count);
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Points {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .copied()
        .map(|(d_x, d_y)| {
            (
                d_x + i64::try_from(x).unwrap(),
                d_y + i64::try_from(y).unwrap(),
            )
        })
        .filter(|(d_x, d_y)| self.is_valid_index(*d_x, *d_y))
        .map(|(d_x, d_y)| (usize::try_from(d_x).unwrap(), usize::try_from(d_y).unwrap()))
        .collect()
    }

    fn step(&mut self) {
        self.increment();
        self.propagate_flashes();
        self.set_to_zero()
    }

    fn step2(&mut self) -> usize {
        self.increment();
        let count = self.propagate_flashes_with_count();
        self.set_to_zero();
        count
    }

    fn increment(&mut self) {
        for val in self.dumbos.iter_mut() {
            *val += 1;
        }
    }

    fn set_to_zero(&mut self) {
        for val in self.dumbos.iter_mut() {
            if *val > 9 {
                *val = 0;
            }
        }
    }

    fn propagate_flashes(&mut self) {
        let mut ready_to_flash: Vec<Point> = vec![];

        for y in 0..self.height {
            for x in 0..self.height {
                let index = self.get_index(x, y);
                if self.dumbos[index] > 9 {
                    ready_to_flash.push((x, y));
                }
            }
        }

        let mut flashed: Flashed = HashSet::new();

        ready_to_flash
            .iter()
            .for_each(|point| self.propagate_flash(*point, &mut flashed));
    }

    fn propagate_flashes_with_count(&mut self) -> usize {
        let mut ready_to_flash: Vec<Point> = vec![];

        for y in 0..self.height {
            for x in 0..self.height {
                let index = self.get_index(x, y);
                if self.dumbos[index] > 9 {
                    ready_to_flash.push((x, y));
                }
            }
        }

        let mut flashed: Flashed = HashSet::new();

        ready_to_flash
            .iter()
            .for_each(|point| self.propagate_flash(*point, &mut flashed));

        flashed.len()
    }

    fn propagate_flash(&mut self, point: Point, flashed: &mut Flashed) {
        // println!("{}", self);
        let mut queue = VecDeque::from([point]);

        while !queue.is_empty() {
            let curr_point = queue.pop_front().unwrap();
            if !flashed.contains(&curr_point) {
                self.flashes += 1;
                flashed.insert(curr_point);
                let neighbors = self.get_neighbors(curr_point.0, curr_point.1);
                for neighbor in neighbors {
                    let (x, y) = neighbor;
                    let index = self.get_index(x, y);
                    self.dumbos[index] += 1;
                    if self.dumbos[index] > 9 {
                        queue.push_back((x, y));
                    }
                }
            }
        }
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Grid> {
        let height = s.lines().count();
        let dumbos: Vec<usize> = s
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|x| !x.is_empty())
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
            })
            .flatten()
            .collect();

        let width = dumbos.len() / height;

        Ok(Grid {
            height,
            width,
            dumbos,
            flashes: 0,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                write!(f, "{} ", &self.dumbos[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = Grid::from_str(input).unwrap();
    part2(&mut grid);
}

fn part1(grid: &mut Grid) {
    for _ in 0..194 {
        grid.step();
    }
    println!("{}", grid);
    println!("{}", grid.flashes);
}

fn part2(grid: &mut Grid) {
    grid.find_first_simultaneous_flash();
}
