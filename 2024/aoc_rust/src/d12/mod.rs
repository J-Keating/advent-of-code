use ::function_name::named;
use itertools::Itertools;
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

#[derive(Debug, PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Debug)]
struct FenceSegment {
    pub loc: PointRC,
    pub direction: Direction
}

struct Plot2 {
    pub plot: char,
    pub area: i32,
    pub fences: Vec<FenceSegment>
}

fn add_fences(board: &Board<char>, fences: &mut Vec<FenceSegment>, loc: &PointRC) {
    let plot = *board.at(loc);
    let neighbors = vec![
        (PointRC { r: loc.r, c: loc.c - 1 }, Direction::Left),
        (PointRC { r: loc.r, c: loc.c + 1 }, Direction::Right),
        (PointRC { r: loc.r - 1, c: loc.c }, Direction::Top),
        (PointRC { r: loc.r + 1, c: loc.c }, Direction::Bottom)];
    for (neighbor, dir) in neighbors {
        if !board.in_bounds(&neighbor) || *board.at(&neighbor) != plot {
            fences.push(FenceSegment { loc: *loc, direction: dir });
        }
    }
}

fn gather2(plot: char, board: &Board<char>, visited: &mut Board<bool>, fences: &mut Vec<FenceSegment>, loc: &PointRC) {
    if !board.in_bounds(&loc) || *visited.at(&loc) || *board.at(&loc) != plot {
        return;
    }
    visited.data[loc.r as usize][loc.c as usize] = true;
    add_fences(board, fences, &loc);
    let locs = loc.neighbors_cardinal();
    locs.iter().for_each(|neighbor| gather2(plot, board, visited, fences, &neighbor));
}

fn rowwise_point_compare(a: &PointRC, b: &PointRC) -> std::cmp::Ordering {
    if a.r == b.r {
        return a.c.cmp(&b.c);
    }
    a.r.cmp(&b.r)
}

fn count_sides(fences: &Vec<FenceSegment>) -> i32 {
    let tops = fences.iter().filter(|fence| fence.direction == Direction::Top).map(|fs| fs.loc).sorted_by(rowwise_point_compare).collect::<Vec<PointRC>>();
    let top_side_count = tops.iter().tuple_windows().filter(|(a, b)| a.r != b.r || a.c + 1 < b.c).count() + 1;

    let bottoms = fences.iter().filter(|fence| fence.direction == Direction::Bottom).map(|fs| fs.loc).sorted_by(rowwise_point_compare).collect::<Vec<PointRC>>();
    let bottom_side_count = bottoms.iter().tuple_windows().filter(|(a, b)| a.r != b.r || a.c + 1 < b.c).count() + 1;

    (top_side_count + bottom_side_count) as i32
}

fn do_part_2(filename: &str) {
    let board = Board::<char>::load_data_chars(filename);
    let mut visited: Board<bool> = Board::<bool>::new(board.height, board.width, false);
    // let mut plot_list = Vec::<Plot>::new();
    for r in 0..board.height {
        for c in 0..board.width {
            if !visited.data[r][c] {
                let mut fences = Vec::<FenceSegment>::new();
                gather2(board.data[r][c], &board, &mut visited, &mut fences, &PointRC { r: r as i32, c: c as i32 });
                println!("fences: {:?}", fences);
                let sides = count_sides(&fences);
                // plot_list.push(plot);
            }
        }
    }
    let total_cost = 0; //plot_list.iter().fold(0, |acc, plot| acc + plot.area * plot.permimeter);
    //println!("{:?}", plot_list);
    println!("part2: {}: {}", filename, total_cost);
}

#[named]
fn part2() {
    do_part_2(&("src\\".to_string() + DAY + "\\data_test.txt"));
    // do_part_2(&("src\\".to_string() + DAY + "\\data_test_2.txt"));
    // do_part_2(&("src\\".to_string() + DAY + "\\data_test_3.txt"));
    // do_part_2(&("src\\".to_string() + DAY + "\\data.txt"));
    //println!("{}: {}", function_name!(), "done");
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: src\d14\data.txt: 1415378
// part2: 1