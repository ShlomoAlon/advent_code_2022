use std::collections::VecDeque;
use advent_code_2022::parser::Parser;
use advent_code_2022::read_file::read_day;

#[derive(Debug, Clone)]
struct Stack {
    rows: Vec<Vec<char>>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            rows: (0..9).map(|_| Vec::new()).collect(),
        }
    }
    fn add_to_row(&mut self, row: usize, c: char) {
        self.rows[row].push(c)
    }
    fn move_one_to(&mut self, from: usize, to: usize) {
        let f = self.rows[from - 1].pop().unwrap();
        self.rows[to - 1].push(f)
    }
    fn move_to(&mut self, from: usize, to: usize, amount: usize) {
        for _ in 0..amount {
            self.move_one_to(from, to)
        }
    }
    fn move_to_two(& mut self, from: usize, to: usize, amount: usize){
        let mut v = Vec::new();
        for _ in (0 .. amount){
            v.push(self.rows[from - 1].pop().unwrap())
        }
        while !v.is_empty(){
            self.rows[to - 1].push(v.pop().unwrap())
        }

    }
    fn get_tops(& self) -> String{
        self.rows.iter().map(|x| x[x.len() - 1]).collect()
    }
}
fn main() {
    let file = read_day(5);
    let (first_half, second_half) = file.split_once("\n\n").unwrap();
    let mut first_half = first_half.lines().rev();
    let mut second_half = second_half.lines();
    first_half.next().unwrap();
    let mut stack = Stack::new();
    for line in first_half {
        for i in 0..9 {
            let index = 1 + i * 4;
            let v = line.chars().nth(index).unwrap();
            if !v.is_whitespace() {
                stack.add_to_row(i, v);
            }
        }
    }
    let mut stack2 = stack.clone();
    for line in second_half {
        let mut parser = Parser::new(line);
        let moved_digit = parser.parse_next_digit().unwrap();
        let from_digit = parser.parse_next_digit().unwrap();
        let to_digit = parser.parse_next_digit().unwrap();
        stack.move_to(from_digit as usize, to_digit as usize, moved_digit as usize);
        stack2.move_to_two(from_digit as usize, to_digit as usize, moved_digit as usize);
    }

    println!("{:?}", stack.get_tops());
    println!("{:?}", stack2.get_tops());
}
