use regex::Regex;
use advent_code_2022::read_file::read_day;

#[derive(Debug)]
struct Cycle{
    cycle: [i32; 1000],
    x: i32,
    current_cycle: usize,
    screen: [[char; 40]; 7],
}

impl Cycle {
    fn noop(&mut self){
        self.cycle();
    }

    fn cycle(& mut self){
        self.cycle[self.current_cycle] = self.x;
        let cycle = self.current_cycle - 1;
        let x_component = cycle % 40;
        let y_component = cycle / 40;
        let difference = x_component as i32 - self.x;
        if difference.abs() <= 1{
            self.screen[y_component][x_component] = '#';
        } else {
            self.screen[y_component][x_component] = '.';
        }
        self.current_cycle += 1;
    }
    fn add_x(&mut self, value: i32){
        self.cycle();
        self.cycle();
        self.x += value;
    }
}
fn main() {
    let file = read_day(10);
    let file = file.lines();
    // let file = "noop
    // addx 3
    // addx -5".lines();
//     let file = "addx 15
// addx -11
// addx 6
// addx -3
// addx 5
// addx -1
// addx -8
// addx 13
// addx 4
// noop
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx 5
// addx -1
// addx -35
// addx 1
// addx 24
// addx -19
// addx 1
// addx 16
// addx -11
// noop
// noop
// addx 21
// addx -15
// noop
// noop
// addx -3
// addx 9
// addx 1
// addx -3
// addx 8
// addx 1
// addx 5
// noop
// noop
// noop
// noop
// noop
// addx -36
// noop
// addx 1
// addx 7
// noop
// noop
// noop
// addx 2
// addx 6
// noop
// noop
// noop
// noop
// noop
// addx 1
// noop
// noop
// addx 7
// addx 1
// noop
// addx -13
// addx 13
// addx 7
// noop
// addx 1
// addx -33
// noop
// noop
// noop
// addx 2
// noop
// noop
// noop
// addx 8
// noop
// addx -1
// addx 2
// addx 1
// noop
// addx 17
// addx -9
// addx 1
// addx 1
// addx -3
// addx 11
// noop
// noop
// addx 1
// noop
// addx 1
// noop
// noop
// addx -13
// addx -19
// addx 1
// addx 3
// addx 26
// addx -30
// addx 12
// addx -1
// addx 3
// addx 1
// noop
// noop
// noop
// addx -9
// addx 18
// addx 1
// addx 2
// noop
// noop
// addx 9
// noop
// noop
// noop
// addx -1
// addx 2
// addx -37
// addx 1
// addx 3
// noop
// addx 15
// addx -21
// addx 22
// addx -6
// addx 1
// noop
// addx 2
// addx 1
// noop
// addx -10
// noop
// noop
// addx 20
// addx 1
// addx 2
// addx 2
// addx -6
// addx -11
// noop
// noop
// noop".lines();
    let add_x_regex = Regex::new(r"^\s*addx\s+(\S+)\s*").unwrap();
    let mut cycle = Cycle{cycle: [0; 1000], x: 1, current_cycle: 1, screen: [['.'; 40]; 7]};
    for line in file{
        if let Some(caps) = add_x_regex.captures(line){
            let value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            cycle.add_x(value);
            println!("addx {}", value);
        } else {
            cycle.noop();
            println!("noop")
        }
    }
    println!("{}", cycle.screen.into_iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
    let result = cycle.cycle[20] * 20 + cycle.cycle[60] * 60 + cycle.cycle[100] * 100
        + cycle.cycle[140] * 140 + cycle.cycle[180] * 180 + cycle.cycle[220] * 220;
    println!("{}", result);
}