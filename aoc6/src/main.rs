use std::env;
use std::fs;

struct Lanternfish {
    age: u16
}

impl Lanternfish {
    fn age(&self) -> Vec<Lanternfish> {
        if self.age == 0 {
            vec![
                Lanternfish { age: 6 },
                Lanternfish { age: 8 }
            ]
        } else {
            vec![
                Lanternfish { age: self.age - 1 }
            ]
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect(&"Unable to read input file!");
    let lines: Vec<&str> = input.lines().collect();

    // let lanternfish: Vec<Lanternfish> = lines[0]
    //         .split(',')
    //         .map(|x| x.parse::<u16>().expect("Unable to parse [] to age!"))
    //         .map(|age| Lanternfish { age: age })
    //         .collect();

    // Naive approach
    // let result: Vec<Lanternfish> = (0..256).into_iter()
    //     .fold(lanternfish, |lv, idx| {
    //         print!(".");
    //         if idx % 5 == 0 {
    //             println!("{}", idx);
    //         }
    //         lv.par_iter().flat_map(|l| l.age()).collect()
    //     });

    const CYCLE: usize = 9;
    let mut map = [0u64; CYCLE];

    let x: Vec<_> = lines[0].split(',').map(|x| x.parse::<usize>().unwrap()).collect();

    for n in x {
        map[n] += 1;
    }

    for day in 0..256 {
        println!("{}", day);

        let new = map[0];

        for d in 1..CYCLE {
            map[d-1] = map[d]
        }

        map[6] += new; // Add parents
        map[8] = new; // Only new
    }

    let result = map.into_iter().fold(0, |z, v| z + v);

    println!("{}", result);
}
