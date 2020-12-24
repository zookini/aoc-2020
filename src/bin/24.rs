use std::collections::HashMap;

fn main() {
    let tiles = include_str!("../../input/24.txt")
        .lines()
        .map(|line| parse(line.chars()).fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy)))
        .fold(HashMap::new(), |mut tiles, point| {
            tiles.entry(point).and_modify(|p| *p = if *p == "black" { "white" } else { "black" }).or_insert("black");
            tiles
        });
    
    println!("Part 1: {}", run(tiles.clone(), 0));
    println!("Part 2: {}", run(tiles, 100));
}

fn parse(mut steps: impl Iterator<Item = char>) -> impl Iterator<Item = (i32, i32)> {
    std::iter::from_fn(move || match steps.next() {
        Some('e') => Some((-2, 0)),
        Some('w') => Some((2, 0)),
        Some('n') => match steps.next() {
            Some('e') => Some((-1, -1)),
            Some('w') => Some((1, -1)),
            _ => unreachable!()
        }
        Some('s') => match steps.next() {
            Some('e') => Some((-1, 1)),
            Some('w') => Some((1, 1)),
            _ => unreachable!()
        },
        _ => None
    })
}

fn run(tiles: HashMap<(i32, i32), &str>, days: usize) -> usize {
    std::iter::successors(Some(tiles), |tiles| Some(day(&tiles)))
        .nth(days)
        .unwrap()
        .values()
        .filter(|&&tile| tile == "black")
        .count()
}

fn day<'a>(tiles: &HashMap<(i32, i32), &'a str>) -> HashMap<(i32, i32), &'a str> {
    tiles
        .iter()
        .filter(|(_, &colour)| colour == "black")
        .flat_map(|((x, y), _)| [(-2, 0), (2, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)].iter().map(move |(dx, dy)| (x + dx, y + dy)))
        .fold(HashMap::new(), |mut next, point| {
            next.entry(point).and_modify(|blacks| *blacks += 1).or_insert(1);
            next
        })
        .iter()
        .map(|(&point, &blacks)| (point, match (*tiles.get(&point).unwrap_or(&"white"), blacks) {
            ("white", 2) => "black",
            ("black", n) if n == 0 || n > 2 => "white",
            (colour, _) => colour,
        }))
        .collect()
}
