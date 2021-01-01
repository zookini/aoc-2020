use itertools::Itertools;

fn main() {
    let (a, b) = include_str!("../../input/22.txt")
        .split("\r\n\r\n")
        .map(|deck| deck.lines().skip(1).map(|line| line.parse().unwrap()).collect::<Deck>())
        .collect_tuple()
        .unwrap();
        
    for &recurse in &[false, true] {
        let winner = play(a.clone(), b.clone(), recurse).1;
        println!("Part {}: {:?}", recurse as u8 + 1, (1..=winner.len()).rev().zip(winner).map(|(i, j)| i * j).sum::<usize>());
    }
}

type Deck = Vec<usize>;

fn play(mut a: Deck, mut b: Deck, recurse: bool) -> (bool, Deck) {
    let mut played = std::collections::HashSet::new();

    loop {
        if a.is_empty() {
            return (false, b);
        } else if b.is_empty() || !played.insert((a.clone(), b.clone())) {
            return (true, a);
        } else {
            let (a0, b0) = (a.remove(0), b.remove(0));

            let awin = if recurse && a0 <= a.len() && b0 <= b.len() {
                play(a[..a0].to_vec(), b[..b0].to_vec(), true).0
            } else {
                 a0 > b0
            };

            if awin {
                a.extend(&[a0, b0]);
            } else {
                b.extend(&[b0, a0]);
            }
        }
    }
}
