use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", parse()
        .map(|(mask, mem, value)| (mem, value & u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap() | u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap()))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
    );

    println!("Part 2: {}", parse()
        .flat_map(|(mask, mem, value)| addresses(mask, mem.rotate_right(36)).map(move |address| (address, value)))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
    );
}

fn parse() -> impl Iterator<Item = (&'static str, u64, u64)> {
    include_str!("../../input/14.txt")
        .split("mask = ")
        .skip(1)
        .flat_map(|section| section[36..]
            .lines()
            .flat_map(move |line| line
                .split(" = ")
                .collect_tuple()
                .map(|(mem, value)| (&section[..36], mem[4..mem.len() - 1].parse().unwrap(), value.parse().unwrap()))
            )
        )
}

fn addresses(mask: &str, mem: u64) -> Box<dyn Iterator<Item = u64>> {
    match mask.bytes().next() {
        Some(b'X') => Box::new(addresses(&mask[1..], mem.rotate_left(1)).chain(addresses(&mask[1..], mem.rotate_left(1) ^ 1))),
        Some(n) => addresses(&mask[1..], mem.rotate_left(1) | (n - b'0') as u64),
        None => Box::new(std::iter::once(mem)),
    }
}
