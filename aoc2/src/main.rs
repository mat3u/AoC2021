use itertools::Itertools;
use std::env;
use std::fs;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn parse(raw: &str) -> Direction {
        let (op, v) = raw.splitn(2, ' ').collect_tuple().unwrap();
        let val: i32 = v.parse::<i32>().unwrap();

        // println!("OP = {}; VAL={}", op, val);

        match op.as_ref() {
            "forward" => Direction::Forward(val),
            "down" => Direction::Down(val),
            "up" => Direction::Up(val),
            _ => panic!("Unknown value: [{:?}]!", op),
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("./sample.txt".to_string());

    let input = fs::read_to_string(path).expect("Unable to read file!");

    let directions = input.lines().map(Direction::parse);

    let mut horiz = 0;
    let mut aim = 0;
    let mut depth = 0;

    for d in directions {
        match d {
            Direction::Down(v) => aim = aim + v,
            Direction::Up(v) => aim = aim - v,
            Direction::Forward(v) => {
                horiz = horiz + v;
                depth = depth + aim * v;
            }
        }
    }

    println!("{}", horiz*depth);
}
