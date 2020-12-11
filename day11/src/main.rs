use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

struct Room {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl Room {
    fn new(width: usize, height: usize, seats: Vec<Seat>) -> Room {
        Room {
            width,
            height,
            seats,
        }
    }

    fn get_index(&self, row: isize, col: isize) -> isize {
        row * (self.width as isize) + col
    }

    fn neighbor_count(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        let deltas: Vec<isize> = vec![-1, 0, 1];
        for delta_row in deltas.iter().cloned() {
            for delta_col in deltas.iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = row as isize + delta_row;
                let neighbor_col = col as isize + delta_col;
                let idx = self.get_index(neighbor_row, neighbor_col);
                // dbg!(idx);
                // don't do this. this is bad.
                if idx < (neighbor_row * self.width as isize) + self.width as isize
                    && neighbor_col >= 0
                    && 0 <= idx
                    && idx < (self.seats.len() as isize)
                    && self.seats[idx as usize] == Seat::Occupied
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn tick(&mut self) -> bool {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row as isize, col as isize);
                let seat = self.seats[idx as usize];
                let neighbors = self.neighbor_count(row, col);
                // dbg!(idx, neighbors);

                let next_seat = match (seat, neighbors) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, 4..=std::usize::MAX) => Seat::Empty, // lol
                    (seat, _) => seat,
                };

                next[idx as usize] = next_seat;
            }
        }
        // println!("{:?}\n\n{:?}", next, self.seats);
        if compare_vecs(&next, &self.seats) {
            println!("{}", self.count_occupied_seats());
            return true;
        }
        self.seats = next;
        false
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .filter(|&seat| *seat == Seat::Occupied)
            .count()
    }
}

fn compare_vecs(a: &[Seat], b: &[Seat]) -> bool {
    a.iter().zip(b).all(|(a, b)| *a == *b)
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.seats.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    Seat::Empty => "L",
                    Seat::Occupied => "#",
                    Seat::Floor => ".",
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let (seats, width, height) = read_file("input.txt");
    let mut room = Room::new(width, height, seats);
    part1(&mut room);
}

fn part1(room: &mut Room) {
    let mut stable = false;

    while !stable {
        println!("{}", room);
        stable = room.tick();
    }
}

fn read_file(filename: &str) -> (Vec<Seat>, usize, usize) {
    let contents = fs::read_to_string(filename).unwrap();
    //     let contents = "L.LL.LL.LL
    // LLLLLLL.LL
    // L.L.L..L..
    // LLLL.LL.LL
    // L.LL.LL.LL
    // L.LLLLL.LL
    // ..L.L.....
    // LLLLLLLLLL
    // L.LLLLLL.L
    // L.LLLLL.LL";
    let width = contents.find('\n').unwrap();
    let height = contents.len() / width - 1;
    println!("{} {}", width, height);
    let contents = contents.replace("\n", "");
    (
        contents
            .chars()
            .map(|char| match char {
                '.' => Seat::Floor,
                '#' => Seat::Occupied,
                'L' => Seat::Empty,
                _ => panic!("whoa now, {}", char),
            })
            .collect(),
        width,
        height,
    )
}
