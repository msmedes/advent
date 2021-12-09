use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
fn main() {
    let input: Vec<(Vec<&str>, Vec<&str>)> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|p| p.split(' ').filter(|x| !x.is_empty()).collect())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let digit_counts = part2(input);
    dbg!(digit_counts);
}

fn part1(input: Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    input
        .iter()
        .map(|(_, second)| second)
        .map(|s: &Vec<&str>| {
            s.iter()
                .filter(|m| m.len() == 2 || m.len() == 4 || m.len() == 3 || m.len() == 7)
                .count()
        })
        .sum()
}

fn get_permutation_map(permutation: Vec<&str>) -> HashMap<&str, &str> {
    permutation
        .iter()
        .zip("abcdefg".split("").filter(|x| !x.is_empty()))
        .map(|(a, b)| (*a, b))
        .collect::<HashMap<&str, &str>>()
}

fn part2(input: Vec<(Vec<&str>, Vec<&str>)>) -> i64 {
    let display = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let display_set = display.iter().copied().collect::<HashSet<&str>>();

    input
        .par_iter()
        .map(|(a, b)| {
            let mut num = 0;
            for perm in "abcdefg"
                .split("")
                .filter(|x| !x.is_empty())
                .permutations(7)
            {
                let char_map = get_permutation_map(perm);
                let first_mapping = a
                    .iter()
                    .map(|c| {
                        c.split("")
                            .filter(|x| !x.is_empty())
                            .map(|letter| char_map.get(&letter).unwrap())
                            .sorted()
                            .join("")
                    })
                    .collect::<HashSet<String>>();
                if first_mapping
                    == display_set
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<HashSet<String>>()
                {
                    let second_mapping: Vec<String> = b
                        .iter()
                        .map(|c| {
                            c.split("")
                                .filter(|x| !x.is_empty())
                                .map(|letter| char_map.get(&letter).unwrap())
                                .sorted()
                                .join("")
                        })
                        .collect();
                    num = second_mapping
                        .iter()
                        .map(|numeral| display.iter().position(|n| n == numeral).unwrap())
                        .join("")
                        .parse::<i64>()
                        .unwrap();
                }
            }
            num
        })
        .sum()
}
