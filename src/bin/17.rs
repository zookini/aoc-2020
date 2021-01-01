use itertools::{enumerate, iproduct};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

fn main() {
    println!("Part 1: {}", run(0..=0));
    println!("Part 2: {}", run(-1..=1));
}

fn run(wrange: RangeInclusive<i16>) -> usize {
    let active: HashSet<[i16; 4]> = enumerate(include_str!("../../input/17.txt").lines())
        .flat_map(|(y, line)| line.chars().enumerate().filter(|&(_, c)| c == '#').map(move |(x, _)| [x as i16, y as i16, 0, 0]))
        .collect();

    (0..6).fold(active, |active, _| cycle(&active, &wrange)).len()
}

fn cycle(active: &HashSet<[i16; 4]>, wrange: &RangeInclusive<i16>) -> HashSet<[i16; 4]> {
    active
        .iter()
        .flat_map(|&[x, y, z, w]| iproduct!((-1..=1), (-1..=1), (-1..=1), wrange.clone())
            .filter(|&delta| delta != (0, 0, 0, 0)) 
            .map(move |(dx, dy, dz, dw)| [x + dx, y + dy, z + dz, w + dw])
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
