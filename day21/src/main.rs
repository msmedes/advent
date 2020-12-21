use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Allergens = FxHashMap<String, FxHashSet<String>>;
type Ingredients = FxHashMap<String, usize>;

fn main() {
    let (mut allergens, ingredients) = parse_input("input.txt");
    filter_ingredients(&allergens, ingredients);
    filter_allergens(&mut allergens);
}

fn filter_allergens(allergens: &mut Allergens) {
    let mut queue: VecDeque<String> = allergens
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(k, _)| k.clone())
        .collect();

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let ingredients = allergens[&curr].clone();
        for (allergen, set) in allergens.iter_mut() {
            if *allergen == curr || set.len() == 1 {
                continue;
            }
            *set = set.difference(&ingredients).cloned().collect();
            if set.len() == 1 {
                queue.push_back(allergen.to_string());
            }
        }
    }
    let mut keys = allergens.keys().cloned().collect::<Vec<String>>();
    keys.sort();
    let values: Vec<String> = keys
        .iter()
        .map(|key| allergens.get(key).unwrap().iter().next().unwrap().clone())
        .collect();

    println!("{:?}", values.join(","));
}

fn filter_ingredients(allergens: &Allergens, ingredients: Ingredients) {
    let potential_allergens: FxHashSet<String> = allergens.values().flatten().cloned().collect();
    let sum: usize = ingredients
        .iter()
        .filter(|(k, _)| !potential_allergens.contains(*k))
        .map(|(_, v)| *v)
        .sum();
    println!("{}", sum);
}

fn parse_input(filename: &str) -> (Allergens, Ingredients) {
    // let contents = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    // trh fvjkl sbzzf mxmxvkd (contains dairy)
    // sqjhc fvjkl (contains soy)
    // sqjhc mxmxvkd sbzzf (contains fish)";
    let mut ingredients: Ingredients = FxHashMap::default();
    let mut allergens: Allergens = FxHashMap::default();
    let contents = std::fs::read_to_string(filename).unwrap();
    for line in contents.lines() {
        let line = line.trim();
        let mut pieces = line.split('(');
        let first = pieces.next().unwrap(); // ingredients
        let second = pieces.next().unwrap(); //allergens
        let ings: FxHashSet<String> = first
            .split_whitespace()
            .map(|ing| {
                let ing = ing.to_string();
                ingredients
                    .entry(ing.clone())
                    .and_modify(|val| *val += 1)
                    .or_insert_with(|| 1);
                ing
            })
            .collect();
        let allers = second
            .strip_prefix("contains ")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ");
        for allergen in allers {
            let allergen = allergen.to_string();
            let ingreds = allergens.get(&allergen);
            match ingreds {
                Some(i) => allergens.insert(allergen, i.intersection(&ings).cloned().collect()),
                None => allergens.insert(allergen, ings.clone()),
            };
        }
    }
    (allergens, ingredients)
}
