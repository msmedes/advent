use std::collections::HashSet;
use std::str::FromStr;

type Numbers = Vec<usize>;

#[derive(Debug)]
struct Boards {
    boards: Vec<BingoBoard>,
    winners: HashSet<usize>,
}

impl Boards {
    fn mark_boards(&mut self, value: usize) {
        self.boards
            .iter_mut()
            .for_each(|board| board.mark_number(value));
    }

    fn find_first_winner(&self) -> Option<&BingoBoard> {
        for board in &self.boards[..] {
            // dbg!(board);
            if board.winner() {
                return Some(board);
            }
        }
        None
    }

    fn find_last_winner(&mut self) -> Option<&BingoBoard> {
        for (index, board) in self.boards.iter().enumerate() {
            if !self.winners.contains(&index) {
                if board.winner() {
                    self.winners.insert(index);
                }
                if self.winners.len() == self.boards.len() {
                    return Some(board);
                }
            }
        }
        None
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Marked,
    Unmarked,
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    value: usize,
    state: State,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(Cell {
            value: s.parse::<usize>().unwrap(),
            state: State::Unmarked,
        })
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    cells: Vec<Cell>,
}

impl FromStr for BingoBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let cells = s
            .lines()
            .map(|line| line.split(' '))
            .flatten()
            .filter(|val| !val.is_empty())
            .map(str::parse::<Cell>)
            .map(Result::unwrap)
            .collect();
        Ok(BingoBoard { cells })
    }
}

impl BingoBoard {
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * 5)
    }

    fn calc_score(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| cell.state == State::Unmarked)
            .map(|cell| cell.value)
            .sum()
    }

    fn mark_number(&mut self, value: usize) {
        if let Some(index) = self.cells.iter().position(|&cell| cell.value == value) {
            self.cells[index].state = State::Marked;
        }
    }

    fn winner(&self) -> bool {
        // horizontal winners
        let x_winner = (0..5)
            .map(|y| {
                self.cells[self.get_index(0, y)..=self.get_index(4, y)]
                    .iter()
                    .all(|cell| cell.state == State::Marked)
            })
            .any(|x| x);

        let y_winner = (0..5)
            .map(|x| {
                (0..5)
                    .map(|y| self.get_index(x, y))
                    .map(|index| self.cells[index])
                    .all(|cell| cell.state == State::Marked)
            })
            .any(|x| x);

        x_winner || y_winner
    }
}

fn parse_input(input: Vec<&str>) -> (Boards, Numbers) {
    let numbers = input[0]
        .split(',')
        .filter(|val| !val.is_empty())
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let boards = input[1..]
        .iter()
        .map(|s| (*s).parse::<BingoBoard>().unwrap())
        .collect();

    (
        Boards {
            boards,
            winners: HashSet::new(),
        },
        numbers,
    )
}

fn part1(boards: &mut Boards, numbers: Numbers) -> Option<usize> {
    for number in &numbers {
        boards.mark_boards(*number);
        if let Some(winner) = boards.find_first_winner() {
            return Some(winner.calc_score() * number);
        }
    }
    None
}

fn part2(boards: &mut Boards, numbers: Numbers) -> Option<usize> {
    for number in &numbers {
        boards.mark_boards(*number);
        if let Some(last_winner) = boards.find_last_winner() {
            return Some(last_winner.calc_score() * number);
        }
    }
    None
}

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let (mut boards, numbers) = parse_input(input);
    // let score = part1(&mut boards, numbers);
    let score = part2(&mut boards, numbers);
    dbg!(score);
}
