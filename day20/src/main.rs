use anyhow::{Error, Result};
// use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Tile {
    id: usize,
    image: Vec<Vec<String>>,
    borders: HashSet<String>,
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
        println!("{}", id);
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
        let mut borders = HashSet::new();
        // lol k
        let mut left = vec![];
        let mut right = vec![];

        for row in &image {
            left.push(row[0].as_str());
            right.push(row[width - 1].as_str());
        }
        borders.insert(left.iter().map(|char| char.to_string()).collect());
        borders.insert(right.iter().map(|char| char.to_string()).collect());
        borders.insert(left.into_iter().rev().collect());
        borders.insert(right.into_iter().rev().collect());

        let mut top = vec![];
        let mut bottom = vec![];
        for col in 0..width {
            top.push(image[0][col].as_str());
            bottom.push(image[height - 1][col].as_str());
        }

        borders.insert(top.iter().map(|char| char.to_string()).collect());
        borders.insert(bottom.iter().map(|char| char.to_string()).collect());
        borders.insert(top.into_iter().rev().collect());
        borders.insert(bottom.into_iter().rev().collect());
        println!("{:?}", borders);
        Ok(Tile { id, image, borders })
    }
}

fn grapherizer(tiles: Vec<Tile>) -> HashMap<usize, HashSet<usize>> {
    let mut borders = HashMap::<String, HashSet<usize>>::new();
    let mut graph = HashMap::<usize, HashSet<usize>>::new();

    for tile in tiles {
        for border in &tile.borders {
            borders
                .entry(border.to_string())
                .and_modify(|set| {
                    let _ = set.insert(tile.id);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::<usize>::new();
                    let _ = set.insert(tile.id);
                    set
                });
        }
    }

    for ids in borders.values() {
        for combo in itertools::Itertools::combinations(ids.iter(), 2) {
            graph
                .entry(*combo[0])
                .and_modify(|set| {
                    let _ = set.insert(*combo[1]);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::<usize>::new();
                    let _ = set.insert(*combo[1]);
                    set
                });
            graph
                .entry(*combo[1])
                .and_modify(|set| {
                    let _ = set.insert(*combo[0]);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::<usize>::new();
                    let _ = set.insert(*combo[0]);
                    set
                });
        }
    }
    graph
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

    let graph = grapherizer(tiles);

    let corners: usize = graph
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(k, _)| k)
        .product();

    // let image = build_image(graph);
}
