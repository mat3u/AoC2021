use std::collections::HashMap;
use std::*;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn increase_deltas(delta_levels: &mut HashMap<Point, i32>, point: &Point) {
    for (dx, dy) in vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        /* (0,0), */
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let dp = Point {
            x: point.x + dx,
            y: point.y + dy,
        };

        if let Some(delta) = delta_levels.get_mut(&dp) {
            *delta += 1;
        } else {
            delta_levels.insert(dp, 1);
        }
    }
}

// fn do_step(energy_levels: &mut HashMap<Point, i32>) -> u32 {
//     let mut delta_levels: HashMap<Point, i32> = Default::default();

//     let mut flashes = 0;

//     loop {
//         let mut it_flashes = 0;

//         for (point, initial_level) in energy_levels.into_iter() {
//             print!(".");
//             let dp = point.clone();

//             let maybe_delta = delta_levels.get_mut(point);

//             let delta = match maybe_delta {
//                 Some(value) => value,
//                 None => {
//                     delta_levels.insert(dp.clone(), 1);
//                     delta_levels.get_mut(&dp).expect("This must exists!")
//                 }
//             };

//             let level = delta.clone() + initial_level.clone();

//             if level > 9 {
//                 it_flashes += 1;
//                 // flash
//                 *delta = -100000000 as i32;

//                 increase_deltas(&mut delta_levels, &dp.clone());
//             } else {
//                 *delta += 1;
//             }
//         }

//         flashes += it_flashes;

//         println!("{}", it_flashes);
//         if it_flashes == 0 {
//             break;
//         }
//     }

//     // Update entryLevel
//     for (point, value) in energy_levels {
//         let dp = point.clone();

//         if let Some(delta_level) = delta_levels.get(&dp)
//         {
//             // Reset to 0
//             if delta_level < &0 {
//                 *value = 0;
//             } else {
//                 *value = value.clone() + delta_level.clone();
//             }
//         } else {
//             panic!("Should not reach this point! There should be delta for each energy point!");
//         }
//     }

//     flashes
// }

fn do_step(energy_levels: &mut HashMap<Point, i32>) -> u32 {
    // 1. Increase energy level
    for x in 0..10 {
        for y in 0..10 {
            let point = Point { x, y };
            if let Some(value) = energy_levels.get_mut(&point) {
                *value += 1;
            }
        }
    }

    let mut flash = 0;

    loop {
        let mut it_flash = 0;

        for x in 0..10 {
            for y in 0..10 {
                let point = Point { x, y };
                // Flash
                let value = energy_levels
                    .get_mut(&point)
                    .expect(format!("Energy level at [{:?}] must exists!", point).as_str());

                if value.clone() > 9 {
                    print!("*");
                    it_flash += 1;
                    *value = -100000000;

                    increase_deltas(energy_levels, &point)
                } else {
                    print!(".")
                }

                if y == 9 {
                    println!("");
                }
            }
        }
        if it_flash == 0 {
            break;
        } else {
            flash += it_flash;
        }
    }

    for x in 0..10 {
        for y in 0..10 {
            let point = Point { x, y };

            if let Some(value) = energy_levels.get_mut(&point) {
                if value.clone() < 0 {
                    *value = 0;
                }
            }
        }
    }

    flash
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to open file!");
    let chars: Vec<char> = input.chars().collect();

    let mut energy_levels: HashMap<Point, i32> = Default::default();

    let mut x = 0;
    let mut y = 0;

    for c in chars {
        if c == '\n' {
            y += 1;
            x = 0;
            continue;
        }

        let level = c as i32 - '0' as i32;
        let point = Point { x, y };

        energy_levels.insert(point, level);

        x += 1;
    }

    // println!("{:?}", energy_levels);

    const N: u16 = 1000;
    let mut flashes = 0;
    for i in 0..N {
        println!("{}.", i);
        
        let it_flashes = do_step(&mut energy_levels);
        println!("{}", flashes);

        if it_flashes == 100 {
            break;
        }

        flashes += it_flashes;
    }

    println!("{}", flashes);
}
