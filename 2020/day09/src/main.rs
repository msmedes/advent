use std::collections::HashSet;
use std::fs;

fn main() {
    let nums = read_file("input.txt");
    let weakness = find_weakness(&nums).unwrap();
    println!("{}", weakness);
    let set_sum = find_sequence_window(&nums, weakness);
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
            lookup.insert(val);
        } else if contains && val != complement {
            return true;
        }
    }
    false
}

fn find_sequence(nums: &[isize], target: isize) -> Option<isize> {
    // I do not think a sliding window approach would work here for every valid
    // input since the array is not sorted, and cannot be sorted. I can't think
    // of any other ways to use the structure (or lack thereof) of the data to
    // our advantage.

    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i + 1..nums.len() {
            sum += nums[j];
            if sum == target {
                return Some(
                    nums[i..j + 1].iter().max().unwrap() + nums[i..j + 1].iter().min().unwrap(), // lol
                );
            }
        }
    }
    None
}

fn find_sequence_window(nums: &[isize], target: isize) -> isize {
    let mut low = 0;
    let mut high = 0;
    let mut sum = 0;

    while sum != target {
        if sum > target {
            sum -= nums[low];
            low += 1;
        } else if sum < target {
            sum += nums[high];
            high += 1;
        }
    }
    nums[low..high + 1].iter().max().unwrap() + nums[low..high + 1].iter().min().unwrap()
}

fn read_file(filename: &str) -> Vec<isize> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.trim().parse::<isize>().unwrap())
        .collect()
}
