use aoc::regex;
use itertools::Itertools;

fn main() {
    let tiles: Vec<Tile> = include_str!("../../input/20.txt").split("\r\n\r\n").map(|s| Tile::parse(s)).collect();

    let corner = tiles
        .iter()
        .find_map(|tile| tile
            .orientations()
            .find(|a| !tiles.iter().find_map(|b| a.connect(&b, |a, b| a.top() == b.bottom() || a.left() == b.right())).is_some())
        )
        .unwrap();

    let mut image: Vec<Vec<Tile>> = vec![vec![corner]];
    let dim = (tiles.len() as f64).sqrt() as usize;

    for i in 0..dim {
        if i != 0 {
            tiles.iter().find_map(|tile| image[i - 1][0].connect(&tile, |a, b| a.bottom() == b.top())).map(|tile| image.push(vec![tile]));
        }

        for j in 0..dim - 1 {
            tiles.iter().find_map(|tile| image[i][j].connect(&tile, |a, b| a.right() == b.left())).map(|tile| image[i].push(tile));
        }
    }

    println!("Part 1 {}", image[0][0].id * image[image.len() - 1][0].id * image[0][image.len() - 1].id * image[image.len() - 1][image.len() - 1].id);

    let borderless: Vec<String> = image
        .iter()
        .flat_map(|row| {
            let row: Vec<Vec<String>> = row
                .iter()
                .map(|tile| tile.show().lines().skip(1).take(8).map(|line| line[1..line.len() - 1].into()).collect())
                .collect();

            (0..8).map(move |i| (0..row.len()).map(|tile| &row[tile][i]).join(""))
        })
        .collect();

    let dragons = orientations(&borderless).map(|orientation| count_dragons(&orientation)).find(|&dragons| dragons > 0).unwrap();

    println!("Part 2 {}", borderless.iter().flat_map(|s| s.chars().filter(|&c| c == '#')).count() - dragons * 15);
}

fn count_dragons(image: &[String]) -> usize {
    image
        .windows(3)
        .flat_map(|w| regex!("#....##....##....###")
            .find_iter(&w[1])
            .filter(move |m| regex!(".#..#..#..#..#..#...").is_match(&w[2][m.start()..m.start() + 20]))
            .filter(move |m| regex!("..................#.").is_match(&w[0][m.start()..m.start() + 20]))
        )
        .count()
}

fn orientations(image: &[String]) -> impl Iterator<Item = Vec<String>> {
    std::iter::successors(Some(image.to_vec()), |orient| Some(rotate(&orient)))
        .take(4)
        .flat_map(|rotation| vec![fliph(&rotation), flipv(&rotation), rotation])
}

fn fliph(image: &[String]) -> Vec<String> {
    image.iter().cloned().rev().collect()
}

fn flipv(image: &[String]) -> Vec<String> {
    image.iter().map(|line| line.chars().rev().collect()).collect()
}

fn rotate(image: &[String]) -> Vec<String> {
    (0..image[0].len()).map(|i| (0..image.len()).rev().map(|j| image[j].as_bytes()[i] as char).collect()).collect()
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    id: usize,
    grid: u128,
}

impl Tile {
    fn new(id: usize, grid: u128) -> Self {
        Self { id, grid }
    }

    fn parse(tile: &str) -> Self {
        Self::new(
            tile[5..9].parse::<usize>().unwrap(),
            tile[10..].chars().filter(|&c| "#.".contains(c)).fold(0, |grid, c| grid * 2 + (c == '#') as u128),
        )
    }

    fn top(&self) -> u16 {
        (self.grid >> 90) as u16
    }

    fn bottom(&self) -> u16 {
        (self.grid & 0b1111111111) as u16
    }

    fn left(&self) -> u16 {
        (9..100).step_by(10).rev().fold(0, |side, i| side * 2 + ((self.grid >> i) & 1)) as u16
    }

    fn right(&self) -> u16 {
        (0..91).step_by(10).rev().fold(0, |side, i| side * 2 + ((self.grid >> i) & 1)) as u16
    }

    fn fliph(&self) -> Self {
        Self::new(self.id, (0..100).step_by(10).fold(0, |grid, i| grid << 10 | self.grid >> i & 0b1111111111))
    }

    fn flipv(&self) -> Self {
        Self::new(self.id, self.grid.reverse_bits() >> 28).fliph()
    }

    fn rotate(&self) -> Self {
        Self::new(self.id, (0..10).rev().cartesian_product((0..100).step_by(10)).fold(0, |grid, (i, j)| grid << 1 | self.grid >> i + j & 1))
    }

    fn orientations(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(*self), |orient| Some(orient.rotate()))
            .take(4)
            .flat_map(|rotation| vec![rotation, rotation.fliph(), rotation.flipv()])
    }

    fn connect(&self, other: &Self, p: fn(&Self, &Self) -> bool) -> Option<Self> {
        if self.id == other.id {
            None
        } else {
            other.orientations().find(|orientation| p(self, orientation))
        }
    }

    fn show(&self) -> String {
        (0..100).step_by(10).rev().map(|i| show(((self.grid >> i) & 0b1111111111) as u16)).join("\n")
    }
}

fn show(row: u16) -> String {
    (0..10).rev().map(|i| if row >> i & 1 == 1 { '#' } else { '.' }).collect()
}
