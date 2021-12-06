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
    let mut fish_counts = [0; 9];

    for fish in lanternfish {
        fish_counts[fish] += 1;
    }
    let mut prev_0 = fish_counts[0];
    for _ in 0..256 {
        (0..9).for_each(|index| {
            fish_counts[index] = match index {
                8 => prev_0,
                6 => prev_0 + fish_counts[7],
                other => fish_counts[other + 1],
            };
        });
        prev_0 = fish_counts[0];
    }

    fish_counts.iter().sum()
}
