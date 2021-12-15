use std::collections::*;
use std::*;

fn expand(pattern: String, rules: &HashMap<(char, char), char>) -> String {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let next_pattern_chars: Vec<char> = pattern.chars().skip(1).collect();
    let mut result: Vec<char> = Default::default();

    for (a, b) in pattern_chars.into_iter().zip(&next_pattern_chars) {
        result.push(a);
        result.push(rules[&(a, b.clone())]);
    }

    if let Some(last) = next_pattern_chars.last() {
        result.push(last.clone());
    }

    result.iter().collect()
}

fn expand_histogam(
    histogram: &HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_histogram: HashMap<(char, char), u64> = Default::default();
    for ((a, b), v) in histogram {
        if v == &0 {
            continue;
        }

        let new_char = rules[&(a.clone(), b.clone())];

        let initial_key = (a.clone(), b.clone());
        let predecesor = (a.clone(), new_char.clone());
        let successor = (new_char.clone(), b.clone());

        //*new_histogram.entry(initial_key).or_insert(0) = 0;

        *new_histogram.entry(predecesor).or_insert(0) += v;
        *new_histogram.entry(successor).or_insert(0) += v;

        //println!("{:?}", new_histogram);
    }
    new_histogram
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = fs::read_to_string(path).expect("Unable to read a input file!");

    let lines: Vec<&str> = input.lines().collect();

    let pattern = lines[0];

    let mut map: HashMap<(char, char), char> = Default::default();

    for rule in &lines[2..] {
        let part: Vec<_> = rule.split(" -> ").collect();

        assert_eq!(part.len(), 2);
        let key_chars: Vec<_> = part[0].chars().collect();
        let val_chars: Vec<_> = part[1].chars().collect();

        assert_eq!(key_chars.len(), 2);
        assert_eq!(val_chars.len(), 1);

        let key = (key_chars[0], key_chars[1]);
        let value = val_chars[0];

        map.insert(key, value);
    }

    println!("{:?}", map);

    (0..0).fold(pattern.to_string(), |input, idx| {
        let result = expand(input, &map);

        let mut histogram: HashMap<char, u64> = Default::default();
        for char in result.chars() {
            *histogram.entry(char).or_insert(0) += 1;
        }

        println!("{}. {} = {:?}", idx, result, histogram);

        result
    });

    println!("------------- Second part of the task! -------------");

    let initial_histogram = {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let next_pattern_chars: Vec<char> = pattern.chars().skip(1).collect();
        let mut result: HashMap<(char, char), u64> = Default::default();

        for (a, b) in pattern_chars.into_iter().zip(&next_pattern_chars) {
            *result.entry((a, b.clone())).or_insert(0) += 1;
        }
        result
    };

    println!("{:?}", initial_histogram);

    let last_letter = &pattern
        .chars()
        .last()
        .expect("Pattern should have at least one character!");

    (0..40).fold(initial_histogram, |input, idx| {
        let result = expand_histogam(&input, &map);

        let mut histogram_of_letters: Vec<(char, u64)> = {
            let mut tmp: HashMap<char, u64> = Default::default();
            for ((a, _), v) in &result {
                *tmp.entry(a.clone()).or_insert(0) += v;
            }

            // Add last letter that was not counded earlier
            *tmp.entry(last_letter.clone()).or_insert(0) += 1;

            tmp
        }
        .into_iter()
        .collect();

        print!("{}. ", idx);

        histogram_of_letters.sort_by(|(a, va), (b, vb)| va.cmp(vb));

        for (k, v) in histogram_of_letters {
            print!("{} -> {}, ", k, v);
        }
        println!("");

        result
    });
}
