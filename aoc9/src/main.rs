use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: i16,
    y: i16,
}

trait Queries {
    fn is_below(&self, point: &Point, dx: i16, dy: i16) -> bool;
}

impl Queries for HashMap<Point, i32> {
    fn is_below(&self, point: &Point, dx: i16, dy: i16) -> bool {
        let this = self.get(point).expect("This must exists!");
        let new = Point {
            x: point.x + dx,
            y: point.y + dy,
        };

        self.get(&new).map(|v| v > this).unwrap_or(true)
    }
}

fn find_basin(map: &HashMap<Point, i32>, point: &Point) -> Vec<Point> {
    let mut basin = vec![Point {
        x: point.x,
        y: point.y,
    }];

    //println!("{:?}", basin);

    loop {
        let bb = &basin.clone();
        for p in bb {
            let base_depth = map[&p];

            //println!("S: {:?} -> {}", p, base_depth);

            for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = Point {
                    x: p.x + dx,
                    y: p.y + dy,
                };

                match map.get(&next) {
                    Some(depth) => {
                        //println!("{:?} -> {}", next, depth);
                        if (depth - base_depth).abs() <= 1 && depth != &9 && !basin.contains(&next) {
                            basin.push(next);
                        }
                    }
                    _ => (),
                }
            }
        }

        //println!("{:?}", basin);
        if basin.len() == bb.len() {
            break;
        }
    }

    basin.dedup();

    basin
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to open a file!");
    let chars = input.chars();

    let mut map: HashMap<Point, i32> = Default::default();

    let mut x: i16 = 0;
    let mut y: i16 = 0;
    for c in chars {
        if c == '\n' {
            y += 1;
            x = 0;
        } else {
            let point = Point { x, y };
            map.insert(point, c as i32 - '0' as i32);

            x += 1;
        }
    }

    let mut result: Vec<Point> = Default::default();

    for x1 in 0..x{
        for y1 in 0..y + 1 {
            let point = Point { x: x1, y: y1 };

            if map.is_below(&point, -1, 0)
                && map.is_below(&point, 1, 0)
                && map.is_below(&point, 0, -1)
                && map.is_below(&point, 0, 1)
            {
                result.push(point);
            }
        }
    }

    let mut sum = 0;
    for r in &result {
        sum += map[&r] + 1;
    }

    println!("PART 1 = {}", sum);

    // Part 2

    let mut basins: Vec<Vec<Point>> = Default::default();
    for point in &result {
        let basin = find_basin(&map, point);

        basins.push(basin);
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    for basin in &basins[0..3] {
        // Why plus one ?
        println!("{}", basin.len() + 1);
    }

    let result3 = basins.into_iter().take(3).map(|b| b.len() + 1).fold(1, |z,v| v * z);

    println!("{}", result3);
}
