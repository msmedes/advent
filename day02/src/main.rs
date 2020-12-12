use std::fs;

// https://adventofcode.com/2020/day/2
fn main() {
    let passwords = read_file(String::from("input.txt"));
    let count = passwords
        .iter()
        .map(|line| parse_line(line.to_string()))
        .filter(|entry| entry.is_valid2())
        .count();
    println!("{}", count);
}

struct Entry {
    low: usize,
    high: usize,
    password: String,
    target: char,
}

impl Entry {
    fn is_valid(&self) -> bool {
        let count = self.count_chars();
        self.low <= count && count <= self.high
    }
    fn count_chars(&self) -> usize {
        self.password.matches(self.target).count()
    }

    fn is_valid2(&self) -> bool {
        let low_char = self.password.chars().nth(self.low - 1).unwrap() == self.target;
        let high_char = self.password.chars().nth(self.high - 1).unwrap() == self.target;
        low_char != high_char
    }
}

fn read_file(file_name: String) -> Vec<String> {
    let file = fs::read_to_string(file_name).unwrap();
    file.lines().map(|line| line.trim().to_string()).collect()
}

fn parse_line(line: String) -> Entry {
    // lol
    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc

    let mut parts = line.split_whitespace();
    let bounds = parts.next().unwrap();
    let target = parts.next().unwrap();
    let target = target.chars().next().unwrap();
    let password = parts.next().unwrap();

    let mut bounds = bounds.split('-');
    let low = bounds.next().unwrap();
    let high = bounds.next().unwrap();

    Entry {
        low: low.parse::<usize>().unwrap(),
        high: high.parse::<usize>().unwrap(),
        target,
        password: String::from(password),
    }
}
