use std::fs;

fn main() {
    let mut adapters = read_file("input.txt");
    let part1 = adapter_distribution(&mut adapters);
    println!("{}", part1);
    let part2 = adapter_combinations(&adapters);
    println!("{}", part2);
}

fn adapter_combinations(adapters: &[usize]) -> isize {
    let mut dp: Vec<isize> = vec![0; adapters[adapters.len() - 1] + 1];
    dp[0] = 1;
    for adapter in adapters {
        for difference in 1..4 {
            let previous = (*adapter as isize) - difference;
            println!("{}, {}", adapter, previous);
            if previous >= 0 {
                dp[*adapter] += dp[previous as usize];
            }
        }
    }

    dp[dp.len() - 1]
}

fn adapter_distribution(adapters: &mut Vec<usize>) -> usize {
    adapters.push(0);
    adapters.sort_unstable();

    let mut threes = 0;
    let mut ones = 0;

    for i in 0..adapters.len() - 1 {
        let curr = adapters[i];
        let next = adapters[i + 1];
        if next - curr == 3 {
            threes += 1;
        }
        if next - curr == 1 {
            ones += 1
        }
    }
    threes += 1;
    println!("{} {}", ones, threes);
    threes * ones
}

fn read_file(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename).unwrap();
    // let contents = "16
    // 10
    // 15
    // 5
    // 1
    // 11
    // 7
    // 19
    // 6
    // 12
    // 4";
    // let contents = "28
    // 33
    // 18
    // 42
    // 31
    // 14
    // 46
    // 20
    // 48
    // 47
    // 24
    // 23
    // 49
    // 45
    // 19
    // 38
    // 39
    // 11
    // 1
    // 32
    // 25
    // 35
    // 8
    // 17
    // 7
    // 9
    // 4
    // 2
    // 34
    // 10
    // 3";
    contents
        .trim()
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect()
}
