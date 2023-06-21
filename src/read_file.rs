use std::fmt::format;
use std::fs;
use std::path::Path;

pub fn read_day(day: i64) -> String {
    let s = format!(r"./src/input/day{}.txt", day);
    let path: &Path = Path::new(&s);
    fs::read_to_string(path).expect(&*format!("path: {:?} does not exist", path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_day() {
        let day1 = read_day(1);
        println!("{}", day1);
    }
}
