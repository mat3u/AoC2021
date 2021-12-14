use std::collections::*;
use std::*;

#[derive(Debug, Eq, PartialEq)]
enum Result<'a> {
    Correct,
    Incorrect(&'a char),
    Incomplete(Vec<&'a char>),
}

fn check(input: &[char]) -> Result {
    let mut stack: Vec<&char> = Default::default();
    let mut result = Result::Correct;

    for c in input {
        match c {
            '(' => stack.push(&')'),
            '<' => stack.push(&'>'),
            '[' => stack.push(&']'),
            '{' => stack.push(&'}'),
            found => match stack.last() {
                Some(expected) => {
                    if expected == &found {
                        stack.pop();
                    } else {
                        result = Result::Incorrect(found);
                        break;
                    }
                }
                _ => {
                    panic!("Never reached!");
                }
            },
        }
        // println!("Stack = {:?}", stack);
    }

    if result == Result::Correct && !stack.is_empty() {
        let mut stack_copy = stack.clone();
        stack_copy.reverse();
        Result::Incomplete(stack_copy)
    } else {
        result
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to open a file!");

    let lines: Vec<&str> = input.split_whitespace().collect();

    let mut points_map: HashMap<char, u64> = Default::default();
    points_map.insert(')', 3);
    points_map.insert(']', 57);
    points_map.insert('}', 1197);
    points_map.insert('>', 25137);

    let mut missing_map: HashMap<char, u64> = Default::default();

    missing_map.insert(')', 1);
    missing_map.insert(']', 2);
    missing_map.insert('}', 3);
    missing_map.insert('>', 4);
    let mut missing_aa: Vec<u64> = Default::default();

    let mut result = 0;
    for line in &lines {
        let chars: Vec<_> = line.chars().collect();
        let is_valid = check(&chars[0..]);

        match is_valid {
            Result::Incorrect(c) => {
                result += points_map[c];
            }
            Result::Incomplete(missing) => {
                println!("{:?}", missing);

                let mut missing_points = 0;
                for c in missing {
                    missing_points *= 5;
                    missing_points += missing_map[&c];
                }

                missing_aa.push(missing_points);
            }
            _ => (),
        }
    }

    missing_aa.sort();

    println!("{:?}", missing_aa);

    println!("{}", result);
    println!("{}", missing_aa[missing_aa.len() / 2]);

}
