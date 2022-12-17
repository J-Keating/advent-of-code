use std::{fs, fmt};

use ::function_name::named;
use itertools::Itertools;

//use itertools::Itertools;
//use lazy_static::lazy_static;

use utilities::{alloc_2d_vec, Point};

fn print_grid(grid: &Vec<Vec<GridState>>) {
    for row in grid {
        println!("|{}|", row.iter().fold(String::new(), |acc, gs| acc + &gs.to_string()));
    }
}

#[derive(Clone, PartialEq)]
enum GridState {
    Air,
    Rock,
    Sand
}

impl fmt::Display for GridState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            GridState::Air => { " " }
            GridState::Rock => { "#" }
            GridState::Sand => { "o" }
        })
    }
}

enum SimulationResult {
    Falling(Point),
    Resting(Point),
    OOB
}

fn simulate_sand(curr_loc: &Point, grid: &Vec<Vec<GridState>>) -> SimulationResult {
    let mut loc = curr_loc.add(&Point { x: 0, y: 1 });
    if loc.y >= grid.len() as i32 { return SimulationResult::OOB; }
    if grid[loc.y as usize][loc.x as usize] == GridState::Air { return SimulationResult::Falling(loc); }
    loc.move_by(&Point { x: -1, y: 0 });
    if grid[loc.y as usize][loc.x as usize] == GridState::Air { return SimulationResult::Falling(loc); }
    loc.move_by(&Point { x: 2, y: 0 });
    if grid[loc.y as usize][loc.x as usize] == GridState::Air { return SimulationResult::Falling(loc); }
    SimulationResult::Resting(*curr_loc)
}

fn do_it_all(build_floor: bool) -> i32 {
    let mut paths: Vec<Vec<Point>> = Vec::new();
    let file_contents = fs::read_to_string("src\\d15\\data_test.txt").expect("Error loading file");
    for path_line in file_contents.split("\r\n") {
        let pairs_this_line = path_line.split(" -> ").map(
            |p| p.split(',').map(
                |n| n.parse::<i32>().unwrap()
            ).collect_vec()
        ).map( |pair| Point{ x: pair[0], y: pair[1] } ).collect_vec();
        
        paths.push(pairs_this_line);
    }
    
    let mut min = paths[0][0];
    let mut max = min;
    for path in &paths {
        for pair in path {
            min = min.min(&pair);
            max = max.max(&pair);
        }
    }
    if !build_floor {
        min.move_by(&Point { x: -1, y: -min.y } );
        max.move_by(&Point { x: 1, y: 1 } );
    }
    else {
        let height = max.y + 3;
        min = Point { x: 500 - height - 3, y: 0 };
        max = Point { x: 500 + height + 3, y: height };
    }
    let (height, width) = (max.y - min.y, max.x - min.x);
    
    let mut shifted_paths: Vec<Vec<Point>> = Vec::new();
    for path in &paths {
        shifted_paths.push(path.iter().map(|p| p.sub(&min)).collect_vec());
    }

    let mut grid = alloc_2d_vec(height as usize, width as usize, GridState::Air);
    for path in &shifted_paths {
        for i in 1..path.len() {
            path[i-1].interpolate(&path[i], |p| grid[p.y as usize][p.x as usize] = GridState::Rock);
        }
    }

    if build_floor {
        Point { x: 0, y: height-1 }.interpolate(&Point { x: width-1, y: height-1 }, |p| grid[p.y as usize][p.x as usize] = GridState::Rock)
    }

    print_grid(&grid);

    let mut rested_sand_count = 0;
    let start_loc = (Point { x: 500, y: 0}).sub(&min);
    let mut loc = start_loc;
    loop {
        match simulate_sand(&loc, &grid) {
            SimulationResult::Falling(p) => { loc = p; }
            SimulationResult::Resting(p) => {
                grid[p.y as usize][p.x as usize] = GridState::Sand;
                //print_grid(&grid);
                rested_sand_count += 1;
                if loc == start_loc {
                    break;
                }
                loc = start_loc;
            }
            SimulationResult::OOB => { print_grid(&grid); break; }
        }
    }
    print_grid(&grid);
    rested_sand_count
}

#[named]
fn part1() {
    let rested_sand_count = do_it_all(false);
    println!("{}: {}", function_name!(), rested_sand_count);
}

#[named]
fn part2() {
    let rested_sand_count = do_it_all(true);
    println!("{}: {}", function_name!(), rested_sand_count);
}

pub fn run() {
    part1();
    part2();
}

// part1: 779
// part2: 1