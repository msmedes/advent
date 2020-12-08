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
        println!("{}", self.cursor);
    }

    fn execute_program(&mut self) {
        while !self.executed.contains(&(self.cursor as usize)) {
            self.executed.insert(self.cursor as usize);
            self.execute_instruction();
        }
        println!("{}", self.accumulator);
    }
    
    fn switch_operation(&mut self) -> {
        for i in 0..self.instructions.len() {

        }
    }

    fn change_instruction(&mut self, i) {
        let self.
    }
}

fn main() {
    let mut vm = VM::new("input.txt");
    vm.execute_program();
}

fn read_file(filename: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6";
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
