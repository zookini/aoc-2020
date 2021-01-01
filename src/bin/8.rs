fn main() {
    let mut program: Vec<(&str, i32)> = include_str!("../../input/8.txt")
        .lines()
        .map(|line| (&line[0..3], line[4..].parse().unwrap()))
        .collect();

    println!("Part 1: {}", run(&program).unwrap_err());

    println!("Part 2: {}", (0..)
        .find_map(|i| match program[i].0 {
                "nop" => Some("jmp"),
                "jmp" => Some("nop"),
                _ => None
            }
            .and_then(|mut mnemonic| {
                std::mem::swap(&mut program[i].0, &mut mnemonic);
                run(&program).map_err(|_| std::mem::swap(&mut program[i].0, &mut mnemonic)).ok()
            })
        )
        .unwrap()
    );
}

fn run(program: &[(&str, i32)]) -> Result<i32, i32> {
    let mut ip = 0;
    let mut acc = 0;
    let mut visited = std::collections::HashSet::new();

    loop {
        if ip == program.len() {
            return Ok(acc)
        } else if !visited.insert(ip) {
            return Err(acc)
        } 

        match program[ip] {
            ("nop", _) => (),
            ("acc", i) => acc += i,
            ("jmp", i) => ip = (ip as i32 + i - 1) as usize,
            _ => unreachable!()
        }

        ip += 1;
    }
}
