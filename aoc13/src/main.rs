use std::collections::HashSet;
use std::*;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to read a file!");

    let mut map: HashSet<Point> = Default::default();
    let mut folds: Vec<Fold> = Default::default();

    // Parse input
    let mut is_map = true;
    for line in input.split('\n') {
        if line.trim() == "" {
            is_map = false;
            continue;
        }

        if is_map {
            let parts: Vec<&str> = line.split(",").collect();

            assert_eq!(parts.len(), 2);

            let x = parts[0]
                .parse::<_>()
                .expect(format!("Unable to parse X from [{}]", parts[0]).as_str());
            let y = parts[1]
                .parse::<_>()
                .expect(format!("Unable to parse Y from [{}]", parts[1]).as_str());

            map.insert(Point { x, y });
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            assert_eq!(parts.len(), 3);

            let parts2: Vec<&str> = parts[2].split('=').collect();
            assert_eq!(parts2.len(), 2);

            let value = parts2[1]
                .parse::<_>()
                .expect(format!("Unable to parse fold value from [{}]", parts2[1]).as_str());

            let fold = if parts2[0] == "x" {
                Fold::X(value)
            } else {
                Fold::Y(value)
            };

            folds.push(fold);
        }
    }

    println!("{}", map.len());

    // Make folds
    for fold in &folds {
        let mut new_map: HashSet<Point> = Default::default();

        match fold {
            Fold::X(value) => {
                for point in &map {
                    let new_point = if &point.x > value {
                        Point {
                            x: value - (point.x - value),
                            y: point.y,
                        }
                    } else {
                        point.clone()
                    };

                    if !new_map.contains(&new_point) {
                        new_map.insert(new_point);
                    }
                }
            }
            Fold::Y(value) => {
                for point in &map {
                    let new_point = if &point.y > value {
                        Point {
                            x: point.x,
                            y: value - (point.y - value),
                        }
                    } else {
                        point.clone()
                    };

                    if !new_map.contains(&new_point) {
                        new_map.insert(new_point);
                    }
                }
            }
        }

        map = new_map;

        println!("{:?} -> {}", fold, &map.len())
    }

    let max_x = &map.iter().map(|v| v.x).max().expect("There must be max X");
    let max_y = &map.iter().map(|v| v.y).max().expect("There must be max Y");

    for y in 0..max_y.clone()+1 {
        for x in 0..max_x.clone()+1 {
            let point = Point { x, y};

            if map.contains(&point) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
