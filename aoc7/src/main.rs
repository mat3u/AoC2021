use std::fs;
use std::env;

fn main() {
    let path = env::args().nth(1).unwrap_or("./sample.txt".to_string());
    let input: Vec<_> = fs::read_to_string(path)
        .expect("Unable to read input file!")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().expect(&format!("Cannot parse [{}] to u16", x)))
        .collect();

    let min: i32 = *input.iter().min().unwrap() as i32;
    let max: i32 = *input.iter().max().unwrap() as i32;

    let mut min_result = 1000000000;
    let base: i64 = 2;

    for i in min..max {
        let mut result = 0;

        // 1,2,3,4 

        for idx in &input {
            result += (1..((i - idx).abs()+1)).into_iter().sum::<i32>();

            if result > min_result {
                break;
            }
        }

        if result < min_result {
            min_result = result;
        }

        println!("{}: {} -> {}", i, min_result, result);
    }
}
