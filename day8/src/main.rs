use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}
#[derive(Debug)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

#[derive(Debug)]
struct VM {
    accumulator: isize,
    cursor: isize,
    instructions: Vec<Instruction>,
    executed: HashSet<usize>,
}

impl VM {
    fn new(filename: &str) -> VM {
        VM {
            accumulator: 0,
            cursor: 0,
            instructions: read_file(filename),
            executed: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.cursor = 0;
        self.executed.drain();
    }

    fn execute_instruction(&mut self) {
        let instruction = &self.instructions[self.cursor as usize];
        match instruction.operation {
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.cursor += 1
            }
            Operation::Jmp => self.cursor += instruction.argument,
            Operation::Nop => self.cursor += 1,
        };
    }

    fn execute_program(&mut self) -> Result<(), &'static str> {
        while self.cursor < self.instructions.len() as isize {
            if !self.executed.contains(&(self.cursor as usize)) {
                self.executed.insert(self.cursor as usize);
                self.execute_instruction();
            } else {
                return Err("don't worry about it");
            }
        }
        Ok(())
    }

    fn search_and_destroy(&mut self) {
        for i in 0..self.instructions.len() {
            self.change_instruction(i);
            let answer = self.execute_program();
            if answer.is_ok() {
                println!("{}", self.accumulator);
                break;
            }
            self.change_instruction(i);
            self.reset();
        }
    }

    fn change_instruction(&mut self, i: usize) {
        let new_operation = match self.instructions[i].operation {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            Operation::Acc => Operation::Acc,
        };
        // meh
        self.instructions[i] = Instruction {
            operation: new_operation,
            argument: self.instructions[i].argument,
        }
    }
}

fn main() {
    let mut vm = VM::new("input.txt");
    // vm.execute_program();
    vm.search_and_destroy();
}

fn read_file(filename: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| {
            let mut pieces = line.trim().split_whitespace();
            Instruction {
                operation: match pieces.next().unwrap() {
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    "nop" => Operation::Nop,
                    _ => panic!("invalid operation"),
                },
                argument: pieces.next().unwrap().parse::<isize>().unwrap(),
            }
        })
        .collect()
}
