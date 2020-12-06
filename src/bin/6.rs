use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", aggregate(|a, b| &a | &b));
    println!("Part 2: {}", aggregate(|a, b| &a & &b));
}

fn aggregate(op: fn(HashSet<char>, HashSet<char>) -> HashSet<char>) -> usize {
    include_str!("../../input/6.txt")
        .split("\r\n\r\n")
        .map(|group| group
            .lines()
            .map(|line| line.chars().collect())
            .fold1(|a, b| op(a, b))
            .unwrap()
            .len()
        )
        .sum()
}
