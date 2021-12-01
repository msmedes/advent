fn main() -> anyhow::Result<()> {
    let input: Vec<i64> = include_str!("../input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let part1 = count_ascending_depths(&input);
    dbg!(part1);
    let part2 = count_ascending_depths_windows(&input, 3);
    dbg!(part2);

    Ok(())
}

fn count_ascending_depths(measurements: &[i64]) -> usize {
    measurements.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_ascending_depths_windows(measurements: &[i64], window_size: usize) -> usize {
    measurements
        .windows(window_size)
        .map(|w| (w.iter().sum::<i64>()))
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            .split('\n')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect();

        let count = count_ascending_depths(&input);

        assert!(count == 7)
    }

    #[test]
    fn part2() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            .split('\n')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect();

        let count = count_ascending_depths_windows(&input, 3);

        assert!(count == 5)
    }
}
