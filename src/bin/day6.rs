use advent_code_2022::bitwise::{CharSet, get_value, get_value_shifted};
use advent_code_2022::read_file::read_day;
use itertools::Itertools;

fn main() {
    let file = read_day(6);
    let iter = file.chars().map(|x| get_value_shifted(x) ).collect::<Vec<_>>();
    let pos = iter.windows(4).position(
        |w| {
            println!("{:?}", w);
            let mut value: CharSet = 0;
            for i in w{
                value |= *i
            }
            println!("{:b}", value);
            value.count_ones() == 4
        }
    ).unwrap();
    let pos2 = iter.windows(14).position(
        |w| {
            println!("{:?}", w);
            let mut value: CharSet = 0;
            for i in w{
                value |= *i
            }
            println!("{:b}", value);
            value.count_ones() == 14
        }
    ).unwrap();
    println!("{}", pos2 + 14);
    println!("{}", pos + 4);
}