fn main() {
    println!("Part 1: {}", travel(&(3, 1)));
    println!("Part 2: {}", [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter().map(travel).product::<usize>());
}

fn travel(&(right, down): &(usize, usize)) -> usize {
    include_str!("../../input/3.txt")
        .lines()
        .step_by(down)
        .zip((0..).step_by(right))
        .skip(1)
        .filter(|(row, right)| row.as_bytes()[right % row.len()] == b'#')
        .count()
}
