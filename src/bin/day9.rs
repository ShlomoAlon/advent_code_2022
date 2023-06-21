use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use regex::Regex;
use advent_code_2022::read_file::read_day;
struct Board{
    board: HashSet<(i32, i32)>,
    head: (i32, i32),
    tail: (i32, i32),
}

struct Board2{
    board: HashSet<(i32, i32)>,
    body: [(i32, i32); 10],

}
impl Board2{
    fn move_head(&mut self, direction: &str){
        match direction{
            "U" => self.body[0].1 += 1,
            "D" => self.body[0].1 -= 1,
            "R" => self.body[0].0 += 1,
            "L" => self.body[0].0 -= 1,
            _ => panic!("Invalid direction")
        }
    }
    fn move_body(&mut self){
        for i in 1..self.body.len(){
            let segment = self.body[i];
            let parent = self.body[i - 1];
            let x_dir = parent.0 - segment.0;
            let y_dir = parent.1 - segment.1;
            if x_dir.abs() > 1 || y_dir.abs() > 1{
                self.body[i].0 += x_dir.signum();
                self.body[i].1 += y_dir.signum();
            }
        }
        self.board.insert(self.body[self.body.len() - 1]);
    }
}


impl Board{
    fn move_head(&mut self, direction: &str){
        match direction{
            "U" => self.head.1 += 1,
            "D" => self.head.1 -= 1,
            "R" => self.head.0 += 1,
            "L" => self.head.0 -= 1,
            _ => panic!("Invalid direction")
        }
    }
    fn update_tail(&mut self){
        let x_dir = self.head.0 - self.tail.0;
        let y_dir = self.head.1 - self.tail.1;
        if x_dir.abs() > 1 || y_dir.abs() > 1{
            self.tail.0 += x_dir.signum();
            self.tail.1 += y_dir.signum();
            self.board.insert(self.tail);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = [[' '; 10]; 10];
        for (x, y) in &self.board{
            board[*y as usize][*x as usize] = 'X';
        }
        board[self.head.1 as usize][self.head.0 as usize] = 'O';
        board[self.tail.1 as usize][self.tail.0 as usize] = 'T';
        board.into_iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n").fmt(f)
    }
}


fn main() {
    let file = read_day(9);
    let file = file.lines();
//     let file = "R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2".lines();
    let regex = Regex::new(r"^(.) (\d+)$").unwrap();
    let mut board = Board{board: HashSet::from([(0, 0)]), head: (0, 0), tail: (0, 0)};
    let mut board2 = Board2{board: HashSet::from([(0, 0)]), body: [(0, 0); 10]};
    for line in file{
        let caps = regex.captures(line).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        for _ in 0..value{
            board.move_head(direction);
            board.update_tail();
            board2.move_head(direction);
            board2.move_body();
        }
    }
    println!("{:?}", board.board.len());
    println!("{:?}", board2.board.len());

}

