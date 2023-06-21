extern crate advent_code_2022;

use advent_code_2022::read_file::*;

fn calorie_per_elf(input: String) -> Vec<i64> {
    let mut result = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if let Ok(i) = line.parse::<i64>() {
            sum += i;
        } else {
            result.push(sum);
            sum = 0;
        }
    }
    result
}
fn main() {
    let file = read_day(1);
    let mut per_elf = calorie_per_elf(file);
    per_elf.sort_by(|a, b| b.cmp(a));
    println!("{}", per_elf[0]);
    println!("{}", per_elf.iter().take(3).sum::<i64>());
}
