use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

#[derive(Debug, Clone)]
struct Room {
    seats: Vec<Vec<Seat>>,
    width: isize,
    height: isize,
    dirs: Vec<(isize, isize)>,
}

impl Room {
    fn new(seats: Vec<Vec<Seat>>) -> Room {
        Room {
            width: seats[0].len() as isize,
            height: seats.len() as isize,
            seats,
            dirs: vec![
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ],
        }
    }

    fn play(&mut self) -> usize {
        let mut same = false;

        while !same {
            let mut next = self.seats.clone();

            for row in 0..self.height {
                for col in 0..self.width {
                    let neighbors = self.count_neighbors2(row, col);
                    next[row as usize][col as usize] =
                        match (self.seats[row as usize][col as usize], neighbors) {
                            (Seat::Empty, 0) => Seat::Occupied,
                            (Seat::Occupied, 5..=std::i32::MAX) => Seat::Empty,
                            (seat, _) => seat,
                        };
                }
            }
            same = next == self.seats;
            self.seats = next;
        }

        self.seats
            .iter()
            .flatten()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }

    fn count_neighbors1(&self, row: isize, col: isize) -> i32 {
        let mut count = 0;
        for (r, c) in &self.dirs {
            let (new_row, new_col) = (row + r, col + c);
            if 0 <= new_row && new_row < self.height && 0 <= new_col && new_col < self.width {
                if self.seats[new_row as usize][new_col as usize] == Seat::Occupied {
                    count += 1
                }
            }
        }
        count
    }

    fn count_neighbors2(&self, row: isize, col: isize) -> i32 {
        let mut count = 0;

        for (r, c) in &self.dirs {
            let mut new_row = row;
            let mut new_col = col;
            let mut found = false;
            while !found {
                new_row += r;
                new_col += c;
                if 0 <= new_row && new_row < self.height && 0 <= new_col && new_col < self.width {
                    match self.seats[new_row as usize][new_col as usize] {
                        Seat::Occupied => {
                            count += 1;
                            found = true;
                        }
                        Seat::Empty => found = true,
                        Seat::Floor => continue,
                    }
                } else {
                    break;
                }
            }
        }
        count
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.seats {
            for cell in line {
                let symbol = match cell {
                    Seat::Empty => "L",
                    Seat::Occupied => "#",
                    Seat::Floor => ".",
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let seats = read_file("input.txt");
    let mut room = Room::new(seats);
    part1(&mut room);
}

fn part1(room: &mut Room) {
    let result = room.play();
    println!("{}", result);
}

fn read_file(filename: &str) -> Vec<Vec<Seat>> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "L.LL.LL.LL
    // LLLLLLL.LL
    // L.L.L..L..
    // LLLL.LL.LL
    // L.LL.LL.LL
    // L.LLLLL.LL
    // ..L.L.....
    // LLLLLLLLLL
    // L.LLLLLL.L
    // L.LLLLL.LL";

    contents
        .lines()
        .map(|line| {
            line.trim()
                .to_string()
                .split("")
                .filter(|char| *char != "")
                .map(|char| match char {
                    "." => Seat::Floor,
                    "#" => Seat::Occupied,
                    "L" => Seat::Empty,
                    _ => panic!("the heck"),
                })
                .collect()
        })
        .collect()
}
