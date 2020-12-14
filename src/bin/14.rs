use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", parse()
        .map(|(mask, mem, value)| (mem, value & usize::from_str_radix(&mask.replace('X', "1"), 2).unwrap() | usize::from_str_radix(&mask.replace('X', "0"), 2).unwrap()))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<usize>()
    );

    println!("Part 2: {}", parse()
        .flat_map(|(mask, mem, value)| addresses(mask.as_bytes(), 0, mem).map(move |address| (address, value)))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<usize>()
    );
}

fn parse() -> impl Iterator<Item = (&'static str, usize, usize)> {
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

fn addresses(mask: &[u8], i: usize, address:usize) -> Box<dyn Iterator<Item = usize>> {
    match mask.get(i) {
        Some(b'0') => addresses(mask, i + 1, address),
        Some(b'1') => addresses(mask, i + 1, address | 1 << 35 - i),
        Some(_) => Box::new(addresses(mask, i + 1, address).chain(addresses(mask, i + 1, address ^ 1 << 35 - i))),
        None => Box::new(std::iter::once(address)),
    }
}
