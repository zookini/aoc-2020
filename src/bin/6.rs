use itertools::Itertools;

fn main() {
    println!("Part 1: {}", aggregate(|a, b| a | b));
    println!("Part 2: {}", aggregate(|a, b| a & b));
}

fn aggregate(op: fn(u32, u32) -> u32) -> u32 {
    include_str!("../../input/6.txt")
        .split("\r\n\r\n")
        .map(|group| group
            .lines()
            .map(|line| line.bytes().fold(0, |set, b| set | 1 << (b - b'a') as u32))
            .fold1(|a, b| op(a, b))
            .unwrap()
            .count_ones()
        )
        .sum()
}
