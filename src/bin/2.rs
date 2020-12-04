fn main() {
    println!("Part 1: {}", passwords(|min, max, c, password| (min..=max).contains(&password.matches(c).count())));

    println!("Part 2: {}", passwords(|i, j, c, password| [i, j]
        .iter()
        .filter(|i| password.as_bytes()[*i - 1] == c.as_bytes()[0])
        .count() == 1
    ));
}

fn passwords(p: fn(usize, usize, &str, &str) -> bool) -> usize {
    regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)")
        .unwrap()
        .captures_iter(include_str!("../../input/2.txt"))
        .filter(|cap| p(cap[1].parse().unwrap(), cap[2].parse().unwrap(), &cap[3], &cap[4]))
        .count()
}
