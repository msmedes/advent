use std::fs;

fn main() {
    let passes = read_file("input.txt");
    let ids = calc_ids(passes);
    let my_seat = find_seat(ids).unwrap();
    // let max_id = max_id(&ids);
    // println!("{}", max_id);
    println!("{}", my_seat);
}

fn find_seat(mut ids: Vec<usize>) -> Option<usize> {
    ids.sort_unstable(); // idk clippy told me to use unstable
    let min = ids[0];
    let max = ids[ids.len() - 1];

    for seat in min..max {
        if seat != ids[seat - min] {
            return Some(seat);
        }
    }
    None
}

fn max_id(ids: &[usize]) -> usize {
    *ids.iter().max().unwrap()
}

fn convert_to_binary(pass: &str) -> String {
    pass.chars()
        .map(|ch| match ch {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => ch,
        })
        .collect()
}

fn calc_ids(passes: Vec<String>) -> Vec<usize> {
    passes
        .iter()
        .map(|pass| {
            let binary = convert_to_binary(pass);
            calc_seat_id_binary(&binary)
        })
        .collect()
}

fn calc_seat_id_binary(binary: &str) -> usize {
    let row = usize::from_str_radix(&binary[..7], 2).unwrap();
    let col = usize::from_str_radix(&binary[7..], 2).unwrap();
    row * 8 + col
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example5() {
        let pass = "FBFBBFFRLR";
        let binary = convert_to_binary(pass);
        let id = calc_seat_id_binary(&binary);
        assert_eq!(id, 357);
    }

    #[test]
    fn example6() {
        let pass = "BFFFBBFRRR";
        let binary = convert_to_binary(pass);
        let id = calc_seat_id_binary(&binary);
        assert_eq!(id, 567);
    }

    #[test]
    fn example7() {
        let pass = "FFFBBBFRRR";
        let binary = convert_to_binary(pass);
        let id = calc_seat_id_binary(&binary);
        assert_eq!(id, 119);
    }

    #[test]
    fn example8() {
        let pass = "BBFFBBFRLL";
        let binary = convert_to_binary(pass);
        let id = calc_seat_id_binary(&binary);
        assert_eq!(id, 820);
    }
}
