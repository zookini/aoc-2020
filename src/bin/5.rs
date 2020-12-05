use itertools::Itertools;

fn main() {
    let ids: Vec<_> = include_str!("../../input/5.txt")
        .lines()
        .map(|pass| pass.chars().fold(0, |id, c| id * 2 + "BR".contains(c) as u32))
        .sorted()
        .collect();

    println!("Part 1: {}", ids.last().unwrap());
    println!("Part 2: {}", ids.windows(2).find(|p| p[0] + 1 != p[1]).unwrap()[0] + 1);
}
