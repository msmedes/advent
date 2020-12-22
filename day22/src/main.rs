use std::collections::VecDeque;

use rustc_hash::FxHashSet;

type Deck = VecDeque<usize>;
fn main() {
    let (mut player1, mut player2) = parse_input("input.txt");
    println!("{:?} {:?}", player1, player2);
    // let sum = play_game(&mut player1, &mut player2);
    // println!("{}", sum);
    let (deck1, deck2) = recursive_game(&mut player1, &mut player2);
    let sum = match (deck1, deck2) {
        (Some(deck1), None) => calc_score(deck1),
        (None, Some(deck2)) => calc_score(deck2),
        (_, _) => panic!("lol what"),
    };
    println!("{}", sum);
}

fn play_game(deck1: &mut Deck, deck2: &mut Deck) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card_one = deck1.pop_front().unwrap();
        let card_two = deck2.pop_front().unwrap();

        if card_one > card_two {
            deck1.push_back(card_one);
            deck1.push_back(card_two);
        } else {
            deck2.push_back(card_two);
            deck2.push_back(card_one);
        }
    }
    // println!("{:?} {:?}", deck1, deck2);

    let mut sum = 0;
    if !deck1.is_empty() {
        for i in 0..deck1.len() {
            sum += deck1[i] * (deck1.len() - i);
        }
        return sum;
    }
    for i in 0..deck2.len() {
        sum += deck2[i] * (deck2.len() - i);
    }
    return sum;
}

fn recursive_game<'a>(
    deck1: &'a mut Deck,
    deck2: &'a mut Deck,
) -> (Option<&'a mut Deck>, Option<&'a mut Deck>) {
    let mut prev_hands = FxHashSet::default();
    while !deck1.is_empty() && !deck2.is_empty() {
        let deck1_hash = deck1.iter().map(|i| i.to_string()).collect::<String>();
        let deck2_hash = deck2.iter().map(|i| i.to_string()).collect::<String>();
        if prev_hands.contains(&deck1_hash) || prev_hands.contains(&deck2_hash) {
            return (Some(deck1), None);
        } else {
            prev_hands.insert(deck1_hash);
            prev_hands.insert(deck2_hash);
        }
        let card_one = deck1.pop_front().unwrap();
        let card_two = deck2.pop_front().unwrap();
        let deck1_winner: bool;
        if card_one <= deck1.len() && card_two <= deck2.len() {
            // turns out VecDeque's can't have slices taken from them so I had to 
            // write this hideous double copy
            let deck1_vec: Vec<usize> = deck1.iter().cloned().collect();
            let mut deck1_copy: VecDeque<usize> = deck1_vec[..card_one].iter().copied().collect();
            let deck2_vec: Vec<usize> = deck2.iter().cloned().collect();
            let mut deck2_copy = deck2_vec[..card_two].iter().copied().collect();
            let (deck1_recursive, deck2_recursive) =
                recursive_game(&mut deck1_copy, &mut deck2_copy);
            match (deck1_recursive, deck2_recursive) {
                (Some(_), None) => deck1_winner = true,
                (None, Some(_)) => deck1_winner = false,
                (_, _) => panic!("weird"),
            }
        } else {
            if card_one > card_two {
                deck1_winner = true;
            } else {
                deck1_winner = false;
            }
        }
        if deck1_winner {
            deck1.push_back(card_one);
            deck1.push_back(card_two);
        } else {
            deck2.push_back(card_two);
            deck2.push_back(card_one);
        }
    }
    if !deck1.is_empty() {
        return (Some(deck1), None);
    }
    (None, Some(deck2))
}

fn calc_score(deck: &Deck) -> usize {
    let mut sum = 0;
    for i in 0..deck.len() {
        sum += deck[i] * (deck.len() - i);
    }
    return sum;
}

fn parse_input(filename: &str) -> (Deck, Deck) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut pieces = contents.split("\n\n");
    let first = pieces.next().unwrap();
    let second = pieces.next().unwrap();

    let mut player1 = first.split("\n");
    let _ = player1.next();
    let deck1 = player1
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    let mut player2 = second.split("\n");
    let _ = player2.next();
    let deck2 = player2
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    (deck1, deck2)
}
