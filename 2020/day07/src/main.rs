use std::collections::{HashMap, HashSet};
use std::fs;

use regex::Regex;

type Graph = HashMap<String, Vec<(String, usize)>>;

fn main() {
    let bags = read_file("input.txt");
    let (graph, nodes) = build_graph(bags);
    let target = "shiny gold";
    let total: usize = part1(nodes, target, &graph);
    let total2 = part2(target, &graph);
    println!("{}", total);
    println!("{}", total2);
}

fn part2(target: &str, graph: &Graph) -> usize {
    match graph.get(target) {
        Some(bags) => bags
            .iter()
            .map(|(node, count)| count + count * part2(node, &graph))
            .sum(),
        None => 0,
    }
}

fn part1(nodes: HashSet<String>, target: &str, graph: &Graph) -> usize {
    nodes
        .iter()
        .map(|node| {
            if node != target {
                search(node, target, &graph) as usize
            } else {
                0
            }
        })
        .sum()
}

fn search(start: &str, target: &str, graph: &Graph) -> bool {
    if start == target {
        true
    } else {
        match graph.get(start) {
            Some(edges) => edges
                .iter()
                .map(|node| search(&node.0, target, graph))
                .any(|x| x),
            None => false,
        }
    }
}

fn build_graph(bags: Vec<String>) -> (Graph, HashSet<String>) {
    // there's gotta be a better way, and I'm sure there is

    let re = Regex::new(r"([a-z]+ [a-z]+) bags contain (.*)\.").unwrap();
    let re2 = Regex::new(r"(\d) ([a-z]+ [a-z]+) bag[s]?").unwrap();
    let mut graph: Graph = HashMap::new();
    let mut nodes: HashSet<String> = HashSet::new();
    for bag in &bags {
        let mut source = String::from("");
        let mut dest_match = String::from("");
        for captures in re.captures_iter(bag) {
            source = captures.get(1).unwrap().as_str().to_string();
            dest_match = captures.get(2).unwrap().as_str().to_string();
        }
        nodes.insert(source.clone());
        let dests = dest_match.split(", ");
        for dest in dests {
            if dest != "no other bags" {
                let mut amount = String::from("");
                let mut bag_type = String::from("");
                for captures in re2.captures_iter(dest) {
                    amount = captures.get(1).unwrap().as_str().to_string();
                    bag_type = captures.get(2).unwrap().as_str().to_string();
                }
                graph
                    .entry(source.clone())
                    .and_modify(|vec| {
                        vec.push((bag_type.clone(), amount.parse::<usize>().unwrap()))
                    })
                    .or_insert_with(|| vec![(bag_type.clone(), amount.parse::<usize>().unwrap())]);
            }
        }
    }

    (graph, nodes)
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}
