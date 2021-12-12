use hashbrown::HashSet;
use itertools::Itertools;
use multimap::MultiMap;

fn main() {
    let input: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split('-').collect_tuple())
        .flatten()
        .collect();

    let mut graph = MultiMap::new();

    for edge in input {
        graph.insert(edge.0, edge.1);
        graph.insert(edge.1, edge.0);
    }

    part1(&graph);
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(char::is_lowercase)
}

fn paths(
    graph: &MultiMap<&str, &str>,
    visited: &mut HashSet<&str>,
    current: &str,
    count: &mut usize,
    time_left: usize,
) {
    if current == "end" {
        *count += 1;
    } else {
        let mut curr_set = HashSet::new();
        // if is_small_cave(current) {
        curr_set.insert(current);
        // }
        let mut curr_visited: HashSet<&str> = visited.union(&curr_set).copied().collect();

        for neighbor in graph.get_vec(current).unwrap() {
            let mut time_spent = 0;
            if is_small_cave(neighbor) && curr_visited.contains(neighbor) {
                if time_left == 0 || *neighbor == "start" {
                    continue;
                }
                time_spent = 1
            }
            paths(
                graph,
                &mut curr_visited,
                neighbor,
                count,
                time_left - time_spent,
            );
        }
    }
}

fn part1(graph: &MultiMap<&str, &str>) {
    let mut count = 0;
    paths(graph, &mut HashSet::new(), "start", &mut count, 1);
    dbg!(count);
}
