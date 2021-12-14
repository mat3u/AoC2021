use std::collections::HashMap;
use std::*;

trait SuperDuper {
    fn is_lowercase(&self) -> bool;
}

impl SuperDuper for String {
    fn is_lowercase(&self) -> bool {
        let s = self.clone().to_owned();

        s.to_lowercase().eq(&s)
    }
}

fn find_possible_paths<'a>(
    map: &HashMap<String, Vec<String>>,
    prefix: &Vec<String>,
    forbidden: &Vec<String>,
) -> Vec<Vec<String>> {
    let start = prefix
        .last()
        .expect("There must be at least one element in path!");

    let possible_next_caves = &map[start];

    let nexts: Vec<String> = possible_next_caves
        .into_iter()
        .map(|v| v.clone())
        .filter(|cave| {
            (if forbidden.contains(&cave) {
                forbidden
                    .into_iter()
                    .find(|e| forbidden.into_iter().filter(|v| v == e).count() == 2)
                    .is_none()
            } else {
                true
            }) && cave != "start"
        })
        .collect();

    if nexts.is_empty() || start == "end" {
        vec![prefix.clone()]
    } else {
        nexts
            .into_iter()
            .flat_map(|cave| {
                let new_forbidden = if cave.is_lowercase() {
                    let mut new_forbidden_tmp = forbidden.clone();
                    new_forbidden_tmp.push(cave.clone());

                    new_forbidden_tmp
                } else {
                    forbidden.clone()
                };
                let mut new_prefix = prefix.clone();

                new_prefix.push(cave.clone());

                println!("Prefix: {:?}", new_prefix);

                find_possible_paths(map, &new_prefix, &new_forbidden)
            })
            .collect()
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to read a file!");

    /*
        1. Create sparse matrix from -> to

        So I can ask possible_caves_from(X) and get [a, b, C, ...];

        and generate all possible paths using recurring algorithm

    */

    let mut cave_map: HashMap<String, Vec<String>> = Default::default();

    for line in input.split('\n') {
        let parts: Vec<_> = line.split('-').map(|v| v.to_owned()).collect();
        assert_eq!(parts.len(), 2);

        let from = parts[0].to_owned();
        let to = parts[1].to_owned();

        if let Some(locations) = cave_map.get_mut(&from) {
            if !(&locations.contains(&to)).clone() {
                locations.push(to.clone());
            }
        } else {
            cave_map.insert(from.clone(), vec![to.clone()]);
        }
        if let Some(locations) = cave_map.get_mut(&to) {
            if !(&locations.contains(&from)).clone() {
                locations.push(from);
            }
        } else {
            cave_map.insert(to, vec![from]);
        }
    }

    let start = vec!["start".to_owned()];
    let possible_paths: Vec<Vec<String>> = find_possible_paths(&cave_map, &start, &start)
        .into_iter()
        .filter(|v| v.last().expect("Must be something!") == "end")
        .collect();
    println!("---------------");
    for path in &possible_paths {
        println!("{:?}", path);
    }

    println!("{}", possible_paths.len());
}
