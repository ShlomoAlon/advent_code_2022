use crate::Move::{Paper, Rock, Scissors};
use advent_code_2022::read_file::read_day;

#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn point_value(self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn win_value(self, other: Self) -> i64 {
        let (first, second) = (self.point_value(), other.point_value());
        if first == second {
            3
        } else if first == second + 1 || (second == 3 && first == 1) {
            6
        } else {
            0
        }
    }
}

fn parse_line2(s: &str) -> (Move, Move) {
    let first = match s.chars().nth(0).unwrap() {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!("be one of these values"),
    };

    let second = match s.chars().nth(2).unwrap() {
        'X' => match first {
            Move::Rock => Scissors,
            Move::Paper => Rock,
            Move::Scissors => Paper,
        },
        'Y' => first,
        'Z' => match first {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        _ => panic!("should be one of those values"),
    };

    (first, second)
}

fn parse_line(s: String) -> (Move, Move) {
    let first = match s.chars().nth(0).unwrap() {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!("be one of these values"),
    };

    let second = match s.chars().nth(2).unwrap() {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!("should be one of these values"),
    };
    (first, second)
}

fn line_value(first: Move, second: Move) -> i64 {
    second.point_value() + second.win_value(first)
}

fn main() {
    let file = read_day(2);
    let mut result1 = 0;
    let mut result2 = 0;
    for line in file.lines() {
        let (first, second) = parse_line(line.to_string());
        result1 += line_value(first, second);
        let (first, second) = parse_line2(line);
        result2 += line_value(first, second);
    }
    println!("q1: {} q2: {}", result1, result2)
}
