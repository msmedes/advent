use std::fs;

fn main() {
    let buses = read_file("input.txt");
    // part1(depart, buses);
    part2(buses);
}

fn part1(depart: usize, buses: Vec<usize>) {
    let misses: Vec<usize> = buses.iter().map(|bus| depart % bus).collect();
    let waits: Vec<usize> = buses
        .iter()
        .zip(misses)
        .map(|(bus, miss)| bus - miss)
        .collect();

    let mut min_wait = std::usize::MAX;
    let mut index = 0;
    for (i, wait) in waits.iter().enumerate() {
        if *wait < min_wait {
            min_wait = *wait;
            index = i;
        }
    }
    println!("{}", buses[index] * min_wait);
}

fn part2(buses: Vec<String>) {
    let mut multiple = 1;
    let mut step = 1;
    let base = buses[0].parse::<usize>().unwrap();

    for (offset, bus) in buses.iter().enumerate() {
        if offset == 0 || *bus == "x" {
            continue;
        }
        let bus_int = bus.parse::<usize>().unwrap();
        while ((base * multiple) + offset) % bus_int != 0 {
            multiple += step;
        }
        step *= bus_int;
    }

    println!("{}", multiple * base);
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    //     let contents = "939
    // 7,13,x,x,59,x,31,19";

    let mut pieces = contents.split('\n');
    let _ = pieces.next().unwrap();
    pieces
        .next()
        .unwrap()
        .split(',')
        // .filter(|char| *char != "x")
        .map(|x| x.to_string())
        .collect()
}
