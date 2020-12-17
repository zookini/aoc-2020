use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

fn main() {
    println!("Part 1: {}", run(0..=0));
    println!("Part 2: {}", run(-1..=1));
}

fn run(w: RangeInclusive<i16>) -> usize {
    let active: HashSet<[i16; 4]> = include_str!("../../input/17.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(move |(x, _)| [x as i16, y as i16, 0, 0]))
        .collect();

    (0..6).fold(active, |active, _| cycle(&active, &w)).len()
}

fn cycle(active: &HashSet<[i16; 4]>, w: &RangeInclusive<i16>) -> HashSet<[i16; 4]> {
    active
        .iter()
        .flat_map(|cube| iproduct!((-1..=1), (-1..=1), (-1..=1), w.clone())
            .filter(|&delta| delta != (0, 0, 0, 0)) 
            .map(move |delta| [cube[0] + delta.0, cube[1] + delta.1, cube[2] + delta.2, cube[3] + delta.3])
        )
        .fold(HashMap::new(), |mut neighbours, cube| {
            *neighbours.entry(cube).or_insert(0) += 1;
            neighbours
        })
        .into_iter()
        .filter(|(cube, neighbours)| active.contains(cube) && [2, 3].contains(neighbours) || *neighbours == 3)
        .map(|(cube, _)| cube)
        .collect()
}
