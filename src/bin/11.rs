use itertools::Itertools;
use std::convert::TryInto;
use std::iter::successors;

type Direction = fn(&Grid, (usize, usize), (isize, isize)) -> Option<(usize, usize)>;
type Grid = Vec<Vec<u8>>;

fn main() {
    let grid: Grid = include_str!("../../input/11.txt")
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    println!("Part 1: {}", seated(grid.clone(), 4, |grid, s, n| steps(&grid, s, n).next()));
    println!("Part 2: {}", seated(grid.clone(), 5, |grid, s, n| steps(&grid, s, n).find(|&(x, y)| grid[y][x] != b'.')));
}

fn seated(start: Grid, limit: usize, direction: Direction) -> usize {
    successors(Some(start), |grid| round(&grid, limit, direction).filter(|next| next != grid))
        .last()
        .unwrap()
        .iter()
        .flat_map(|row| row.iter().filter(|&&b| b == b'#'))
        .count()
}

fn round(grid: &Grid, limit: usize, direction: Direction) -> Option<Grid> {
    Some(grid.iter().enumerate().map(|(y, row)|
        row.iter().enumerate().map(|(x, &b)| {
            let neighbours = (-1..=1).cartesian_product(-1..=1)
                .filter(|&step| step != (0, 0))
                .filter_map(|step| direction(grid, (x, y), step).filter(|&(x, y)| grid[y][x] == b'#'))
                .count();

            match b {
                b'L' if neighbours == 0 => b'#',
                b'#' if neighbours >= limit => b'L',
                _ => b
            }
        }).collect()
    ).collect())
}

fn steps(grid: &Grid, start: (usize, usize), step: (isize, isize)) -> impl Iterator<Item = (usize, usize)> {
    let size = (grid[0].len(), grid.len());

    successors(Some(start), move |&(x, y)| Some(((x as isize + step.0).try_into().ok()?, (y as isize + step.1).try_into().ok()?)))
        .skip(1)
        .take_while(move |&pos| pos.0 < size.0 && pos.1 < size.1)
}
