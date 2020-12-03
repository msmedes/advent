use std::fs;

fn main() {
    let map = read_file("input.txt");
    count_trees1(&map);
    count_trees2(&map);
}


fn read_file(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    file.lines()
    .map(|line| line.trim().to_string())
    .collect::<Vec<String>>()
}

fn count_trees1(map: &Vec<String>){
    let mut count = 0;
    let mut col = 0;
    for row in map{
        let row_str = &row;
        if row_str.chars().nth(col).unwrap() == '#' {
            count += 1
        }
        col = (col + 3) % row_str.len();
    }
    println!("{}", count); 
}

fn count_trees2(map: &Vec<String>){
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut mult: usize = 1;
    for (col_i, row_i) in &slopes {
        let mut count = 0;
        let mut col = 0;
        for row in (0..map.len()).step_by(*row_i){
            let row_str = &map[row];
            if row_str.chars().nth(col).unwrap() == '#' {
                count += 1
            }
            col = (col + col_i) % row_str.len();
        }
        mult *= count;
    }
    println!("{}", mult); 
}