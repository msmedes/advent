use std::collections::HashSet;
use std::fs;

fn main() {
    let nums = read_file("input.txt");
    let weakness = find_weakness(&nums).unwrap();
    println!("{}", weakness);
    let set_sum = find_sequence(&nums, weakness).unwrap();
    println!("{}", set_sum);
}

fn find_weakness(nums: &[isize]) -> Option<isize> {
    for num in 25..nums.len() {
        let val = nums[num];
        if !two_sum(nums, num) {
            return Some(val);
        }
    }
    None
}

fn two_sum(nums: &[isize], index: usize) -> bool {
    let mut lookup: HashSet<isize> = HashSet::new();

    for i in index - 25..index {
        let val = nums[i];
        let complement = nums[index] - val;
        let contains = lookup.contains(&complement);
        if !contains {
            lookup.insert(val as isize);
        } else if contains && val != complement {
            return true;
        }
    }
    false
}

fn find_sequence(nums: &[isize], target: isize) -> Option<isize> {
    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i + 1..nums.len() {
            sum += nums[j];
            if sum == target {
                return Some(
                    nums[i..j + 1].iter().max().unwrap() + nums[i..j + 1].iter().min().unwrap(),
                );
            }
        }
    }
    None
}

fn read_file(filename: &str) -> Vec<isize> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "35
    // 20
    // 15
    // 25
    // 47
    // 40
    // 62
    // 55
    // 65
    // 95
    // 102
    // 117
    // 150
    // 182
    // 127
    // 219
    // 299
    // 277
    // 309
    // 576";
    contents
        .lines()
        .map(|line| line.trim().parse::<isize>().unwrap())
        .collect()
}
