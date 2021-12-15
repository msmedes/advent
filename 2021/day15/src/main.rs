use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn get_neighbors(&self, x: usize, y: usize, factor: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width * factor - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height * factor - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn calc_cost(&self, x: usize, y: usize) -> usize {
        let grid_x = x % self.width;
        let grid_y = y % self.height;

        let x_risk: i64 = (x / self.width) as i64;
        let y_risk: i64 = (y / self.height) as i64;

        let cost = std::cmp::max(
            (self.grid[grid_y][grid_x] as i64 + x_risk + y_risk) % 10,
            self.grid[grid_y][grid_x] as i64 + x_risk + y_risk - 9,
        );
        cost as usize
    }

    fn calc_cheapest_path(&self, factor: usize) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        let mut dist: Vec<Vec<usize>> = (0..self.height * factor)
            .map(|_| {
                (0..self.width * factor)
                    .map(|_| usize::max_value())
                    .collect()
            })
            .collect();

        dist[0][0] = 0;
        heap.push((Reverse(0), (0, 0)));

        while let Some((Reverse(cost), (x, y))) = heap.pop() {
            if x == self.width * factor - 1 && y == self.height * factor - 1 {
                return Some(cost);
            }

            for (nx, ny) in self.get_neighbors(x, y, factor) {
                let new_cost = cost + self.calc_cost(nx, ny);
                if new_cost < dist[ny][nx] {
                    dist[ny][nx] = new_cost;
                    heap.push((Reverse(new_cost), (nx, ny)));
                }
            }
        }
        None
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let grid = lines
            .iter()
            .map(|line| {
                line.split("")
                    .filter(|x| !x.is_empty())
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        Ok(Grid {
            grid,
            height,
            width,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");
    let grid = Grid::from_str(&input).unwrap();

    let cost = grid.calc_cheapest_path(5).unwrap();
    dbg!(cost);
}
