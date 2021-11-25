use indicatif::ProgressIterator;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let paths = read_input("input.txt");
    let dir_vec: Vec<(&str, (f32, f32))> = vec![
        ("ne", (0.5, 1.0)),
        ("nw", (-0.5, 1.0)),
        ("se", (0.5, -1.0)),
        ("sw", (-0.5, -1.0)),
        ("w", (-1.0, 0.0)),
        ("e", (1.0, 0.0)),
    ];
    let directions: FxHashMap<&str, (f32, f32)> = dir_vec.iter().map(|(k, v)| (*k, *v)).collect();
    let flipped = flip_tiles(paths, &directions);
    for_fucks_sake(flipped, directions)
}

fn for_fucks_sake(flipped: FxHashSet<String>, directions: FxHashMap<&str, (f32, f32)>) {
    let mut room: FxHashMap<String, usize> = flipped.iter().map(|tile| (tile.clone(), 1)).collect();

    for _ in 0..100 {
        let mut next = room.clone();
        let room_keys = room.keys().cloned().collect::<FxHashSet<String>>();
        let mut neighbors = FxHashMap::<String, usize>::default();

        for (tile, color) in room {
            if color != 1 {
                continue;
            }
            let split: Vec<&str> = tile.split(',').collect();
            let x = split[0].parse::<f32>().unwrap();
            let y = split[1].parse::<f32>().unwrap();

            for (dx, dy) in directions.values() {
                let this_tile = format!("{},{}", x + dx, y + dy);
                neighbors
                    .entry(this_tile.clone())
                    .and_modify(|val| *val += 1)
                    .or_insert_with(|| 1);
            }
        }

        let all: FxHashSet<String> = room_keys
            .union(&neighbors.keys().cloned().collect::<FxHashSet<String>>())
            .map(|key| key.clone())
            .collect();

        for tile in all {
            let count = match neighbors.get(&tile) {
                Some(val) => *val,
                None => 0,
            };
            let color = next.get(&tile);
            match color {
                Some(val) if *val == 1 => {
                    if count == 0 || count > 2 {
                        next.insert(tile.clone(), 0);
                    }
                }
                Some(val) if *val == 0 => {
                    if count == 2 {
                        next.insert(tile.clone(), 1);
                    }
                }
                None => {
                    if count == 2 {
                        next.insert(tile.clone(), 1);
                    } else {
                        next.insert(tile.clone(), 0);
                    }
                }
                Some(_huh) => panic!("nope {}", _huh),
            }
        }
        room = next;
    }
    let count = room.iter().filter(|(_, v)| **v == 1).count();
    println!("{}", count);
}

fn flip_tiles(
    paths: Vec<Vec<String>>,
    directions: &FxHashMap<&str, (f32, f32)>,
) -> FxHashSet<String> {
    let mut flipped_tiles = FxHashSet::<String>::default();
    for path in paths {
        follow_path(path, &mut flipped_tiles, directions);
    }

    flipped_tiles
}

fn follow_path(
    path: Vec<String>,
    tiles: &mut FxHashSet<String>,
    directions: &FxHashMap<&str, (f32, f32)>,
) {
    let mut curr_coord = (0.0, 0.0);
    for direction in path {
        let delta = directions[&direction.as_str()];
        curr_coord = (curr_coord.0 + delta.0, curr_coord.1 + delta.1);
    }
    let tile = format!("{},{}", curr_coord.0, curr_coord.1);
    if tiles.contains(&tile) {
        tiles.remove(&tile);
    } else {
        tiles.insert(tile);
    }
}

fn read_input(filename: &str) -> Vec<Vec<String>> {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"se|nw|ne|sw|e|w").unwrap();

    contents
        .lines()
        .map(|line| {
            let mut path = vec![];
            for cap in re.captures_iter(&line.trim().to_string()) {
                path.push(cap[0].to_string());
            }
            path
        })
        .collect()
}
