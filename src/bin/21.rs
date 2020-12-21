use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

fn main() {
    let foods: Vec<(HashSet<&str>, Vec<&str>)> = include_str!("../../input/21.txt")
        .lines()
        .filter_map(|line| line
            .split(" (contains ")
            .collect_tuple()
            .map(|(ingredients, allergens)| (ingredients.split(" ").collect(), allergens[..allergens.len() - 1].split(", ").collect()))
        )
        .collect();

    let mut ai: BTreeMap<&str, HashSet<&str>> = BTreeMap::new();

    for (ingredients, allergens) in &foods {
        for &allergen in allergens {
            ai.entry(allergen).and_modify(|i| *i = ingredients & i).or_insert_with(|| ingredients.clone());
        }
    }

    let mut bad: BTreeMap<&str, &str> = BTreeMap::new();

    while let Some((&allergen, ingredients)) = ai.iter().find(|(_, ingredients)| ingredients.len() == 1) {
        let &ingredient = ingredients.iter().next().unwrap();

        for is in ai.values_mut() {
            is.remove(ingredient);
        }

        bad.insert(allergen, ingredient);
    }

    let irritants: HashSet<&str> = bad.values().copied().collect();

    println!("Part 1: {}", foods.iter().flat_map(|(ingredients, _)| ingredients - &irritants).count());
    println!("Part 2: {}", bad.values().join(","));
}
