use collections::HashMap;
use pathfinding::astar;
use std::*;

fn neighbours(x: i32, y: i32, map: &HashMap<(i32, i32), i32>) -> Vec<((i32, i32), i32)> {
    let options = vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];

    options
        .into_iter()
        .filter(|p| map.contains_key(&p))
        .map(|p| (p, map[&p].clone()))
        .collect()
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to read input file!");
    let chars: Vec<char> = input.chars().collect();

    let mut map: HashMap<(i32, i32), i32> = Default::default();

    let MULTIPLIER = 5;
    let SIZE = input.chars().position(|c| c == '\n').unwrap() as i32;

    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                x = 0;
                y += 1;
            }
            other => {
                for mx in 0..MULTIPLIER {
                    for my in 0..MULTIPLIER {
                        let v = other as i32 - '0' as i32 + mx + my;

                        let value = if v > 9 {
                            (v % 10) + 1
                        } else {
                            v
                        };

                        map.insert(
                            (mx * SIZE + x.clone(), my * SIZE + y.clone()),
                            value
                        );
                    }
                }

                x += 1;
            }
        }
    }

    let GOAL: (i32, i32) = (MULTIPLIER * SIZE - 1, MULTIPLIER * SIZE - 1);

    println!("{:?}", GOAL);

    if let Some((found, cost)) = astar(
        &(0, 0),
        |&(x, y)| neighbours(x, y, &map),
        |(px, py)| (px - x).abs() + (py - y).abs(),
        |p| p == &GOAL,
    ) {
        for a in 0..GOAL.0 {
            for b in 0..GOAL.1 {
                if found.contains(&(b, a)) {
                    print!("\x1b[93m{}\x1b[0m", map[&(b,a)]);
                } else {
                    print!("{}", map[&(b,a)]);
                }
            }
            println!("")
        }

        println!("COST = {}", cost);
    }
}
