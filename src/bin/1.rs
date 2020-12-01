use itertools::Itertools;

fn main() {
    let expenses: Vec<u32> = include_str!("../../input/1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", product(&expenses, 2));
    println!("Part 2: {}", product(&expenses, 3));
}

fn product(expenses: &[u32], k: usize) -> u32 {
    expenses
        .iter()
        .copied()
        .combinations(k)
        .filter(|combination| combination.iter().sum::<u32>() == 2020)
        .map(|combination| combination.iter().product::<u32>())
        .next()
        .unwrap()
}
