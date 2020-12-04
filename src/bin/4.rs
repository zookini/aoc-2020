fn main() {
    println!("Part 1: {}", validate(|k, _| k != "cid"));

    println!("Part 2: {}", validate(|k, v| match k {
        "byr" => (1920u16..=2002).contains(&v.parse().unwrap()),
        "iyr" => (2010u16..=2020).contains(&v.parse().unwrap()),
        "eyr" => (2020u16..=2030).contains(&v.parse().unwrap()),
        "hgt" => v.ends_with("cm") && (150u16..=193).contains(&v[..v.len() - 2].parse().unwrap()) ||
                 v.ends_with("in") && (059u16..=076).contains(&v[..v.len() - 2].parse().unwrap()),
        "hcl" => v.len() == 7 && v.starts_with('#') && v.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
        "cid" => false,
        _ => unreachable!()
    }));
}

fn validate(p: fn(k: &str, v: &str) -> bool) -> usize {
    include_str!("../../input/4.txt")
        .split("\r\n\r\n")
        .filter(|s| s.split_whitespace().filter(|kv| p(&kv[..3], &kv[4..])).count() == 7)
        .count()
}
