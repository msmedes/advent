use hashbrown::HashMap;

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split("").filter(|x| !x.is_empty()).collect())
        .collect();

    // dbg!(&input);
    part2(&input);
}

fn is_corrupted(chunks: &[&str], close_to_open: &HashMap<&str, &str>) -> Option<String> {
    let mut stack: Vec<&str> = vec![];
    for ch in chunks {
        if close_to_open.contains_key(ch) {
            if stack[stack.len() - 1] == *close_to_open.get(ch).unwrap() {
                stack.pop();
            } else {
                return Some(ch.to_string());
            }
        } else {
            stack.push(ch)
        }
    }
    None
}

fn get_autocomplete(chunks: &[&str], close_to_open: &HashMap<&str, &str>) -> Option<Vec<String>> {
    let mut stack: Vec<&str> = vec![];
    for ch in chunks {
        if close_to_open.contains_key(ch) {
            if stack[stack.len() - 1] == *close_to_open.get(ch).unwrap() {
                stack.pop();
            } else {
                return None;
            }
        } else {
            stack.push(ch)
        }
    }
    stack.reverse();
    Some(stack.iter().map(|m| m.to_string()).collect())
}

fn part1(input: &[Vec<&str>]) {
    let close_to_open = [(")", "("), ("]", "["), ("}", "{"), (">", "<")];
    let close_to_open = close_to_open
        .iter()
        .copied()
        .collect::<HashMap<&str, &str>>();

    let scores = [(")", 3), ("]", 57), ("}", 1197), (">", 25137)]
        .iter()
        .copied()
        .collect::<HashMap<&str, usize>>();

    let score: usize = input
        .iter()
        .map(|l| is_corrupted(l, &close_to_open))
        .flatten()
        .map(|c| scores.get(c.as_str()).unwrap())
        .sum();

    dbg!(score);
}

fn part2(input: &[Vec<&str>]) {
    let close_to_open = [(")", "("), ("]", "["), ("}", "{"), (">", "<")];
    let close_to_open = close_to_open
        .iter()
        .copied()
        .collect::<HashMap<&str, &str>>();

    let scores = [("(", 1), ("[", 2), ("{", 3), ("<", 4)]
        .iter()
        .copied()
        .collect::<HashMap<&str, usize>>();

    let autocompletes: Vec<Vec<String>> = input
        .iter()
        .map(|l| get_autocomplete(l, &close_to_open))
        .flatten()
        .collect();

    let mut values: Vec<usize> = autocompletes
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| *scores.get(c.as_str()).unwrap())
                .reduce(|a, b| 5 * a + b)
        })
        .flatten()
        .collect();

    values.sort_unstable();
    let mid_point = values.len() / 2;
    let median = values[mid_point];
    dbg!(median);
}
