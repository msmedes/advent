use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Points {
    points: Vec<Vec<i64>>,
    width: usize,
    height: usize,
}

impl FromStr for Points {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let points: Vec<Vec<i64>> = s
            .lines()
            .map(|l| {
                l.split("")
                    .filter(|x| !x.is_empty())
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();

        Ok(Points {
            height: points.len(),
            width: points[0].len(),
            points,
        })
    }
}

fn main() {
    let points = Points::from_str(include_str!("../input.txt")).unwrap();

    // dbg!(&points);
    let risk_scores = part1(&points);
    dbg!(risk_scores);
}

fn is_valid_point(x: i64, y: i64, points: &Points) -> bool {
    0 <= x && x < (points.width as i64) && 0 <= y && y < (points.height as i64)
}

fn adjacent_points(x: usize, y: usize, points: &Points) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
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
    adjacent_points(x, y, points)
        .iter()
        .all(|(l_x, l_y)| points.points[*l_y][*l_x] > points.points[y][x])
}

fn calc_risk_score(x: usize, y: usize, points: &Points) -> usize {
    points.points[y][x] as usize + 1
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
