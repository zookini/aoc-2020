fn main() {
    println!("Part 1: {}", run(false));
    println!("Part 2: {}", run(true));
}

fn run(precedence: bool) -> usize {
    include_str!("../../input/18.txt")
        .lines()
        .map(|line| Exp { scanner: line.chars().filter(|&c| c != ' ').peekable(), precedence }.eval())
        .sum()
}

struct Exp<I: Iterator<Item = char>> {
    scanner: std::iter::Peekable<I>,
    precedence: bool,
}

impl<I: Iterator<Item = char>> Exp<I> {
    fn eval(&mut self) -> usize {
        let mut val = self.term();
    
        loop {
            match self.scanner.next() {
                Some('+') => val += self.term(),
                Some('*') => val *= if self.precedence { self.add() } else { self.term() },
                _ => return val
            }
        }
    }
    
    fn add(&mut self) -> usize {
        let mut val = self.term();
    
        loop {
            match self.scanner.peek() {
                Some('+') => {
                    self.scanner.next();
                    val += self.term();
                }
                _ => return val
            }
        }
    }
    
    fn term(&mut self) -> usize {
        match self.scanner.next() {
            Some('(') => self.eval(),
            Some(num) => num.to_digit(10).unwrap() as usize,
            _ => unreachable!()
        }
    }
}
