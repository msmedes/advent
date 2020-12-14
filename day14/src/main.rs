use std::collections::{HashMap, HashSet};
use std::fs;

use regex::Regex;

fn main() {
    let program = read_file("input.txt");
    // part1(program);
    part2(program);
}

fn part1(program: Vec<String>) {
    let mut curr_mask: HashMap<usize, String> = HashMap::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let memory_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for line in program {
        let mut mem_loc = 0;
        let mut value = 0;
        if line.starts_with("mask") {
            curr_mask = process_mask(line.strip_prefix("mask = ").unwrap());
        } else {
            for captures in memory_re.captures_iter(&line) {
                mem_loc = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                value = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            }
            let mut binary_value = format!("{:036b}", value);
            for (index, binary) in &curr_mask {
                let upper = index + 1;
                binary_value.replace_range(index..&upper, &binary[..]);
            }
            memory.insert(mem_loc, usize::from_str_radix(&binary_value, 2).unwrap());
        }
    }
    println!("{:?}", memory.values().sum::<usize>());
}

fn part2(program: Vec<String>) {
    let mut x_set = vec![];
    let mut one_set = vec![];
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let memory_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    for line in program {
        let mut mem_loc = 0;
        let mut value = 0;
        if line.starts_with("mask") {
            let (curr_x, curr_one) = process_mask2(line.strip_prefix("mask = ").unwrap());
            x_set = curr_x;
            one_set = curr_one;
        } else {
            for captures in memory_re.captures_iter(&line) {
                mem_loc = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                value = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            }
            // first we have to replace all the 1 indices, as those will not change
            let mut binary_mem = format!("{:036b}", mem_loc);
            for index in &one_set {
                let upper = index + 1;
                binary_mem.replace_range(index..&upper, "1");
            }
            combos(binary_mem, value, &mut memory, &x_set, 0);
        }
    }
    // dbg!(&memory);
    println!("{:?}", memory.values().sum::<usize>());
}

fn combos(
    mem_loc: String,
    value: usize,
    memory: &mut HashMap<usize, usize>,
    x_set: &Vec<usize>,
    index: usize,
) {
    if index < x_set.len() {
        let mut this_zero = mem_loc.clone();
        let mut this_one = mem_loc.clone();
        let location = x_set[index];
        this_zero.replace_range(location..location + 1, "0");
        this_one.replace_range(location..location + 1, "1");
        memory.insert(usize::from_str_radix(&this_zero, 2).unwrap(), value);
        memory.insert(usize::from_str_radix(&this_one, 2).unwrap(), value);
        combos(this_zero, value, memory, x_set, index + 1);
        combos(this_one, value, memory, x_set, index + 1);
    }
}

fn process_mask(mask: &str) -> HashMap<usize, String> {
    let mut mask_map = HashMap::new();
    for (i, char) in mask.chars().enumerate() {
        if char != 'X' {
            mask_map.insert(i, char.to_string());
        }
    }
    mask_map
}

fn process_mask2(mask: &str) -> (Vec<usize>, Vec<usize>) {
    let mut x_set = vec![];
    let mut one_set = vec![];
    for (i, char) in mask.chars().enumerate() {
        if char == 'X' {
            x_set.push(i);
        }
        if char == '1' {
            one_set.push(i);
        }
    }
    (x_set, one_set)
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11
    // mem[7] = 101
    // mem[8] = 0";
    // let contents = "mask = 000000000000000000000000000000X1001X
    // mem[42] = 100
    // mask = 00000000000000000000000000000000X0XX
    // mem[26] = 1";
    contents
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}
