use std::fs;

fn main() {
    let passports = read_file("input.txt");
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let present_count = present(&passports, &fields);
    let num_valid = valid_count(&passports, &fields);
    println!("{}", present_count);
    println!("{}", num_valid);
}

fn valid(passport: &str) -> bool {
    let fields = passport.split_whitespace();
    for field in fields {
        // lol
        let pairs = field.split(':').collect::<Vec<_>>();
        if !valid_field(pairs[0], pairs[1]) {
            return false;
        }
    }
    true
}

fn valid_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => validate_byr(value),
        "iyr" => validate_iyr(value),
        "eyr" => validate_eyr(value),
        "hgt" => validate_hgt(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        "cid" => true,
        _ => false,
    }
}

fn validate_ecl(value: &str) -> bool {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    colors.contains(&value)
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9 && value.parse::<i32>().is_ok()
}

fn validate_hcl(value: &str) -> bool {
    if !value.starts_with('#') {
        return false;
    }
    let code = &value[1..];
    for char in code.chars() {
        if ('0' <= char && char <= '9') || ('a' <= char && char <= 'z') {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn validate_hgt(value: &str) -> bool {
    let valid: bool;
    if value.ends_with("cm") {
        // lol
        let cms = value.strip_suffix("cm").unwrap().parse::<i32>().unwrap();
        valid = 150 <= cms && cms <= 193;
    } else {
        let ins = match value.strip_suffix("in") {
            Some(height) => height.parse::<i32>().unwrap(),
            None => return false,
        };
        valid = 59 <= ins && ins <= 76;
    }
    valid
}

fn validate_eyr(value: &str) -> bool {
    let len = value.len() == 4;
    let year = value.parse::<i32>().unwrap();
    len && 2020 <= year && year <= 2030
}

fn validate_iyr(value: &str) -> bool {
    let len = value.len() == 4;
    let year = value.parse::<i32>().unwrap();
    len && 2010 <= year && year <= 2020
}

fn validate_byr(value: &str) -> bool {
    let len = value.len() == 4;
    let year = value.parse::<i32>().unwrap();
    len && 1920 <= year && year <= 2002
}

fn valid_count(passports: &[String], fields: &[&str]) -> usize {
    passports
        .iter()
        .filter(|passport| fields_present(passport, fields))
        .filter(|passport| valid(*passport))
        .count()
}

fn fields_present(passport: &str, fields: &[&str]) -> bool {
    let total_fields = fields
        .iter()
        .filter(|field| passport.contains(*field))
        .count();
    total_fields == fields.len()
}

fn present(passports: &[String], fields: &[&str]) -> usize {
    passports
        .iter()
        .filter(|passport| fields_present(&passport, &fields))
        .count()
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .split("\n\n")
        .map(|line| line.to_string())
        .collect()
}
