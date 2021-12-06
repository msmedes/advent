fn main() {
    let lanternfish: Vec<usize> = include_str!("../input.txt")
        .split(',')
        .filter(|x| !x.is_empty())
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let num_lanternfish = part1(lanternfish);
    dbg!(num_lanternfish);
}

fn part1(lanternfish: Vec<usize>) -> usize {
    let mut fish_counts = vec![0; 9];

    for fish in lanternfish {
        fish_counts[fish] += 1;
    }

    for _ in 0..256 {
        fish_counts = fish_counts
            .iter()
            .enumerate()
            .map(|(index, _)| match index {
                8 => fish_counts[0],
                6 => fish_counts[7] + fish_counts[0],
                other => fish_counts[other + 1],
            })
            .collect();
    }

    fish_counts.iter().sum()
}
