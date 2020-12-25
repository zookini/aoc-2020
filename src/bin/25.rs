fn main() {
    println!("{}", encryption_key(15086442, loops(7, 15335876)));
}

fn loops(sn: usize, key: usize) -> usize {
    rounds(sn).enumerate().find(|&(_, k)| k == key).unwrap().0
}

fn encryption_key(sn: usize, loops: usize) -> usize {
    rounds(sn).nth(loops).unwrap()
}

fn rounds(sn: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(1), move |&k| Some(k * sn % 20201227))
}
