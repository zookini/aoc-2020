fn main() {
    let mut program: Vec<(&str, i32)> = include_str!("../../input/8.txt")
        .lines()
        .map(|line| (&line[0..3], line[4..].parse().unwrap()))
        .collect();

    println!("Part 1: {}", run(&program).unwrap_err());

    for patch in 0.. {
        if let Some(mut mnemonic) = flip(program[patch].0) {
            std::mem::swap(&mut program[patch].0, &mut mnemonic);
            
            if let Ok(acc) = run(&program) {
                println!("Part 2: {}", acc);
                break;
            }

            std::mem::swap(&mut program[patch].0, &mut mnemonic);
        }
    }
}

fn run(program: &[(&str, i32)]) -> Result<i32, i32> {
    let mut ip = 0;
    let mut acc = 0;
    let mut visited = vec![false; program.len()];

    loop {
        if ip == program.len() {
            return Ok(acc)
        } else if visited[ip] {
            return Err(acc)
        } 

        visited[ip] = true;

        match program[ip] {
            ("nop", _) => (),
            ("acc", i) => acc += i,
            ("jmp", i) => ip = (ip as i32 + i - 1) as usize,
            _ => unreachable!()
        }

        ip += 1;
    }
}

fn flip(mnemonic: &str) -> Option<&str> {
    match mnemonic {
        "nop" => Some("jmp"),
        "jmp" => Some("nop"),
        _ => None
    }
}
