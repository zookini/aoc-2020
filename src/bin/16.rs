use aoc::regex;
use itertools::Itertools;

fn main() {
    let (fields, ticket, nearby): (Vec<Field>, Ticket, Vec<Ticket>) = include_str!("../../input/16.txt")
        .split("\r\n\r\n")
        .collect_tuple()
        .map(|(fields, ticket, nearby)| (
            fields.lines().map(|line| Field::parse(line)).collect(),
            ticket.lines().skip(1).map(|line| parse_ticket(line)).next().unwrap(),
            nearby.lines().skip(1).map(|line| parse_ticket(line)).collect(),
        ))
        .unwrap();

    println!("Part 1 {}", nearby.iter().flatten().filter(|v| !fields.iter().any(|f| f.contains(v))).sum::<usize>());

    let valid: Vec<Ticket> = nearby
        .into_iter()
        .filter(|ticket| ticket
            .iter()
            .all(|v| fields.iter().any(|field| field.contains(v)))
        )
        .collect();

    let mut taken = std::collections::HashSet::new();

    println!("Part 2: {}", fields
        .iter()
        .map(|field| (
            field.name,
            (0..valid[0].len()).filter(|&i| valid.iter().all(|t| field.contains(&t[i]))).collect::<Vec<_>>()
        ))
        .sorted_by_key(|(_, columns)| columns.len())
        .map(|(field, columns)| (field, columns.into_iter().find(|&c| taken.insert(c)).unwrap()))
        .filter(|(field, _)| field.starts_with("departure"))
        .fold(1, |product, (_, column)| product * ticket[column])
    );
}

struct Field<'a> {
    name: &'a str,
    ranges: Vec<std::ops::RangeInclusive<usize>>,
}

impl<'a> Field<'a> {
    fn parse(field: &'a str) -> Self {
        field
            .split(": ")
            .collect_tuple()
            .map(|(name, ranges)| Self {
                name,
                ranges: regex!(r"(\d+)-(\d+)")
                    .captures_iter(ranges)
                    .map(|caps| caps[1].parse().unwrap()..=caps[2].parse().unwrap())
                    .collect()
            })
            .unwrap()
    }

    fn contains(&self, value: &usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

type Ticket = Vec<usize>;

fn parse_ticket(ticket: &str) -> Ticket {
    ticket.split(',').map(|s| s.parse().unwrap()).collect()
}
