use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let (rules, mine, scanned) = read_file("input.txt");

    let rules: Vec<Rule> = rules
        .split('\n')
        .map(|line| line.trim().parse::<Rule>().unwrap())
        .collect();

    let mine = mine.lines().nth(1).unwrap().parse::<Ticket>().unwrap();

    let scanned: Vec<Ticket> = scanned
        .lines()
        .skip(1)
        .map(|ticket| ticket.trim().parse::<Ticket>().unwrap())
        .collect();

    // let invalid = invalid_tickets(rules, scanned);
    // println!("{}", invalid);

    let valid_tickets = valid_tickets(&rules, &scanned);
    let range: Vec<usize> = (0..valid_tickets[0].values.len()).collect();
    println!("{:#?}", range);
    for perm in range.iter().permutations(range.len()) {
        if valid_permutation(&perm, &rules, &valid_tickets) {
            let mut multiple = 1;
            for value in perm {
                let rule = &rules[*value];
                if rule.name.starts_with("departure") {
                    multiple *= mine.values[*value];
                }
            }
            println!("I am not proud of how this went: {}", multiple);
            break;
        }
    }
}

fn valid_permutation(permutations: &Vec<&usize>, rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> bool {
    for ticket in tickets {
        for (index, perm) in permutations.iter().enumerate() {
            if !rules[**perm].is_valid(ticket.values[index]) {
                return false;
            }
        }
    }
    true
}

fn valid_tickets(rules: &[Rule], tickets: &[Ticket]) -> Vec<Ticket> {
    tickets
        .iter()
        .cloned()
        .filter(|ticket| {
            for value in ticket.values.iter() {
                if !rules.iter().any(|rule| rule.is_valid(*value)) {
                    return false;
                }
            }
            true
        })
        .collect()
}

fn invalid_tickets(rules: Vec<Rule>, tickets: Vec<Ticket>) -> usize {
    let mut errors = 0;
    for ticket in tickets.iter() {
        for value in ticket.values.iter() {
            if !rules.iter().any(|rule| rule.is_valid(*value)) {
                errors += value;
            }
        }
    }
    errors
}

#[derive(Debug, Clone)]
struct Ticket {
    values: Vec<usize>,
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let values = s
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        Ok(Ticket { values })
    }
}

// type Rules = Vec<Rule>;
// type Tickets = Vec<Ticket>;

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    bounds: Vec<(usize, usize)>,
}

impl Rule {
    fn is_valid(&self, value: usize) -> bool {
        self.bounds
            .iter()
            .any(|(low, high)| low <= &value && &value <= high)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"^(\D*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
        let captures = re.captures(s).unwrap();

        let name = captures.get(1).unwrap().as_str();

        let field_one_low = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let field_one_high = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let field_two_low = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let field_two_high = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        Ok(Rule {
            name: name.to_string(),
            bounds: vec![
                (field_one_low, field_one_high),
                (field_two_low, field_two_high),
            ],
        })
    }
}

fn read_file(filename: &str) -> (String, String, String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    // let contents = "class: 1-3 or 5-7
    // row: 6-11 or 33-44
    // seat: 13-40 or 45-50

    // your ticket:
    // 7,1,14

    // nearby tickets:
    // 7,3,47
    // 40,4,50
    // 55,2,20
    // 38,6,12";
    // let contents = "class: 0-1 or 4-19
    // row: 0-5 or 8-19
    // seat: 0-13 or 16-19

    // your ticket:
    // 11,12,13

    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9";

    let mut sections = contents.split("\n\n");

    let rules = sections.next().unwrap();
    let mine = sections.next().unwrap();
    let scanned = sections.next().unwrap();

    (rules.to_string(), mine.to_string(), scanned.to_string())
}
