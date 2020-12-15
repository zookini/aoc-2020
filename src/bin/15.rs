fn main() {
    println!("Part 1: {}", run(2020));
    println!("Part 2: {}", run(30000000));
}

fn run(until: usize) -> usize {
    let mut nums: std::collections::HashMap<_, _> = [18, 8, 0, 5, 4, 1, 20].iter().copied().zip(1..).collect();
    (nums.len() + 1..until).fold(0, |num, turn| turn - nums.insert(num, turn).unwrap_or(turn))
}
