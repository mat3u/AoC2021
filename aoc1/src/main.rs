use std::env;
use std::fs;
use itertools::izip;

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());

    let input = fs::read_to_string(path)
                .expect("Unable to read input file!");

    let values: Vec<u16> = input.lines().map(|v| v.parse::<u16>().unwrap()).collect();


    let mut counter = 0;
    /* First task
    let mut prev = values.first().unwrap();

    println!("{:?}", prev);
    for current in &values[1..] {
        if current > prev {
            println!("{:?} Increased", current);
            counter += 1;
        }
        else {
            println!("{:?}", current);
        }

        prev = current;
    }

    // */

    if values.len() < 3 {
        println!("Fuck it! Not enough data!");
    } else {
        let mut prev = values[0] + values[1] + values[2];
        for (a,b,c) in izip!(&values[1..], &values[2..], &values[3..]) {
            let current = a+b+c;

            if current > prev {
                println!("{:?} Increased", current);
                counter += 1;
            }
            else {
                println!("{:?}", current);
            }
    
            prev = current;
        }
    }

    println!("Result = {:?}", counter);
}
