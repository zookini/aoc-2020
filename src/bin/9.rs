use itertools::Itertools;

fn main() {
    let input: Vec<usize> = include_str!("../../input/9.txt").lines().map(|line| line.parse().unwrap()).collect();
    let width = 25;

    let p1 = input
        .windows(width + 1)
        .find(|w| w[..width].iter().combinations(2).all(|p| p[0] + p[1] != w[width]))
        .unwrap()[width];

    println!("Part 1: {}", p1);

    println!("Part 2: {}", (2..)
        .flat_map(|i| input.windows(i))
        .find(|w| w.iter().sum::<usize>() == p1)
        .map(|w| w.iter().min().unwrap() + w.iter().max().unwrap())
        .unwrap());
}
