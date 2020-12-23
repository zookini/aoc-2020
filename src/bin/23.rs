fn main() {
    let mut input: Vec<_> = "487912365".bytes().map(|b| (b - b'0') as i32).collect();
    println!("Part 1: {}", run(&input, 100).take_while(|&c| c != 1).fold(0, |num, i| num * 10 + i));

    input.extend(input.iter().max().unwrap() + 1..=1_000_000);
    println!("Part 2: {}", run(&input, 10_000_000).take(2).product::<i32>());
}

fn run(input: &[i32], moves: usize) -> impl Iterator<Item = i32> {
    let mut cur = input[0];
    let mut cups = vec![cur; input.len() + 1];

    for w in input.windows(2) {
        cups[w[0] as usize] = w[1];
    }

    for _ in 0..moves {
        let i = cups[cur as usize];
        let j = cups[i as usize];
        let k = cups[j as usize];

        let destination = (cur - 5..cur)
            .rev()
            .map(|c| c.rem_euclid(cups.len() as i32))
            .find(|c| ![i, j, k, 0].contains(c))
            .unwrap() as usize;

        cups[cur as usize] = cups[k as usize];
        cups[k as usize] = cups[destination];
        cups[destination] = i;
        
        cur = cups[cur as usize];
    }

    std::iter::successors(cups.get(1).copied(), move |&c| cups.get(c as usize).copied())
}
