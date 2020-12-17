use anyhow::Result;

use std::collections::{HashMap, HashSet};
use std::fmt;
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
    // println!("{}", space);
    // println!("{:#?}", space.grid);
    for _ in 0..6 {
        space.tick();
        // println!("<----->");
        // println!("{}", space);
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

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut zs: HashMap<isize, Vec<(Coord, State)>> = HashMap::new();
        for coord in self.grid.keys() {
            zs.entry(coord.z)
                .and_modify(|set| {
                    let _ = set.push((*coord, *self.grid.get(&coord).unwrap()));
                })
                .or_insert_with(|| vec![(*coord, *self.grid.get(&coord).unwrap())]);
        }
        let mut z_indices: Vec<isize> = zs.keys().copied().collect();
        z_indices.sort_unstable();

        for z in z_indices {
            writeln!(f, "z={}", z);
            let coords = zs.get(&z).unwrap().clone();
            let mut ys: HashMap<isize, Vec<(Coord, State)>> = HashMap::new();
            for coord in coords {
                ys.entry(coord.0.y)
                    .and_modify(|set| set.push(coord))
                    .or_insert_with(|| vec![coord]);
            }
            let mut y_indices: Vec<isize> = ys.keys().copied().collect();
            y_indices.sort_unstable();
            // coords.sort_by(|a, b| a.0.y.cmp(&b.0.y).then_with(|| a.0.x.cmp(&b.0.x)));
            for y in y_indices {
                let mut xs = ys.get(&y).unwrap().clone();
                xs.sort_by(|a, b| a.0.x.cmp(&b.0.x));
                for x in xs {
                    let symbol = match x.1 {
                        State::Active => "#",
                        State::Inactive => ".",
                    };
                    write!(f, "{}", symbol);
                }
                writeln!(f);
            }
            writeln!(f);
        }
        Ok(())
    }
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Space> {
        let mut grid = HashMap::<Coord, State>::new();
        dbg!(s);
        for (y, line) in s.split('\n').enumerate() {
            let line = line.trim();
            dbg!(line);
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => {
                        let _ = grid.insert(
                            Coord {
                                x: x as isize,
                                y: y as isize,
                                z: 0,
                                w: 0,
                            },
                            State::Inactive,
                        );
                    }
                    '#' => {
                        let _ = grid.insert(
                            Coord {
                                x: x as isize,
                                y: y as isize,
                                z: 0,
                                w: 0,
                            },
                            State::Active,
                        );
                    }
                    _ => panic!("{}", char),
                }
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
