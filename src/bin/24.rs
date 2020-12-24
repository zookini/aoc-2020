use itertools::iproduct;

type HashMap<K, V> = std::collections::HashMap<K, V, std::hash::BuildHasherDefault<fnv::FnvHasher>>;
type HashSet<K> = std::collections::HashSet<K, std::hash::BuildHasherDefault<fnv::FnvHasher>>;

fn main() {
    let blacks: HashSet<(i32, i32)> = include_str!("../../input/24.txt")
        .lines()
        .map(|line| parse(line.chars()).fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy)))
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

fn parse(mut steps: impl Iterator<Item = char>) -> impl Iterator<Item = (i32, i32)> {
    std::iter::from_fn(move || match steps.next() {
        Some('e') => Some((-2, 0)),
        Some('w') => Some((2, 0)),
        Some('n') => Some(match steps.next() {
            Some('e') => (-1, -1),
            Some('w') => (1, -1),
            _ => unreachable!()
        }),
        Some('s') => Some(match steps.next() {
            Some('e') => (-1, 1),
            Some('w') => (1, 1),
            _ => unreachable!()
        }),
        _ => None
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
        .iter()
        .filter_map(|(point, &count)| match (blacks.contains(point), count) {
            (true, 1) | (true, 2) | (false, 2) => Some(*point),
            _ => None
        })
        .collect()
}
