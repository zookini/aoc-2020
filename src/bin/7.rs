use itertools::Itertools;

type Tree<'a> = std::collections::HashMap<&'a str, Vec<(usize, &'a str)>>;

fn main() {
    let child = regex::Regex::new(r"(\d) (\w+ \w+)").unwrap();

    let tree: Tree = include_str!("../../input/7.txt")
        .lines()
        .map(|line| line.split(" bags contain ").collect_tuple().unwrap())
        .map(|(parent, children)| (parent, child
            .captures_iter(children)
            .map(|caps| (caps[1].parse().unwrap(), caps.get(2).unwrap().as_str()))
            .collect()
        ))
        .collect();

    println!("Part 1: {:?}", tree.keys().filter(|k| p1(&tree, k)).count());
    println!("Part 2: {:?}", p2(&tree, "shiny gold") - 1);
}

fn p1(tree: &Tree, parent: &str) -> bool {
    tree[parent].iter().any(|&(_, child)| child == "shiny gold" || p1(tree, child))
}

fn p2(tree: &Tree, parent: &str) -> usize {
    tree[parent].iter().map(|(count, child)| count * p2(tree, child)).sum::<usize>() + 1
}
