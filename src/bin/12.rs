fn main() {
    let commands: Vec<(char, isize)> = include_str!("../../input/12.txt")
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect();

    let ship = commands
        .iter()
        .fold(((0, 0), 'E'), |(ship, facing), &(command, distance)| match if command == 'F' { facing } else { command } {
            'L' => (ship, turn(facing, 360 - distance)),
            'R' => (ship, turn(facing, distance)),
            cmd => (go(ship, cmd, distance), facing)
        }).0;

    println!("Part 1: {}", ship.0.abs() + ship.1.abs());

    let ship = commands
        .iter()
        .fold(((0, 0), (10, 1)), |(ship, waypoint), &(command, distance)| match command {
            'L' => (ship, rotate(waypoint, 360 - distance)),
            'R' => (ship, rotate(waypoint, distance)),
            'F' => ((ship.0 + waypoint.0 * distance, ship.1 + waypoint.1 * distance), waypoint),
            cmd => (ship, go(waypoint, cmd, distance))
        }).0;

    println!("Part 2: {}", ship.0.abs() + ship.1.abs());
}

fn go((x, y): (isize, isize), command: char, distance: isize) -> (isize, isize) {
    match command {
        'N' => (x, y + distance),
        'S' => (x, y - distance),
        'E' => (x + distance, y),
        'W' => (x - distance, y),
        _ => unreachable!(),
    }
}

fn rotate((x, y): (isize, isize), degrees: isize) -> (isize, isize) {
    (0..degrees / 90).fold((x, y), |(x, y), _| (y, -x))
}

const DIRS: &[char] = &['N', 'E', 'S', 'W'];

fn turn(facing: char, degrees: isize) -> char {
    DIRS[(DIRS.iter().position(|&c| c == facing).unwrap() + (degrees / 90) as usize) % DIRS.len()]
}
