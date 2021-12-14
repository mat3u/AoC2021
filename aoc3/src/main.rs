use std::env;
use std::fs;

const N: u8 = 12;

fn find_most_common(values: &[u16], n: u8) -> u8 {
    let mut one_cnt = 0;
    let mut zero_cnt = 0;

    for v in values {
        if v & (1 << n) > 0 {
            one_cnt += 1;
        } else {
            zero_cnt += 1;
        }
    }

    if zero_cnt <= one_cnt {
        1
    } else {
        0
    }
}

fn part_one(values: &[u16]) -> u16 {
    // Part one
    let mut gamma = 0;
    let mut epsilon = 0;
    for n in 0..N {
        let k = find_most_common(values, n);

        if k == 1 {
            gamma = gamma | (1 << n);
        } else {
            epsilon = epsilon | (1 << n)
        }
    }

    println!(
        "EPS = {}, GAMMA = {}, MUL = {}",
        epsilon,
        gamma,
        epsilon as u32 * gamma as u32
    );

    gamma
}

fn part_two(values: &[u16]) {
    let mut V = values.to_vec();

    for n in (0..N).rev() {
        let mcn = find_most_common(&V, n);

        // println!("{:?} without {}", V, mcn);

        V = V
            .to_vec()
            .into_iter()
            .filter(|v| {
                if mcn == 1 {
                    v & (1 << n) > 0
                } else {
                    v & (1 << n) == 0
                }
            })
            .collect();
    }

    let ox = V[0];

    assert_eq!(V.len(), 1);

    println!("{:?}", ox);

    V = values.to_vec();
    for n in (0..N).rev() {
        let mcn = find_most_common(&V, n);

        // println!("{:?} without {}", V, mcn);

        V = V
            .to_vec()
            .into_iter()
            .filter(|v| {
                if mcn == 1 {
                    v & (1 << n) == 0
                } else {
                    v & (1 << n) > 0
                }
            })
            .collect();

        if V.len() == 1 {
            break;
        }
    }

    let co2 = V[0];

    println!("{:?}", co2);

    println!("{}", co2 as u32 * ox as u32);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).unwrap();
    let lines: Vec<_> = input
        .lines()
        .map(|l| u16::from_str_radix(&l, 2).unwrap())
        .collect();

    part_one(&lines);

    part_two(&lines);
}
