use hashbrown::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let splits: Vec<&str> = input.split("\n\n").collect();
    let template = splits[0]
        .split("")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let insertion_rules: HashMap<(&str, &str), &str> = splits[1]
        .lines()
        .map(|line| line.split(" -> ").collect_tuple())
        .flatten()
        .map(|(a, b)| {
            (
                a.split("")
                    .filter(|x| !x.is_empty())
                    .collect_tuple()
                    .unwrap(),
                b,
            )
        })
        .collect();

    // dbg!(template, insertion_rules);
    part2(template, &insertion_rules);
}

fn step<'a>(
    template: Vec<&'a str>,
    insertion_rules: &'a HashMap<(&str, &str), &str>,
) -> Vec<&'a str> {
    let mut new_polymer = vec![template[0]];
    for window in template.windows(2) {
        let first = window[0];
        let second = window[1];
        if let Some(letter) = insertion_rules.get(&(first, second)) {
            new_polymer.push(letter);
        }
        new_polymer.push(second);
    }
    new_polymer
}

fn part2(template: Vec<&str>, insertion_rules: &HashMap<(&str, &str), &str>) {
    let mut pairs: HashMap<(&str, &str), usize> = HashMap::new();
    for window in template.windows(2) {
        *pairs.entry((window[0], window[1])).or_default() += 1;
    }
    // dbg!(&pairs);
    for _ in 0..40 {
        let mut new_pairs: HashMap<(&str, &str), usize> = HashMap::new();
        for pair in pairs.keys() {
            let curr_count = pairs.get(pair).unwrap();
            if let Some(value) = insertion_rules.get(pair) {
                *new_pairs.entry((pair.0, value)).or_insert(0) += *curr_count;
                *new_pairs.entry((value, pair.1)).or_insert(0) += *curr_count;
            } else {
                *new_pairs.entry(*pair).or_insert(*curr_count) += 1;
            }
        }
        pairs = new_pairs;
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for (k, v) in pairs.iter() {
        *counts.entry(k.0).or_insert(0) += v
    }
    *counts.entry(template[template.len() - 1]).or_insert(0) += 1;

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    dbg!(max - min);
}
