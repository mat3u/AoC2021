use std::env;
use std::fs;

const BOARD_SIZE: usize = 5;

struct Board {
    numbers: [[u32; BOARD_SIZE]; BOARD_SIZE],
    marks: [[bool; BOARD_SIZE]; BOARD_SIZE],
    winning: i32,
    order: i32,
}

impl Board {
    fn new() -> Self {
        Board {
            marks: [[false; BOARD_SIZE]; BOARD_SIZE],
            numbers: [[0u32; BOARD_SIZE]; BOARD_SIZE],
            winning: -1,
            order: -1,
        }
    }

    fn parse(raw: Vec<&str>) -> Self {
        let mut board = Self::new();

        assert_eq!(raw.len(), 5);

        for x in 0..BOARD_SIZE {
            let line: Vec<u32> = raw[x]
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            assert_eq!(line.len(), BOARD_SIZE);
            for y in 0..BOARD_SIZE {
                board.set_number(x, y, line[y])
            }
        }

        board
    }

    fn has_winner(&self) -> bool {
        for x in 0..BOARD_SIZE {
            let mut cnt = 0;
            for y in 0..BOARD_SIZE {
                if self.marks[x][y] {
                    cnt += 1;
                }
            }

            if cnt == BOARD_SIZE {
                return true;
            }
        }

        for y in 0..BOARD_SIZE {
            let mut cnt = 0;
            for x in 0..BOARD_SIZE {
                if self.marks[x][y] {
                    cnt += 1;
                }
            }

            if cnt == BOARD_SIZE {
                return true;
            }
        }

        return false;
    }

    fn points(&self) -> u32 {
        let mut result = 0;
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if !self.marks[x][y] {
                    result += self.numbers[x][y];
                }
            }
        }
        result
    }

    fn mark(&mut self, v: u32, id: i32) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.numbers[x][y] == v {
                    self.marks[x][y] = true;
                }
            }
        }

        if self.order == -1 && self.has_winner() {
            self.order = id;
            self.winning = v as i32;
        }
    }

    fn format_number(&self, x: usize, y: usize) -> String {
        if self.marks[x][y] {
            format!("[{}]", self.numbers[x][y])
        } else {
            format!(" {} ", self.numbers[x][y])
        }
    }

    fn set_number(&mut self, x: usize, y: usize, value: u32) {
        self.numbers[x][y] = value;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {} {} {}\r\n{} {} {} {} {}\r\n{} {} {} {} {}\r\n{} {} {} {} {}\r\n{} {} {} {} {}", 
        self.format_number(0,0),self.format_number(0,1), self.format_number(0,2), self.format_number(0,3), self.format_number(0,4),
        self.format_number(1,0),self.format_number(1,1), self.format_number(1,2), self.format_number(1,3), self.format_number(1,4),
        self.format_number(2,0),self.format_number(2,1), self.format_number(2,2), self.format_number(2,3), self.format_number(2,4),
        self.format_number(3,0),self.format_number(3,1), self.format_number(3,2), self.format_number(3,3), self.format_number(3,4),
        self.format_number(4,0),self.format_number(4,1), self.format_number(4,2), self.format_number(4,3), self.format_number(4,4),
        )
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("sample.txt".to_string());

    let input = fs::read_to_string(path).expect("Unable to read input file!");

    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<u32> = lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let vsize = lines.len() - 1;

    // Input data is predictable
    println!("{} {}", vsize, numbers.len());
    assert_eq!(vsize % 6, 0);

    let boards_n = vsize / 6;

    let mut boards: Vec<Board> = (0..boards_n)
        .map(|i| {
            let board = Board::parse(lines[2 + i * 6..(2 + i * 6 + BOARD_SIZE)].to_vec());

            println!("{}", board);
            board
        })
        .collect();

    let mut result =
        (1..numbers.len() + 1)
            .zip(numbers)
            .into_iter()
            .fold(boards, |boards, (id, number)| {
                let result: Vec<Board> = boards
                    .into_iter()
                    .map(|mut board| {
                        if board.winning == -1 {
                            board.mark(number, id as i32);
                        }
                        board
                    })
                    .collect();

                result
            });

    let _r2 = &result.sort_by(|a, b| b.order.cmp(&a.order));

    let board = &result[0];

    println!("{}", board);

    println!(
        "{} * {} = {}",
        board.points(),
        board.winning,
        board.points() * board.winning as u32
    );
}
