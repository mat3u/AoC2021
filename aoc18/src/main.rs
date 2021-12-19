use std::*;

#[derive(Debug, Eq, PartialEq)]
enum Number {
    Plain(i32),
    Complex {
        left: Box<Number>,
        right: Box<Number>,
    },
}

impl Number {
    fn parse(raw: &str) -> (Number, &str) {
        let mut it = raw.chars().into_iter();
        let mut c = it.next().expect("Empty string provided to parser!");

        if c >= '0' && c <= '9' {
            (Number::Plain(c as i32 - '0' as i32), &raw[1..])
        } else if c == '[' {
            let (left, rem) = Number::parse(&raw[1..]);
            let (right, rem2) = Number::parse(&rem[1..]); // Skipping comma

            (Number::Complex { left: Box::new(left), right: Box::new(right) }, &rem2[1..])
        } else {
            panic!("Should not reach here!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    #[test]
    fn simple() {
        let input = "[1,2]";
        let expected = Number::Complex {
            left: Box::new(Number::Plain(1)),
            right: Box::new(Number::Plain(2))
        };

        let (result, _) = Number::parse(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn simple_2() {
        let input = "[1,[2,3]]";
        let expected = Number::Complex {
            left: Box::new(Number::Plain(1)),
            right: Box::new(Number::Complex {
                left: Box::new(Number::Plain(2)),
                right: Box::new(Number::Plain(3))
            })
        };

        let (result, _) = Number::parse(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn complex() {
        let input = "[[[[4,1],[5,1]],[4,[9,1]]],4]";

        let (number, _) = Number::parse(input);

        println!("{:?}", number);
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());
    let input = "";
}
