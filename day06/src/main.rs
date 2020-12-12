use std::collections::HashSet;
use std::fs;

fn main() {
    let answers = read_file("input.txt");
    let count = part1(&answers);
    println!("{}", count);
    let count2 = part2(&answers);
    println!("{}", count2);
}

fn part1(answers: &[String]) -> usize {
    answers.iter().map(|answer| count_unique(&answer)).sum()
}

fn count_unique(group: &str) -> usize {
    let letters: HashSet<char> = group.chars().filter(|ch| ch.is_alphabetic()).collect();
    letters.len()
}

fn part2(answers: &[String]) -> usize {
    answers.iter().map(|answer| count_common(&answer)).sum()
}

fn count_common(group: &str) -> usize {
    let answers: Vec<&str> = group.split('\n').collect();
    let sets: Vec<HashSet<char>> = answers
        .iter()
        .map(|answer| {
            let set: HashSet<char> = answer.chars().collect();
            set
        })
        .collect();
    let first = &sets[0];
    let intersection: HashSet<&char> = first
        .iter()
        .filter(|ch| sets.iter().all(|s| s.contains(ch)))
        .collect();
    intersection.len()
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .split("\n\n")
        .map(|line| line.trim().to_string())
        .collect()
}
