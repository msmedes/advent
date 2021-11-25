use std::collections::HashMap;

use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let mut parts = contents.split("\n\n");

    let mut rules: Rules = parts.next().unwrap().lines().map(parse_rule).collect();

    lol_k(&mut rules, "8: 42 +");
    lol_k(
        &mut rules,
        "11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31", // lol 4 ever
    );

    let messages: Vec<String> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let re = compile(&rules);

    let valid = messages
        .iter()
        .filter(|message| re.is_match(message))
        .count();

    println!("{}", valid);
}

fn lol_k(rules: &mut Rules, rule: &str) {
    let (key, rule) = parse_rule(rule);
    rules.insert(key, rule);
}

#[derive(Debug)]
enum Token {
    Char(char),
    Pipe,
    Plus,
    Rule(usize),
}

type Rule = Vec<Token>;
type Rules = HashMap<usize, Rule>;

fn parse_rule(rule: &str) -> (usize, Rule) {
    let mut parts = rule.trim().split(": ");
    let left = parts.next().unwrap().parse::<usize>().unwrap();
    let tokens = parts.next().unwrap().split(' ').map(parse_token).collect();

    (left, tokens)
}

fn parse_token(token: &str) -> Token {
    if token == "|" {
        return Token::Pipe;
    }
    if token == "+" {
        return Token::Plus;
    }
    if let Ok(val) = token.parse::<usize>() {
        return Token::Rule(val);
    }

    Token::Char(token.chars().nth(1).unwrap()) // "a" or "b"
}

fn compile_rule(rule: &Rule, rules: &Rules) -> String {
    let expanded: String = rule
        .iter()
        .map(|token| match token {
            Token::Char(char) => char.to_string(),
            Token::Pipe => "|".to_string(),
            Token::Plus => "+".to_string(),
            Token::Rule(number) => compile_rule(rules.get(number).unwrap(), rules),
        })
        .collect();
    if expanded.contains('|') {
        format!("({})", expanded)
    } else {
        expanded
    }
}

fn compile(rules: &Rules) -> Regex {
    let compiled = format!("^{}$", compile_rule(rules.get(&0).unwrap(), &rules));
    // println!("{}", compiled; uncomment to see a length 22393 regex of only 2 different chars
    Regex::new(&compiled).unwrap()
}
