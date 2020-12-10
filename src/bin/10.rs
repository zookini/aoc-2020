use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let adapters: Vec<usize> = include_str!("../../input/10.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect();

    let differences = adapters
        .windows(2)
        .fold([1; 3], |mut differences, w| {
            differences[w[1] - w[0] - 1] += 1;
            differences
        });
    
    println!("Part 1: {}", differences[0] * differences[2]);

    println!("Part 2: {:?}", adapters
        .iter()
        .fold(std::iter::once((0, 1)).collect::<HashMap<usize, usize>>(), |mut map, &adapter| {
            map.insert(adapter, (adapter - 3.min(adapter)..adapter).filter_map(|a| map.get(&a)).sum());
            map
        })[adapters.last().unwrap()]
    );
}