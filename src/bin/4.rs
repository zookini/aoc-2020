fn main() {
    println!("Part 1: {}", validate(|k, _| k != "cid"));

    println!("Part 2: {}", validate(|k, v| match k {
        "byr" => matches!(v.parse(), Ok(1920u16..=2002)),
        "iyr" => matches!(v.parse(), Ok(2010u16..=2020)),
        "eyr" => matches!(v.parse(), Ok(2020u16..=2030)),
        "hgt" => matches!((&v[..v.len() - 2].parse(), &v[v.len() - 2..]), (Ok(150u16..=193), "cm") | (Ok(59u16..=76), "in")),
        "hcl" => matches!((&v[0..1], u32::from_str_radix(&v[1..], 16), v.len()), ("#", Ok(_), 7)),
        "ecl" => matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => matches!((v.parse::<u32>(), v.len()), (Ok(_), 9)),
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
