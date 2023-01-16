use std::{fs, time::Instant, collections::HashSet};

use ::function_name::named;
use utilities::{alloc_2d_vec, valid_vec_index};
//use itertools::Itertools;
//use lazy_static::lazy_static;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    pub r: i32,
    pub c: i32
}

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point { r: self.r - 1, c: self.c },
            Point { r: self.r + 1, c: self.c },
            Point { r: self.r, c: self.c - 1 },
            Point { r: self.r, c: self.c + 1 }
        ]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Blizzard {
    start_row: usize,
    start_col: usize,
    direction: Direction
}

impl Blizzard {
    fn new(start_row: usize, start_col: usize, direction: Direction) -> Blizzard {
        Blizzard { start_row, start_col, direction }
    }

    fn location_at_time_handle_walls(&self, time: usize, grid: &Vec<Vec<bool>>) -> (usize, usize) {
        let (usable_height, usable_width) = (grid.len() as i32 - 2, grid[0].len() as i32 - 2);
        let (mut offset_row, mut offset_col) = (self.start_row as i32 - 1, self.start_col as i32 - 1);
        match self.direction {
            Direction::Up => {
                offset_row = (offset_row - time as i32 + (usable_height * time as i32)) % usable_height;
            }
            Direction::Down => {
                offset_row = (offset_row + time as i32) % usable_height;
            }
            Direction::Left => {
                offset_col = (offset_col - time as i32 + (usable_width * time as i32)) % usable_width;
            }
            Direction::Right => {
                offset_col = (offset_col + time as i32) % usable_width;
            }
        }
        (offset_row as usize + 1, offset_col as usize + 1)
    }
}

struct World {
    pub open_space_map: Vec<Vec<bool>>,
    pub blizzards: Vec<Blizzard>,
    pub height: usize,
    pub width: usize,
    pub start_loc: Point,
    pub end_loc: Point
}

impl World {
    fn get_level_at_time(&self, time: usize) -> Level {
        let mut safe: Vec<Vec<bool>> = self.open_space_map.clone();
        for (row, col)in self.blizzards.iter().map(|blizzard| blizzard.location_at_time_handle_walls(time, &self.open_space_map)) {
            assert!(self.open_space_map[row][col] == true);
            safe[row][col] = false;
        }
        Level { safe }
    }
    #[allow(dead_code)]
    fn print_level_at_time(&self, time: usize) {
        let mut grid = alloc_2d_vec(self.height, self.width, ' ');
        for (row_index, row) in self.open_space_map.iter().enumerate() {
            for (col_index, safe) in row.iter().enumerate() {
                if *safe {
                    grid[row_index][col_index] = '.';
                } else {
                    grid[row_index][col_index] = '#';
                }
            }
        }
        for blizzard in &self.blizzards {
            let (row, col) = blizzard.location_at_time_handle_walls(time, &self.open_space_map);
            assert!(self.open_space_map[row][col] == true);
            match grid[row][col] {
                '.' => {
                    grid[row][col] = match blizzard.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                }
                '^' | 'v' | '<' | '>' => {
                    grid[row][col] = '2';
                }
                '2'..='9' => {
                    grid[row][col] = (grid[row][col] as u8 + 1) as char;
                }
                '#' => { panic!("Blizzard {} {} {} hit a wall", row, col, time); }
                _ => { panic!(); }
            }
        }
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

struct Level {
    pub safe: Vec<Vec<bool>>,
}

fn load_data(path: &str) -> World {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let lines = file_contents_as_string.lines().collect::<Vec<&str>>();
    let (height, width) = (lines.len(), lines[0].len());
    let mut open_space_map: Vec<Vec<bool>> = alloc_2d_vec(height, width, true);
    let mut blizzards: Vec<Blizzard> = Vec::new();
    for (row_index, row) in lines.iter().enumerate() {
        for (col_index, val) in row.chars().enumerate() {
            match val {
                '#' => { open_space_map[row_index][col_index] = false; }
                '^' => { blizzards.push(Blizzard::new(row_index, col_index, Direction::Up)); }
                'v' => { blizzards.push(Blizzard::new(row_index, col_index, Direction::Down)); }
                '<' => { blizzards.push(Blizzard::new(row_index, col_index, Direction::Left)); }
                '>' => { blizzards.push(Blizzard::new(row_index, col_index, Direction::Right)); }
                '.' => {}
                _ => { panic!("Unexpected character in input: {}", val); }
            }
        }
    }
    assert!(open_space_map[0][0] == false && open_space_map[0][1] == true);
    assert!(open_space_map[height - 1][width - 1] == false && open_space_map[height - 1][width - 2] == true);

    World { open_space_map, blizzards, height, width, start_loc : Point{ r:0, c:1 }, end_loc: Point{ r: height as i32 - 1, c: width as i32 - 2 } }
}

fn best_time_to_reach_end(world: &World, start_time: usize) -> usize {
    let mut time_curr = start_time;
    let mut prev_options: HashSet<Point> = HashSet::new();
    let mut next_options: HashSet<Point> = HashSet::new();
    prev_options.insert(world.start_loc);
    let mut at_end = false;
    while !at_end {
        time_curr += 1;
        //world.print_level_at_time(time_curr);
        let level = world.get_level_at_time(time_curr);
        for p in prev_options.drain() {
            if level.safe[p.r as usize][p.c as usize] {
                next_options.insert(p);
            }
            for n in p.neighbors() {
                if n == world.end_loc {
                    at_end = true;
                    break;
                }
                if valid_vec_index(&level.safe, n.r)
                && valid_vec_index(&level.safe[0], n.c)
                && level.safe[n.r as usize][n.c as usize]
                && !next_options.contains(&n)
                {
                    next_options.insert(n);
                }
            }
        }
        assert!(!next_options.is_empty());
        prev_options = next_options.clone();
        next_options.clear();
    }
    time_curr
}

#[named]
fn part1() {
    let now = Instant::now();

    let world = load_data("src\\d24\\data.txt");
    let best_time = best_time_to_reach_end(&world, 0);

    println!("{}: {} ({} ms)", function_name!(), best_time, now.elapsed().as_micros() as f32 / 1000.0);
}

fn swap<T: Clone>(i: &mut T, j: &mut T) {
    let tmp = i.clone();
    *i = j.clone();
    *j = tmp;
}

#[named]
fn part2() {
    let now = Instant::now();

    let mut world = load_data("src\\d24\\data.txt");
    let mut best_time = best_time_to_reach_end(&world, 0);
    swap(&mut world.start_loc, &mut world.end_loc);
    best_time = best_time_to_reach_end(&world, best_time);
    swap(&mut world.start_loc, &mut world.end_loc);
    best_time = best_time_to_reach_end(&world, best_time);

    println!("{}: {} ({} ms)", function_name!(), best_time, now.elapsed().as_micros() as f32 / 1000.0);
}

pub fn run() {
    part1();
    part2();
}

// part1: 283 (373.207 ms)
// part2: 883 (1123.216 ms)