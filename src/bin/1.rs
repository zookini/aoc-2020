use itertools::Itertools;

fn main() {
    println!("Part 1: {}", product(2));
    println!("Part 2: {}", product(3));
}

fn product(k: usize) -> u32 {
    include_str!("../../input/1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .combinations(k)
        .find(|combination| combination.iter().sum::<u32>() == 2020)
        .unwrap()
        .iter()
        .product()
}
