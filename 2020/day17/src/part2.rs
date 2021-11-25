use anyhow::Result;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = "####...#
    ......##
    ####..##
    ##......
    ..##.##.
    #.##...#
    ....##.#
    .##.#.#.";

    let mut space = contents.parse::<Space>().unwrap();
    for _ in 0..6 {
        space.tick();
    }
    println!("{}", space.count_active());
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(Debug, Clone)]
struct Space {
    grid: HashMap<Coord, State>,
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Space> {
        let mut grid = HashMap::<Coord, State>::new();
        for (y, line) in s.split('\n').enumerate() {
            let line = line.trim();
            for (x, char) in line.chars().enumerate() {
                let state = match char {
                    '.' => State::Inactive,
                    '#' => State::Active,
                    _ => panic!("{}", char),
                };
                grid.insert(
                    Coord {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        w: 0,
                    },
                    state,
                );
            }
        }
        Ok(Space { grid })
    }
}

impl Space {
    fn get(&self, coord: Coord) -> Option<&State> {
        self.grid.get(&coord)
    }

    fn count_neighbors(&self, coord: Coord) -> usize {
        let mut count = 0;
        for y in coord.y - 1..coord.y + 2 {
            for x in coord.x - 1..coord.x + 2 {
                for z in coord.z - 1..coord.z + 2 {
                    for w in coord.w - 1..coord.w + 2 {
                        if (Coord { x, y, z, w }) == coord {
                            continue;
                        }
                        match self.get(Coord { x, y, z, w }) {
                            Some(c) => {
                                if *c == State::Active {
                                    count += 1
                                }
                            }
                            _ => count += 0,
                        }
                    }
                }
            }
        }
        count
    }

    fn generate_next_dims(&mut self) {
        let mut new_coords: Vec<Coord> = vec![];
        for coord in self.grid.keys() {
            for y in coord.y - 1..coord.y + 2 {
                for x in coord.x - 1..coord.x + 2 {
                    for z in coord.z - 1..coord.z + 2 {
                        for w in coord.w - 1..coord.w + 2 {
                            let curr_coord = Coord { x, y, z, w };
                            if !self.grid.contains_key(&curr_coord) {
                                new_coords.push(curr_coord);
                            }
                        }
                    }
                }
            }
        }
        for coord in new_coords {
            self.grid.insert(coord, State::Inactive);
        }
    }

    fn tick(&mut self) {
        let mut next = HashMap::<Coord, State>::new();
        self.generate_next_dims();
        for coord in self.grid.keys() {
            let neighbors_count = self.count_neighbors(*coord);
            let state = self.grid.get(coord).unwrap();
            let new_state = match state {
                State::Active => {
                    if neighbors_count == 2 || neighbors_count == 3 {
                        State::Active
                    } else {
                        State::Inactive
                    }
                }
                State::Inactive => {
                    if neighbors_count == 3 {
                        State::Active
                    } else {
                        State::Inactive
                    }
                }
            };
            next.insert(*coord, new_state);
        }
        self.grid = next;
    }

    fn count_active(&self) -> usize {
        self.grid
            .iter()
            .filter(|(_, state)| **state == State::Active)
            .count()
    }
}
