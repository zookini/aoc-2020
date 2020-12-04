fn main() {
    println!("Part 1: {}", travel(3, 1));

    println!("Part 2: {}", [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| travel(right, down))
        .product::<usize>()
    );
}

fn travel(right: usize, down: usize) -> usize {
    include_str!("../../input/3.txt")
        .lines()
        .step_by(down)
        .enumerate()
        .skip(1)
        .filter(|(i, row)| row.as_bytes()[i * right % row.len()] == b'#')
        .count()
}
