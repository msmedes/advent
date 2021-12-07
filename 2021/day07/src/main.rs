fn main() {
    let crabs: Vec<i64> = include_str!("../input.txt")
        .split(',')
        .filter(|x| !x.is_empty())
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let least_fuel = part1(&crabs);
    dbg!(least_fuel);
    let least_fuel_triangle = part2(&crabs);
    dbg!(least_fuel_triangle);
}

fn part1(crabs: &[i64]) -> i64 {
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    (*min..=*max)
        .map(|i| crabs.iter().map(|c| i64::abs(c - i)).sum())
        .min()
        .unwrap()
}

fn part2(crabs: &[i64]) -> i64 {
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    (*min..=*max)
        .map(|i| {
            crabs
                .iter()
                .map(|c| {
                    let d = i64::abs(c - i);
                    (d * d + d) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
