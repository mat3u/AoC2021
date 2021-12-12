use std::env;
use std::fs;

trait Useful {
    fn vvsort(&self) -> String;
    fn vvintersect_len(&self, other: &str) -> usize;
}

impl Useful for String {
fn vvsort(&self) -> String { 
    let mut v: Vec<char> = self.chars().collect();
    v.sort();

    String::from_iter(v)
 }
fn vvintersect_len(&self, other: &str) -> usize { 
    let mut cnt = 0;
    for c in self.chars() {
        if other.contains(c) {
            cnt += 1
        }
    }

    cnt as usize
 }
}

fn parse(inputs: &Vec<String>) -> [String; 10] {
    assert_eq!(inputs.len(), 14);

    let mut result: [String; 10] = Default::default();

    for i in inputs {
        match i.len() {
            2 => result[1] = i.vvsort(),
            3 => result[7] = i.vvsort(),
            4 => result[4] = i.vvsort(),
            7 => result[8] = i.vvsort(),
            _ => () // Do nothing in this loop
        }
    }

    for i in inputs {
        match i.len() {
            5 => {
                if i.vvintersect_len(&result[1]) == 2 {
                    result[3] = i.vvsort();
                } else if i.vvintersect_len(&result[4]) == 3 {
                    result[5] = i.vvsort();
                } else {
                    result[2] = i.vvsort();
                }
            },
            6 => {
                if i.vvintersect_len(&result[1]) == 1 {
                    result[6] = i.vvsort();
                } else if i.vvintersect_len(&result[4]) == 4 {
                    result[9] = i.vvsort();
                } else {
                    result[0] = i.vvsort();
                }
            },
            _ => ()
        }
    }

    result
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to open a file!");

    let numbers: Vec<&str> = input
        .split('\n')
        .collect();

    let mut result2 = 0;

    for x in &numbers {
        let xy = x.replace(" | ", " ");
        let xx: Vec<String> = xy.split_whitespace().map(|x| x.to_string()).collect();

        let map = parse(&xx);

        println!("{:?}", map);

        let mut mul = 1;
        let mut result = 0;
        for i in (10..14).into_iter().rev() {
            let code = &xx[i];
            let digit = &map.iter().position(|r| r.as_str() == code.to_string().vvsort()).unwrap();

            result += digit * mul;
            mul *= 10;
        }

        println!("{}", result);

        result2 += result
    }

    println!("RES: {}", result2);
}
