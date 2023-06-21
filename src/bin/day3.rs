use advent_code_2022::read_file::read_day;
use std::ops::Div;

fn get_value(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}
fn get_priority(s: &str) -> i64 {
    let mut array = [false; 64];
    let (first_half, second_half) = s.split_at(s.len().div(2));
    println!("{} {}", first_half, second_half);
    for letter in first_half.chars() {
        array[get_value(letter) as usize] = true
    }
    for letter in second_half.chars() {
        if array[get_value(letter) as usize] == true {
            return get_value(letter) as i64;
        }
    }
    0
}
fn bitwise_it(s: &str) -> i64 {
    let mut result = 0;
    for i in s.chars() {
        let value = get_value(i) - 1;
        result |= (1 << value)
    }
    result
}
fn main() {
    let file = read_day(3);
    let array = [false; 64];
    let result = file.lines().map(|x| get_priority(x)).sum::<i64>();
    println!("{}", result);
    let mut result2 = 0;
    let mut iter = file.lines();
    loop {
        let first = match iter.next() {
            None => break,
            Some(item) => item,
        };
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        let r = bitwise_it(first) & bitwise_it(second) & bitwise_it(third);
        result2 += r.trailing_zeros() + 1
    }
    println!("{}", result2)
}
