use std::fs;

#[derive(Debug)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: isize,
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    direction: Direction,
}

fn part1(instructions: &Vec<Instruction>) {
    let mut position = Position {
        x: 0,
        y: 0,
        direction: Direction::E,
    };
    for instruction in instructions {
        position = follow_instruction(instruction, position);
    }
    let distance_travelled = manhatty(
        Position {
            x: 0,
            y: 0,
            direction: Direction::E,
        },
        &position,
    );
    println!("{:?}, {}", position, distance_travelled);
}

fn manhatty(origin: Position, result: &Position) -> usize {
    (origin.x - result.x).abs() as usize + (origin.y - result.y).abs() as usize
}

fn follow_instruction(instruction: &Instruction, position: Position) -> Position {
    match instruction.action {
        Action::N | Action::S | Action::E | Action::W => move_cardinal(instruction, position),
        Action::L | Action::R | Action::F => move_relative(instruction, position),
    }
}

fn move_relative(instruction: &Instruction, position: Position) -> Position {
    match instruction.action {
        Action::L | Action::R | Action::F => calc_new_direction(instruction, position),
        _ => panic!("not a relative instruction"),
    }
}

fn calc_new_direction(instruction: &Instruction, position: Position) -> Position {
    let directions = vec![Direction::E, Direction::S, Direction::W, Direction::N];
    let direction = match instruction.action {
        Action::L => {
            let turns = instruction.value / 90;
            let to_make = position.direction as isize - turns;
            let index: usize;
            if to_make >= 0 {
                index = position.direction as usize - turns as usize;
            } else {
                index = (4 + to_make) as usize;
            }
            directions[index]
        }
        Action::R => {
            directions[((position.direction as usize + (instruction.value / 90) as usize)
                % directions.len()) as usize]
        }
        _ => position.direction,
    };
    match instruction.action {
        Action::L | Action::R => Position {
            direction,
            ..position
        },
        Action::F => {
            let x = position.x;
            let y = position.y;
            match position.direction {
                Direction::E => Position {
                    x: x + instruction.value,
                    ..position
                },
                Direction::S => Position {
                    y: y + instruction.value,
                    ..position
                },
                Direction::W => Position {
                    x: x - instruction.value,
                    ..position
                },
                Direction::N => Position {
                    y: y - instruction.value,
                    ..position
                },
            }
        }
        _ => panic!("lol idk how you go this far"),
    }
}

fn move_cardinal(instruction: &Instruction, position: Position) -> Position {
    let x = position.x;
    let y = position.y;
    match instruction.action {
        Action::N => Position {
            y: y - instruction.value,
            ..position
        },
        Action::S => Position {
            y: y + instruction.value,
            ..position
        },
        Action::E => Position {
            x: x + instruction.value,
            ..position
        },
        Action::W => Position {
            x: x - instruction.value,
            ..position
        },
        _ => panic!("not a cardinal instruction"),
    }
}

fn main() {
    let instructions = read_file("input.txt");
    part1(&instructions);
}

fn read_file(filename: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "F10
    // N3
    // F7
    // L90
    // F11";
    contents
        .lines()
        .map(|mut line| {
            line = line.trim();
            let action = match &line.chars().next().unwrap() {
                // but not here?
                'N' => Action::N,
                'S' => Action::S,
                'E' => Action::E,
                'W' => Action::W,
                'L' => Action::L,
                'R' => Action::R,
                'F' => Action::F,
                _ => panic!("no"),
            };
            let value = &line[1..].parse::<isize>().unwrap(); // indexing into a slice is fine here
            Instruction {
                action,
                value: *value,
            }
        })
        .collect()
}
