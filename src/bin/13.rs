use itertools::Itertools;

fn main() {
    let (time, buses): (i64, Vec<(i64, i64)>) = include_str!("../../input/13.txt")
        .lines()
        .collect_tuple()
        .map(|(time, ids)| (time.parse().unwrap(), ids
            .split(',')
            .enumerate()
            .filter_map(|(i, id)| id.parse().ok().map(|id| (i as i64, id)))
            .collect()
        ))
        .unwrap();

    println!("Part 1: {}", buses
        .iter()
        .map(|(_, id)| (id, id - time % id))
        .min_by_key(|&(_, diff)| diff)
        .map(|(id, diff)| id * diff)
        .unwrap()
    );

    let (residues, modulii): (Vec<_>, Vec<_>) = buses.iter().map(|(offset, id)| (id - offset, id)).unzip();

    println!("Part 2: {}", crt(&residues, &modulii));
}

fn crt(residues: &[i64], modulii: &[i64]) -> i64 {
    let mp = modulii.iter().product::<i64>();

    residues.iter().zip(modulii)
        .map(|(&r, &m)| r * egcd(mp / m, m).0.rem_euclid(m) * (mp / m))
        .sum::<i64>() % mp
}

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if a == 0 {
        (0, 1)
    } else {
        let (x, y) = egcd(b % a, a);
        (y - (b / a) * x, x)
    }
}
