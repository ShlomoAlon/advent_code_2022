
use advent_code_2022::read_file::read_day;
#[derive(Debug)]
struct Loc{
    value: i32,
    seen: bool,
    see_value: i32
}
struct HeightStack{
    stack: [usize; 10]
}

impl HeightStack{
    fn check(& mut self, height: i32, index: usize) -> i32{
        let result = index - self.stack[height as usize];
        for i in 0..height + 1{
            self.stack[i as usize] = index;
        }
        result as i32
    }
}

fn main() {
    let file = read_day(8);
    let file = file.lines();
    let mut v = file
        .map(|line| {
            line.chars()
                .map(|c| Loc{ value: c.to_digit(10).unwrap() as i32, seen: false, see_value: 1})
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{:?}", v);
    let len = v[0].len() - 1;
    for y in 0..v.len(){
        let mut height_stack = HeightStack{stack: [0; 10]};
        let mut min = -1;
        for x in 0..v[y].len(){
            let mut cur = &mut v[y][x];
            if cur.value > min{
                cur.seen = true;
                min = cur.value;
            }
            cur.see_value *= height_stack.check(cur.value, x);
        }
        let mut min = -1;
        let mut height_stack = HeightStack{stack: [0; 10]};
        for x in (0..v[y].len()).rev(){
            let mut cur = &mut v[y][x];
            if cur.value > min{
                cur.seen = true;
                min = cur.value;
            }
            cur.see_value *= height_stack.check(cur.value, len - x);
        }
    }
    let len = v.len() - 1;
    for x in 0..v[0].len(){
        let mut height_stack = HeightStack{stack: [0; 10]};
        let mut min = -1;
        for y in 0..v.len(){
            let mut cur = &mut v[y][x];
            if cur.value > min{
                cur.seen = true;
                min = cur.value;
            }
            cur.see_value *= height_stack.check(cur.value, y);
        }
        let mut min = -1;
        let mut height_stack = HeightStack{stack: [0; 10]};
        for y in (0..v.len()).rev(){
            let mut cur = &mut v[y][x];
            if cur.value > min{
                cur.seen = true;
                min = cur.value;
            }
            cur.see_value *= height_stack.check(cur.value, len - y);
        }

    }
    let mut count = 0;
    let mut max = 0;
    for row in v{
        for col in row{
            if col.seen {
                count += 1;
            }
            max = max.max(col.see_value);
        }
    }
    println!("{}", count);
    println!("{}", max);
}
