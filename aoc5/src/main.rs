use std::cmp;
use std::env;
use std::fmt::Display;
use std::fs;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i16,
    y: i16,
}

struct Vector {
    a: Point,
    b: Point,
}

impl Point {
    fn parse(raw: &str) -> Option<Point> {
        let parts: Vec<i16> = raw
            .trim()
            .split(',')
            .map(|r| r.parse::<i16>().unwrap())
            .collect();

        if parts.len() != 2 {
            println!(
                "Wrong number of parts to create point! Given: {}, expected 2",
                parts.len()
            );

            None
        } else {
            Some(Point {
                x: parts[0],
                y: parts[1],
            })
        }
    }

    fn new(x: i16, y: i16) -> Point {
        Point { x: x, y: y }
    }

    fn move_by_delta(&self, delta: &Point) -> Point {
        Point {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }
}

impl Vector {
    fn is_vertical_or_horizontal(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    fn is_diagonal(&self) -> bool {
        (self.a.x as i32 - self.b.x as i32).abs() == (self.a.y as i32 - self.b.y as i32).abs()
    }

    fn is_vhd(&self) -> bool {
        self.is_vertical_or_horizontal() || self.is_diagonal()
    }

    fn points(&self) -> Vec<Point> {
        let ax = self.a.x;
        let ay = self.a.y as i16;

        let bx = self.b.x;
        let by = self.b.y;

        /*

                o (x0,y0)
                 \
                  \
                   o (x1,y1)
        */

        let dx = || ((bx - ax) / (bx - ax).abs());
        let dy = || ((by - ay) / (by - ay).abs());

        let delta = if ax == bx {
            Point { x: 0, y: dy() }
        } else if ay == by {
            Point { x: dx(), y: 0 }
        } else {
            Point { x: dx(), y: dy() }
        };

        let start = Point::new(ax, ay);
        let target = Point::new(bx, by);
        let mut current = start.clone();
        let mut result: Vec<Point> = vec![];

        // Simplified to go over V or H
        loop {
            let new = current.move_by_delta(&delta);

            if new == target {
                break;
            }

            current = new.clone();
            result.push(new);
        }

        result.push(start);
        result.push(target);

        println!("{}: {:?}", self, result);

        result
    }

    fn parse(raw: &str) -> Vector {
        let parts: Vec<&str> = raw.split("->").collect();

        Vector {
            a: Point::parse(parts[0]).expect("Unable to create part A of Vector"),
            b: Point::parse(parts[1]).expect("Unable to create part B of Vector"),
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} -> {}", self.a, self.b)
    }
}

const BOARD_SIZE: usize = 1000;

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect(&"Unable to read input file!");
    let lines: Vec<&str> = input.lines().collect();

    let vectors: Vec<Vector> = lines
        .iter()
        .map(|r| Vector::parse(r))
        .filter(|v| v.is_vhd())
        .collect();

    let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];

    vectors
        .iter()
        .flat_map(|v| v.points())
        .for_each(|p| board[p.x as usize][p.y as usize] += 1);

    let mut result = 0;
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if BOARD_SIZE < 50 {
                if board[x][y] == 0 {
                    print!(".")
                } else {
                    print!("{}", board[x][y])
                }
            }
            // Calculating result
            if board[x][y] >= 2 {
                result += 1;
            }
        }
        if BOARD_SIZE < 50 {
            println!();
        }
    }

    println!("Result = {}", result);
}
