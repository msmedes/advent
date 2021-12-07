use std::cmp;

fn main() {
    let crabs: Vec<i64> = include_str!("../input.txt")
        .split(',')
        .filter(|x| !x.is_empty())
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let least_fuel = part1(&crabs);
    dbg!(least_fuel);
    let least_fuel_triangle = part2(crabs);
    dbg!(least_fuel_triangle);
}

fn part1(crabs: &[i64]) -> i64 {
    let mut least_fuel = i64::MAX;
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for dist in *min..=*max {
        let fuel = crabs.iter().map(|c| i64::abs(c - dist)).sum();
        least_fuel = cmp::min(least_fuel, fuel);
    }

    least_fuel
}

fn part2(crabs: Vec<i64>) -> i64 {
    let mut least_fuel = i64::MAX;
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for dist in *min..=*max {
        let fuel = crabs
            .iter()
            .map(|c| {
                let d = i64::abs(c - dist);
                (d * d + d) / 2
            })
            .sum();
        least_fuel = cmp::min(least_fuel, fuel);
    }

    least_fuel
}
