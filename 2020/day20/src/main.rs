use anyhow::{Error, Result};
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEAD: Regex = Regex::new(r"..................#.").unwrap();
    static ref BODY: Regex = Regex::new(r"#....##....##....###").unwrap();
    static ref BASE: Regex = Regex::new(r".#..#..#..#..#..#...").unwrap();
}

type Border = String;

#[derive(Debug)]
struct Borders {
    top: Border,
    bottom: Border,
    left: Border,
    right: Border,
}

type Image = Vec<Vec<String>>;

#[derive(Debug)]
struct Tile {
    id: usize,
    image: Image,
    flipped: bool,
    rotations: usize,
}

impl Tile {
    fn rotate_n(&self, n: usize) -> Self {
        let mut next = self.image.clone();
        for _ in 0..n {
            let mut rotated: Vec<Vec<String>> = vec![];
            for col in 0..next[0].len() {
                rotated.push(next.iter().rev().map(|row| row[col].clone()).collect());
            }
            next = rotated;
        }
        Tile {
            image: next,
            rotations: n,
            ..*self
        }
    }

    fn flip_horizontal(&self) -> Self {
        let next: Vec<Vec<String>> = self
            .image
            .clone()
            .iter()
            .map(|row| {
                row.clone()
                    .iter()
                    .rev()
                    .map(|char| char.to_string())
                    .collect()
            })
            .collect();

        Tile {
            image: next,
            flipped: !self.flipped,
            ..*self
        }
    }

    fn flip_vertical(&self) -> Self {
        let next = self.image.iter().rev().map(|row| row.clone()).collect();
        Tile {
            image: next,
            ..*self
        }
    }

    fn orientations(&self) -> Vec<Tile> {
        let mut orientations: Vec<Tile> = vec![];
        for n in 1..5 {
            let rotated = self.rotate_n(n);
            orientations.push(rotated.flip_horizontal());
            orientations.push(rotated);
        }
        orientations
    }

    fn top(&self) -> Border {
        self.image[0].join("")
    }

    fn bottom(&self) -> Border {
        self.image.last().unwrap().join("")
    }

    fn left(&self) -> Border {
        self.image
            .iter()
            .map(|row| row[0].clone())
            .collect::<Vec<String>>()
            .join("")
    }

    fn right(&self) -> Border {
        self.image
            .iter()
            .map(|row| row.last().unwrap().clone())
            .collect::<Vec<String>>()
            .join("")
    }

    fn borders(&mut self, reversed: bool) -> Borders {
        if reversed {
            return self.rotate_n(2).borders(false);
        }
        Borders {
            top: self.top(),
            bottom: self.bottom(),
            left: self.left(),
            right: self.right(),
        }
    }
}

impl std::str::FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Tile> {
        let mut lines = s.split('\n');
        let id = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let image: Vec<Vec<String>> = lines
            .map(|line| {
                line.trim()
                    .split("")
                    .map(|char| char.to_string())
                    .filter(|char| char != "")
                    .collect()
            })
            .collect();
        let (height, width) = (image.len(), image[0].len());
        // lol k
        let mut left = vec![];
        let mut right = vec![];

        for row in &image {
            left.push(row[0].as_str());
            right.push(row[width - 1].as_str());
        }
        let mut top = vec![];
        let mut bottom = vec![];
        for col in 0..width {
            top.push(image[0][col].as_str());
            bottom.push(image[height - 1][col].as_str());
        }

        Ok(Tile {
            id,
            image,
            flipped: false,
            rotations: 0,
        })
    }
}

fn main() {
    let contents = "Tile 2311:
   ..##.#..#.
   ##..#.....
   #...##..#.
   ####.#...#
   ##.##.###.
   ##...#.###
   .#.#.#..##
   ..#....#..
   ###...#.#.
   ..###..###

   Tile 1951:
   #.##...##.
   #.####...#
       .....#..##
       #...######
       .##.#....#
       .###.#####
       ###.##.##.
       .###....#.
       ..#.#..#.#
       #...##.#..

       Tile 1171:
       ####...##.
       #..##.#..#
       ##.#..#.#.
       .###.####.
       ..###.####
       .##....##.
       .#...####.
       #.##.####.
       ####..#...
       .....##...

       Tile 1427:
       ###.##.#..
       .#..#.##..
       .#.##.#..#
       #.#.#.##.#
       ....#...##
       ...##..##.
       ...#.#####
       .#.####.#.
       ..#..###.#
       ..##.#..#.

       Tile 1489:
       ##.#.#....
       ..##...#..
       .##..##...
       ..#...#...
       #####...#.
       #..#.#.#.#
       ...#.#.#..
       ##.#...##.
       ..##.##.##
       ###.##.#..

       Tile 2473:
       #....####.
       #..#.##...
       #.##..#...
       ######.#.#
       .#...#.#.#
       .#########
       .###.#..#.
       ########.#
       ##...##.#.
       ..###.#.#.

       Tile 2971:
       ..#.#....#
       #...###...
       #.#.###...
       ##.##..#..
       .#####..##
       .#..####.#
       #..#.#..#.
       ..####.###
       ..#.#.###.
       ...#.#.#.#

       Tile 2729:
       ...#.#.#.#
       ####.#....
       ..#.#.....
       ....#..#.#
       .##..##.#.
       .#.####...
       ####.#.#..
       ##.####...
       ##..#.##..
       #.##...##.

       Tile 3079:
       #.#.#####.
       .#..######
       ..#.......
       ######....
       ####.#..#.
       .#...#.##.
       #.#####.##
       ..#.###...
       ..#.......
       ..#.###...";

    let contents = std::fs::read_to_string("input.txt").unwrap();
    let contents: Vec<&str> = contents.split("\n\n").collect();

    let tiles: Vec<Tile> = contents
        .iter()
        .map(|tile| tile.parse::<Tile>().unwrap())
        .collect();

    // println!("{:?}", tiles);

    let grid_size = (tiles.len() as f64).sqrt() as usize;
    println!("{}", grid_size);

    let mut order: Vec<Tile> = vec![];
    let mut used = HashSet::<usize>::new();

    let ok = search(grid_size, &mut order, &mut used, &tiles);

    // println!("{:?}", order);
    let image_tiles: Vec<Tile> = order
        .iter()
        .map(|t| {
            let tile = t.rotate_n(t.rotations);
            if t.flipped {
                tile.flip_horizontal()
            } else {
                tile
            }
        })
        .collect();

    let image = compose_image(image_tiles);
    for i in 0..4 {
        let img = image.rotate_n(i);
        let stringify: Vec<String> = img
            .image
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
            .collect();
        let waves: usize = stringify
            .iter()
            .map(|x| x.chars().filter(|x| *x == '#').count())
            .sum();
        let sea_monsters = find_monster(stringify);

        println!("monsters: {}", waves - (sea_monsters * 15));

        let img = image.rotate_n(i).flip_horizontal();
        let stringify: Vec<String> = img
            .image
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
            .collect();
        let waves: usize = stringify
            .iter()
            .map(|x| x.chars().filter(|x| *x == '#').count())
            .sum();
        let sea_monsters = find_monster(stringify);

        println!("monsters: {}", waves - sea_monsters * 15);
    }
}

fn find_monster(image: Vec<String>) -> usize {
    let mut count = 0;
    for i in 0..image.len() - 1 {
        for m in BODY.find_iter(&image[i]) {
            if BASE.is_match_at(&image[i + 1], m.start())
                && HEAD.is_match_at(&image[i - 1], m.start())
            {
                println!("monster: {}, {}", i, m.start());
                count += 1;
            }
        }
    }
    count
}

fn compose_image(order: Vec<Tile>) -> Tile {
    let grid_size = (order.len() as f64).sqrt() as usize;
    println!("{}", grid_size);
    let tile_size = order[0].image.len();

    let mut image = vec![];

    for i in 0..(grid_size * tile_size) {
        let mut row = vec![];
        for j in 0..(grid_size * tile_size) {
            let img_j = j % tile_size;
            let img_i = i % tile_size;

            let image_i = i / tile_size;
            let image_j = j / tile_size;

            let tile = image_i * grid_size + image_j;
            row.push(order[tile].image[img_i][img_j].clone());
        }
        image.push(row)
    }
    Tile {
        id: 0,
        image,
        flipped: false,
        rotations: 0,
    }
}

fn search(
    grid_size: usize,
    order: &mut Vec<Tile>,
    used: &mut HashSet<usize>,
    tiles: &Vec<Tile>,
) -> bool {
    let index = order.len();

    if index == grid_size * grid_size {
        return true;
    }

    for tile in tiles {
        if used.contains(&tile.id) {
            continue;
        }

        for orientation in tile.orientations() {
            if index % grid_size != 0 && order[index - 1].right() != orientation.left() {
                // no edge match
                continue;
            }
            if index > grid_size && order[index - grid_size].bottom() != orientation.top() {
                //no middle match
                continue;
            }
            let id = orientation.id;
            used.insert(orientation.id);
            order.push(orientation);
            if search(grid_size, order, used, tiles) {
                return true;
            }
            used.remove(&id);
            order.pop();
        }
    }

    false
}
