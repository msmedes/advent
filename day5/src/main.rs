use std::fs;

fn main() {
    let passes = read_file("input.txt");
    let ids = calc_ids(passes);
    let my_seat = find_seat(ids);
    // let max_id = max_id(ids);
    // println!("{}", max_id);
    println!("{}", my_seat);
}

fn find_seat(mut ids: Vec<usize>) -> usize {
    let min = min_id(&ids);
    let max = max_id(&ids);
    ids.sort_unstable();

    for seat in min..max {
        if seat != ids[seat - min] {
            return seat;
        }
    }
    // if nothing found
    0
}

fn max_id(ids: &[usize]) -> usize {
    *ids.iter().max().unwrap()
}

fn min_id(ids: &[usize]) -> usize {
    *ids.iter().min().unwrap()
}

fn calc_ids(passes: Vec<String>) -> Vec<usize> {
    passes
        .iter()
        .map(|pass| {
            let row = find_dimension(pass, 0, 0, 127);
            let col = find_dimension(pass, 7, 0, 7);
            calc_seat_id(row, col)
        })
        .collect()
}

fn find_dimension(pass: &str, index: usize, low: usize, high: usize) -> usize {
    if low == high {
        return low;
    }
    match pass.chars().nth(index) {
        Some('F') | Some('L') => find_dimension(pass, index + 1, low, (high + low) / 2),
        Some('B') | Some('R') => find_dimension(pass, index + 1, (high + 1 + low) / 2, high),
        Some(_) | None => panic!("not supposed to happen"),
    }
}

fn calc_seat_id(row: usize, col: usize) -> usize {
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
    fn example1() {
        let pass = "FBFBBFFRLR";
        let row = find_dimension(pass, 0, 0, 127);
        let col = find_dimension(pass, 7, 0, 7);
        let id = calc_seat_id(row, col);
        assert_eq!(id, 357);
    }

    #[test]
    fn example2() {
        let pass = "BFFFBBFRRR";
        let row = find_dimension(pass, 0, 0, 127);
        let col = find_dimension(pass, 7, 0, 7);
        let id = calc_seat_id(row, col);
        assert_eq!(id, 567);
    }

    #[test]
    fn example3() {
        let pass = "FFFBBBFRRR";
        let row = find_dimension(pass, 0, 0, 127);
        let col = find_dimension(pass, 7, 0, 7);
        let id = calc_seat_id(row, col);
        assert_eq!(id, 119);
    }

    #[test]
    fn example4() {
        let pass = "BBFFBBFRLL";
        let row = find_dimension(pass, 0, 0, 127);
        let col = find_dimension(pass, 7, 0, 7);
        let id = calc_seat_id(row, col);
        assert_eq!(id, 820);
    }
}
