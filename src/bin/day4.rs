use advent_code_2022::read_file::read_day;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Range {
    low: i64,
    high: i64,
}
impl Range {
    fn contains(self, other: Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }
    fn overlaps(self, other: Range) -> bool {
        (self.low <= other.low && self.high >= other.low)
            || (self.low <= other.high && self.high >= other.low)
    }
}
impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once('-').unwrap();
        let low = low.parse::<i64>()?;
        let high = high.parse::<i64>()?;
        Ok(Range { low, high })
    }
}
fn parse_line(s: &str) -> (Range, Range) {
    let (beg, end) = s.split_once(',').unwrap();
    (beg.parse().unwrap(), end.parse().unwrap())
}
fn main() {
    let file = read_day(4);
    let result1 = file
        .lines()
        .filter(|x| {
            let (high, low) = parse_line(x);
            let result = high.contains(low) || low.contains(high);
            result
        })
        .count();
    println!("{}", result1);
    let result2 = file
        .lines()
        .filter(|x| {
            let (high, low) = parse_line(x);
            let result = high.overlaps(low) || low.overlaps(high);
            result
        })
        .count();
    println!("{}", result2)
}
