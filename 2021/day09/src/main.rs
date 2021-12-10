use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Points {
    points: Vec<usize>,
    width: usize,
    height: usize,
}

impl FromStr for Points {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let points: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.split("")
                    .filter(|x| !x.is_empty())
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();

        Ok(Points {
            height: points.len(),
            width: points[0].len(),
            points: points.iter().flatten().copied().collect(),
        })
    }
}

fn main() {
    let points = Points::from_str(include_str!("../input.txt")).unwrap();

    // dbg!(&points);
    // let risk_scores = part1(&points);
    let basins = part2(&points);
    dbg!(basins);
}

fn is_valid_point(x: i64, y: i64, points: &Points) -> bool {
    0 <= x && x < (points.width as i64) && 0 <= y && y < (points.height as i64)
}

fn get_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

fn adjacent_points(x: usize, y: usize, points: &Points) -> Vec<(usize, usize)> {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(delta_x, delta_y)| {
            (
                i64::try_from(x).unwrap() + delta_x,
                i64::try_from(y).unwrap() + delta_y,
            )
        })
        .filter(|(p_x, p_y)| is_valid_point(*p_x, *p_y, points))
        .map(|(p_x, p_y)| (usize::try_from(p_x).unwrap(), usize::try_from(p_y).unwrap()))
        .collect()
}

fn is_low_point(x: usize, y: usize, points: &Points) -> bool {
    adjacent_points(x, y, points).iter().all(|(l_x, l_y)| {
        points.points[get_index(*l_x, *l_y, points.width)]
            > points.points[get_index(x, y, points.width)]
    })
}

fn calc_risk_score(x: usize, y: usize, points: &Points) -> usize {
    points.points[get_index(x, y, points.width)] as usize + 1
}

fn part1(points: &Points) -> usize {
    (0..points.height)
        .map(|y: usize| {
            (0..points.width)
                .filter(|x: &usize| is_low_point(*x, y, points))
                .map(|x: usize| calc_risk_score(x, y, points))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn find_island(
    point: (usize, usize),
    points: &Points,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut stack = vec![point];

    let mut size = 0;

    while !stack.is_empty() {
        let curr_point = stack.pop().unwrap();
        if !visited.contains(&curr_point) {
            size += 1;
            visited.insert(curr_point);
            let curr_val = points.points[get_index(curr_point.0, curr_point.1, points.width)];
            for neighbor in adjacent_points(curr_point.0, curr_point.1, points) {
                let index = get_index(neighbor.0, neighbor.1, points.width);
                let neighbor_val = points.points[index];
                if neighbor_val != 9 && neighbor_val > curr_val {
                    stack.push(neighbor);
                }
            }
        }
    }
    size
}

fn part2(points: &Points) -> usize {
    let mut low_points: Vec<(usize, usize)> = vec![];

    for y in 0..points.height {
        for x in 0..points.width {
            if is_low_point(x, y, points) {
                low_points.push((x, y));
            }
        }
    }
    let mut visited = HashSet::<(usize, usize)>::new();

    let mut sizes: Vec<usize> = low_points
        .iter()
        .map(|point| find_island(*point, points, &mut visited))
        .sorted()
        .collect::<Vec<usize>>();

    sizes.reverse();

    sizes[0..3].iter().copied().product()
}
