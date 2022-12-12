use std::collections::HashSet;
use std::fs;

type RuckSack = HashSet<u8>;
type RuckSacks = (RuckSack, RuckSack);

fn main() {
    let file = fs::read_to_string("./src/inputs/day03.txt").expect("boops");
    let input_1: Vec<RuckSacks> = file
        .lines()
        .map(|line| line.chars().map(to_priority).collect())
        .map(to_sets)
        .collect();

    let input_2: Vec<RuckSack> = file
        .lines()
        .map(|line| line.chars().map(to_priority).collect())
        .collect();

    let answer_1 = part1(input_1);
    println!("{answer_1}");
    let answer_2: usize = part2(input_2);
    println!("{answer_2}")
}

fn part1(all_rucks: Vec<RuckSacks>) -> usize {
    all_rucks.iter().map(intersection_priority).sum()
}

fn part2(all_rucks: Vec<RuckSack>) -> usize {
    all_rucks.chunks(3).map(intersections).sum()
}

fn to_priority(item: char) -> u8 {
    if item.is_lowercase() {
        return (item as u8 - b'a') + 1;
    }
    (item as u8 - b'A') + 27
}

fn to_sets(items: Vec<u8>) -> (RuckSack, RuckSack) {
    let half = items.len() / 2;

    (
        HashSet::from_iter(items[..half].iter().cloned()),
        HashSet::from_iter(items[half..].iter().cloned()),
    )
}

fn intersection_priority(rucksacks: &RuckSacks) -> usize {
    let (sack_1, sack_2) = rucksacks;
    let intersection: HashSet<_> = sack_1.intersection(sack_2).collect();
    **intersection.iter().next().unwrap() as usize
}

fn intersections(rucksacks: &[RuckSack]) -> usize {
    let intersections = rucksacks
        .iter()
        .skip(1)
        .fold(rucksacks[0].clone(), |acc, rs| {
            acc.intersection(rs).cloned().collect()
        });
    *intersections.iter().next().unwrap() as usize
}
