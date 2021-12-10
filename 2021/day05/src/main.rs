use hashbrown::HashMap;
use itertools::Itertools;
use std::cmp;

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split(" -> ").collect())
        .collect();

    let pairs: Vec<Vec<(i64, i64)>> = input
        .iter()
        .map(|l| {
            l.iter()
                .map(|v| {
                    v.split(',')
                        .map(str::parse::<i64>)
                        .map(Result::unwrap)
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();
    let part1_overlaps = part1(&pairs);
    dbg!(part1_overlaps);
    let part2_overlaps = part2(pairs);
    dbg!(part2_overlaps);
}

fn part1(vents: &[Vec<(i64, i64)>]) -> usize {
    let mut map = HashMap::new();
    for vent_line in vents {
        let (start, end) = (vent_line[0], vent_line[1]);
        let (x1, y1) = start;
        let (x2, y2) = end;

        if x1 == x2 || y1 == y2 {
            for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }
    map.values().filter(|val| **val >= 2).count()
}

fn part2(vents: Vec<Vec<(i64, i64)>>) -> usize {
    let mut map = HashMap::new();
    for vent_line in vents {
        let (start, end) = (vent_line[0], vent_line[1]);
        let (x1, y1) = start;
        let (x2, y2) = end;

        let mut dx = x2 - x1;
        let mut dy = y2 - y1;

        if dx != 0 {
            dx = dx / i64::abs(dx);
        }
        if dy != 0 {
            dy = dy / i64::abs(dy);
        }

        let (mut x, mut y) = (x1, y1);

        loop {
            *map.entry((x, y)).or_insert(0) += 1;
            if x == x2 && y == y2 {
                break;
            }
            x += dx;
            y += dy;
        }
    }
    map.values().filter(|val| **val >= 2).count()
}
