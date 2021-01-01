use std::unreachable;

use aoc::regex;
use itertools::iproduct;

type HashMap<K, V> = std::collections::HashMap<K, V, std::hash::BuildHasherDefault<fnv::FnvHasher>>;
type HashSet<K> = std::collections::HashSet<K, std::hash::BuildHasherDefault<fnv::FnvHasher>>;

fn main() {
    let blacks: HashSet<(i32, i32)> = include_str!("../../input/24.txt")
        .lines()
        .map(|line| parse(line).fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy)))
        .fold(HashMap::default(), |mut tiles, point| {
            tiles.entry(point).and_modify(|p| *p = if *p == "black" { "white" } else { "black" }).or_insert("black");
            tiles
        })
        .into_iter()
        .filter(|&(_, colour)| colour == "black")
        .map(|(point, _)| point)
        .collect();
    
    println!("Part 1: {}", run(blacks.clone(), 0));
    println!("Part 2: {}", run(blacks, 100));
}

fn parse(directions: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    regex!("(n|s)?.").captures_iter(directions).map(|cap| match &cap[0] {
        "ne" => (-1, -1),
        "nw" => (1, -1),
        "se" => (-1, 1),
        "sw" => (1, 1),
        "e" => (-2, 0),
        "w" => (2, 0),
        _ => unreachable!()
    })
}

fn run(blacks: HashSet<(i32, i32)>, days: usize) -> usize {
    (0..days).fold(blacks, |blacks, _| day(&blacks)).len()
}

fn day(blacks: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    iproduct!(blacks, &[(-2, 0), (2, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)])
        .map(|((x, y), (dx, dy))| (x + dx, y + dy))
        .fold(HashMap::default(), |mut next, point| {
            next.entry(point).and_modify(|count| *count += 1).or_insert(1);
            next
        })
        .into_iter()
        .filter(|(point, count)| matches!((blacks.contains(point), count), (true, 1..=2) | (false, 2)))
        .map(|(point, _)| point)
        .collect()
}
