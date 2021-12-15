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
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn calc_cheapest_path(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        let mut dist: Vec<Vec<usize>> = (0..self.height)
            .map(|_| (0..self.width).map(|_| usize::max_value()).collect())
            .collect();

        dist[0][0] = 0;
        heap.push((Reverse(0), (0, 0)));

        while let Some((Reverse(cost), (x, y))) = heap.pop() {
            if x == self.width - 1 && y == self.height - 1 {
                return Some(cost);
            }

            for (nx, ny) in self.get_neighbors(x, y) {
                let new_cost = cost + self.grid[y][x];
                if new_cost < dist[ny][nx] {
                    dist[nx][ny] = new_cost;
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
    let input = fs::read_to_string("test_input.txt").expect("Failed to read input");
    let grid = Grid::from_str(&input).unwrap();

    let cost = grid.calc_cheapest_path().unwrap();
    dbg!(cost);
}
