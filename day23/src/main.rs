use rustc_hash::FxHashMap;

type Cups = FxHashMap<usize, usize>;

fn main() {
    // let mut vals = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    // let test2 = "54321";
    let input = "467528193";
    let mut vals: Vec<usize> = input
        .split("")
        .filter(|val| !val.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for i in 10..1_000_001 {
        vals.push(i);
    }
    let mut cups: Cups = vals
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, vals[(idx + 1) % vals.len()]))
        .collect();

    // play_game(&mut cups, vals[0]);
    // let order = get_order(cups);
    // println!("{}", order);

    play_game2(&mut cups, vals[0]);
}

fn get_order(cups: Cups) -> String {
    let mut curr = cups.get(&1).unwrap();
    let mut order = vec![format!("{}", curr)];
    for _ in 0..cups.len() - 2 {
        let next = cups.get(&curr).unwrap();
        order.push(format!("{}", next));
        curr = next;
    }

    order.join("")
}

fn play_game2(cups: &mut Cups, start_val: usize) {
    let mut curr_cup = start_val;
    let mut destination: usize;
    let mut picks: Vec<usize>;
    let min = *cups.keys().min().unwrap();
    let max = *cups.keys().max().unwrap();

    for _ in 0..10_000_000 {
        picks = get_picks(cups, curr_cup);
        destination = get_destination(&picks, curr_cup, min, max);
        insert_picks(cups, &picks, destination);
        curr_cup = *cups.get(&curr_cup).unwrap();
    }

    let mut temp = cups[&1];
    let mut mult = 1;
    for _ in 0..2 {
        mult *= temp;
        temp = cups[&temp];
    }
    println!("{}", mult);
}

fn play_game(cups: &mut Cups, start_val: usize) {
    let mut curr_cup = start_val;
    let mut destination: usize;
    let mut picks: Vec<usize>;
    let max = *cups.keys().max().unwrap();
    let min = *cups.keys().min().unwrap();

    for _ in 0..100 {
        picks = get_picks(cups, curr_cup);
        destination = get_destination(&picks, curr_cup, min, max);
        insert_picks(cups, &picks, destination);
        curr_cup = *cups.get(&curr_cup).unwrap();
    }
}

fn get_destination(picks: &Vec<usize>, curr_cup: usize, min: usize, max: usize) -> usize {
    let mut destination = if curr_cup > 1 { curr_cup - 1 } else { max };
    while picks.contains(&destination) {
        destination -= 1;
        if destination < min {
            destination = max;
        }
    }
    destination
}

fn get_picks(cups: &mut Cups, curr_cup: usize) -> Vec<usize> {
    let mut picks = vec![];
    let mut this_cup = cups[&curr_cup];
    for _ in 0..3 {
        picks.push(this_cup);
        this_cup = cups[&this_cup];
    }
    cups.insert(curr_cup, this_cup);

    picks
}

fn insert_picks(cups: &mut Cups, picks: &Vec<usize>, destination: usize) {
    let temp = cups[&destination];
    cups.insert(destination, picks[0]);
    cups.insert(*picks.last().unwrap(), temp);
}
