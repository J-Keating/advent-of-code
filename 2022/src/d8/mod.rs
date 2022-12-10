use std::{fs, cmp};

use ::function_name::named;
//use itertools::Itertools;

fn mark_view_visible(grid: &Vec<Vec<i32>>, visible: &mut Vec<Vec<bool>>, mut px: i32, mut py: i32, sx: i32, sy: i32) -> i32 {
    let mut max = -1;
    let mut count = 0;
    loop {
        if grid[px as usize][py as usize] > max {
            count += 1;
            visible[px as usize][py as usize] = true;
            max = grid[px as usize][py as usize]
        }
        py += sy;
        px += sx;
        if px < 0 || px >= grid.len() as i32 || py < 0 || py >= grid[0].len() as i32 {
            break;
        }
    }
    count
}

#[test]
fn test_count_visible() {
    let grid = vec![vec![0,2,3,4,3,1], vec![1,2,3,2,1,1]];
    let mut visible: Vec<Vec<bool>> = Vec::new();
    visible.resize(grid.len(), Vec::new());
    for i in 0..grid.len() {
        visible[i].resize(grid[0].len(), false);
    }
    assert!(mark_view_visible(&grid, &mut visible, 0, 0, 0, 1) == 4);
    assert!(visible[0] == vec![true, true, true, true, false, false]);
    assert!(mark_view_visible(&grid, &mut visible, 0, 5, 0, -1) == 3);
    assert!(visible[0] == vec![true, true, true, true, true, true])
}

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d8\\data.txt").expect("Error loading file");
    let file_lines = file_contents.split("\r\n").collect::<Vec<&str>>();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in file_lines {
        grid.push(line.chars().map(|c| (c as u8 - '0' as u8) as i32).collect());
    }

    let x_max = grid.len();
    let y_max = grid[0].len();

    let mut visible: Vec<Vec<bool>> = Vec::new();
    visible.resize(x_max, Vec::new());
    for row in visible.iter_mut() {
        row.resize(y_max, false);
    }

    for x in 0i32..x_max as i32 {
        mark_view_visible(&grid, &mut visible, x, 0, 0, 1);
        mark_view_visible(&grid, &mut visible, x, y_max as i32 -1, 0, -1);
    }
    for y in 0i32..y_max as i32 {
        mark_view_visible(&grid, &mut visible, 0, y, 1, 0);
        mark_view_visible(&grid, &mut visible, x_max as i32 - 1, y, -1, 0);
    }

    let mut match_count = 0;
    for row in visible {
        match_count += row.iter().filter(|&x| x == &true).count();
    }

    println!("{}: {}", function_name!(), match_count);
}

fn score_in_direction(grid: &Vec<Vec<i32>>, mut px: i32, mut py: i32, sx: i32, sy: i32) -> i32 {
    let height = grid[px as usize][py as usize];
    let mut score = 0;
    loop {
        px += sx;
        py += sy;
        if px < 0 || px >= grid.len() as i32 || py < 0 || py >= grid[0].len() as i32 {
            break;
        }
        if grid[px as usize][py as usize] >= height {
            score += 1;
            break;
        }
        score += 1;
    }
    score
}

fn scenic_score(grid: &Vec<Vec<i32>>, score: &mut Vec<Vec<i32>>, px: i32, py: i32) {
    let coords = [(0,1), (1,0), (0,-1), (-1,0)];
    let this_scores = coords.iter().map(|(x,y)| score_in_direction(grid, px, py, *x, *y));
    score[px as usize][py as usize] = this_scores.fold(1, |p, s| p * s);
}

#[named]
fn part2() {
    let file_contents = fs::read_to_string("src\\d8\\data.txt").expect("Error loading file");
    let file_lines = file_contents.split("\r\n").collect::<Vec<&str>>();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in file_lines {
        grid.push(line.chars().map(|c| (c as u8 - '0' as u8) as i32).collect());
    }

    let x_max = grid.len();
    let y_max = grid[0].len();

    let mut score: Vec<Vec<i32>> = Vec::new();
    score.resize(x_max, Vec::new());
    for row in score.iter_mut() {
        row.resize(y_max, 0);
    }

    for x in 0i32..x_max as i32 {
        for y in 0i32..y_max as i32 {
            scenic_score(&grid, &mut score, x, y);
        }
    }

    let mut max_score = 0;
    for row in score {
        let row_max = row.iter().max();
        max_score = cmp::max(max_score, *row_max.unwrap());
    }

    println!("{}: {}", function_name!(), max_score);
}

pub fn run() {
    part1();
    part2();
}

// part1: 1708
// part2: 504000