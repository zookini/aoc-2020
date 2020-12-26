use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let (mut rules, messages) = include_str!("../../input/19.txt")
        .split("\r\n\r\n")
        .collect_tuple()
        .map(|(rules, messages)| (parse(rules), messages))
        .unwrap();

    println!("Part 1: {}", messages.lines().filter(|message| matches(&rules, vec![0], message)).count());

    rules.insert(8, Rule::parse("42 | 42 8"));
    rules.insert(11, Rule::parse("42 31 | 42 11 31"));

    println!("Part 2: {}", messages.lines().filter(|message| matches(&rules, vec![0], message)).count());
}

fn parse(rules: &str) -> HashMap<usize, Rule> {
    rules
        .lines()
        .flat_map(|line| line.split(": ").collect_tuple().map(|(num, rule)| (num.parse().unwrap(), Rule::parse(rule))))
        .collect()
}

enum Rule { Literal(char), Nums(Vec<Vec<usize>>) }

impl Rule {
    fn parse(rule: &str) -> Self {
        if rule.starts_with("\"") {
            Self::Literal(rule.chars().nth(1).unwrap())
        } else {
            Self::Nums(rule.split(" | ").map(|nums| nums.split(" ").flat_map(|n| n.parse()).collect()).collect())
        }
    }
}

fn matches(rules: &HashMap<usize, Rule>, mut stack: Vec<usize>, s: &str) -> bool {
    match stack.pop().map(|num| &rules[&num]) {
        Some(Rule::Literal(l)) => s.chars().next().filter(|c| c == l).is_some() && matches(rules, stack, &s[1..]),
        Some(Rule::Nums(nums)) => nums.iter().any(|ns| matches(rules, stack.iter().chain(ns.iter().rev()).copied().collect(), s)),
        None => s.is_empty(),
    }
}
