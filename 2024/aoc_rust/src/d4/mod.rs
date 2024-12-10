use std::fs;

use ::function_name::named;
use utilities::alloc_2d_vec;

fn load_2d_char_array_from_file(file_name: &str) -> Vec<Vec<char>> {
    let file_contents = fs::read_to_string(file_name).expect("Error loading file");
    file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

struct Board {
    pub data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn load_from_file(file_name: &str) -> Board {
        let data = load_2d_char_array_from_file(file_name);
        let width = data[0].len();
        let height = data.len();
        Board {
            data,
            width,
            height,
        }
    }

    fn match_string(&self, row: usize, col: usize, dx: i32, dy: i32, match_str: &str) -> bool {
        let mut row = row as i32;
        let mut col = col as i32;
        for c in match_str.chars() {
            if row < 0 || row >= self.height as i32 || col < 0 || col >= self.width as i32 {
                return false;
            }
            if self.data[row as usize][col as usize] != c {
                return false;
            }
            row += dy;
            col += dx;
        }
        true
    }

    pub fn count_occurances_of(&self, string_to_match: &str) -> i32 {
        const DIRECTIONS: [(i32, i32); 8] = [
            (1, 0),
            (0, 1),
            (1, 1),
            (1, -1),
            (-1, 0),
            (0, -1),
            (-1, -1),
            (-1, 1),
        ];

        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                for (dx, dy) in DIRECTIONS {
                    if self.match_string(row, col, dx, dy, string_to_match) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn count_x_shapes(&self) -> i32 {
        let mut count = 0;
        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                if self.data[row][col] == 'A' {
                    let test_chars = [
                        self.data[row - 1][col - 1],
                        self.data[row - 1][col + 1],
                        self.data[row + 1][col - 1],
                        self.data[row + 1][col + 1],
                    ];
                    if test_chars[0] != test_chars[3]
                        && test_chars.iter().filter(|c| *c == &'M').count() == 2
                        && test_chars.iter().filter(|c| *c == &'S').count() == 2
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for line in &self.data {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

#[named]
fn part1() {
    let board = Board::load_from_file("src\\d4\\data.txt");
    let count = board.count_occurances_of("XMAS");
    println!("{}: {}", function_name!(), count);
}

#[named]
fn part2() {
    let board = Board::load_from_file("src\\d4\\data.txt");
    let count = board.count_x_shapes();
    println!("{}: {}", function_name!(), count);
}

pub fn run() {
    part1();
    part2();
}

// part1: 511
// part2: 821
