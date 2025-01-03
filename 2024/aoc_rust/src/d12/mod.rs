use ::function_name::named;
//use itertools::Itertools;
use utilities::{Board, PointRC};

const DAY: &str = "d12";

#[derive(Debug)]
struct Plot {
    pub plot: char,
    pub area: i32,
    pub permimeter: i32
}

fn count_permimiter(board: &Board<char>, row: i32, col: i32) -> i32 {
    let plot = board.data[row as usize][col as usize];
    let locs = vec![PointRC { r: row, c: col - 1 }, PointRC { r: row, c: col + 1 }, PointRC { r: row - 1, c: col }, PointRC { r: row + 1, c: col }];
    locs.iter().filter(|loc| !board.in_bounds(loc) || board.data[loc.r as usize][loc.c as usize] != plot).count() as i32
}

fn gather(board: &Board<char>, visited: &mut Board<bool>, plot: &mut Plot, row: i32, col: i32) {
    if !board.in_bounds(&PointRC { r: row, c: col }) || visited.data[row as usize][col as usize] || board.data[row as usize][col as usize] != plot.plot {
        return;
    }
    visited.data[row as usize][col as usize] = true;
    plot.area += 1;
    plot.permimeter += count_permimiter(board, row, col);
    let locs = vec![PointRC { r: row, c: col - 1 }, PointRC { r: row, c: col + 1 }, PointRC { r: row - 1, c: col }, PointRC { r: row + 1, c: col }];
    locs.iter().for_each(|loc| gather(board, visited, plot, loc.r, loc.c));
}

fn do_part_1(filename: &str) {
    let board = Board::<char>::load_data_chars(filename);
    let mut visited: Board<bool> = Board::<bool>::new(board.height, board.width, false);
    let mut plot_list = Vec::<Plot>::new();
    for r in 0..board.height {
        for c in 0..board.width {
            if !visited.data[r][c] {
                let mut plot = Plot { plot: board.data[r][c], area: 0, permimeter: 0 };
                gather(&board, &mut visited, &mut plot, r as i32, c as i32);
                plot_list.push(plot);
            }
        }
    }
    let total_cost = plot_list.iter().fold(0, |acc, plot| acc + plot.area * plot.permimeter);
    //println!("{:?}", plot_list);
    println!("part1: {}: {}", filename, total_cost);
}

//#[named]
fn part1() {
    // do_part_1(&("src\\".to_string() + DAY + "\\data_test.txt"));
    // do_part_1(&("src\\".to_string() + DAY + "\\data_test_2.txt"));
    // do_part_1(&("src\\".to_string() + DAY + "\\data_test_3.txt"));
    do_part_1(&("src\\".to_string() + DAY + "\\data.txt"));
    //println!("{}: {}", function_name!(), "done");
}

#[named]
fn part2() {
    let rested_sand_count = 1;
    println!("{}: {}", function_name!(), rested_sand_count);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: src\d14\data.txt: 1415378
// part2: 1