use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
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

    // this took me like 40 minutes to figure out
    // ok maybe more like two hours.
    let valid_fields: Vec<Vec<HashSet<usize>>> = valid_tickets
        .iter()
        .map(|ticket| potential_rules(&rules, ticket))
        .collect();

    let first = valid_fields[0].clone();
    let valid_fields: Vec<HashSet<usize>> = valid_fields.iter().fold(first, |a, x| {
        a.iter()
            .zip(x)
            .map(|(y, z)| y.intersection(&z).cloned().collect())
            .collect()
    });

    let mut valid_permutation: Vec<usize> = vec![];

    janky_solver(&valid_fields, &mut valid_permutation);

    dbg!(&valid_permutation);

    let mut mult = 1;
    for (i, rule_num) in valid_permutation.iter().enumerate() {
        if rules[*rule_num].name.starts_with("departure") {
            let index = valid_permutation[i];
            let val = mine.values[index];
            mult *= val;
        }
    }
    println!("{}", mult);
}

fn janky_solver(fields: &[HashSet<usize>], perm: &mut Vec<usize>) -> bool {
    // we made it to the end
    if perm.len() == fields.len() {
        return true;
    }

    let curr = &fields[perm.len()];

    for value in curr {
        if !perm.contains(&value) {
            perm.push(*value);

            if janky_solver(fields, perm) {
                return true;
            }
            perm.pop();
        }
    }
    false
}

fn potential_rules(rules: &[Rule], ticket: &Ticket) -> Vec<HashSet<usize>> {
    ticket
        .values
        .iter()
        .map(|value| {
            rules
                .iter()
                .enumerate()
                .filter_map(|(i, rule)| if rule.is_valid(*value) { Some(i) } else { None })
                .collect::<HashSet<usize>>()
        })
        .collect()
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
