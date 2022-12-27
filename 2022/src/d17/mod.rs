use std::{fs, io::BufRead, cmp, char::MAX, borrow::Borrow};
use ::function_name::named;
use lazy_static::lazy_static;

use itertools::Itertools;
use utilities::{Point, valid_vec_index};

const SHAPE_COUNT: usize = 5;
fn get_shape(shape_id: usize) -> &'static Vec<Point> { 
    lazy_static! {
            static ref SHAPE_DEFINITIONS: [Vec<Point>; SHAPE_COUNT] = [
            // ####
            Vec::from([Point { x: 0, y: 0}, Point { x: 1, y: 0}, Point { x: 2, y: 0}, Point { x: 3, y: 0}]),
            // .#.
            // ###
            // .#.
            Vec::from([Point { x: 1, y: 0}, Point { x: 0, y: -1}, Point { x: 1, y: -1}, Point { x: 2, y: -1}, Point{ x: 1, y: -2}]),
            // ..#
            // ..#
            // ###
            Vec::from([Point { x: 2, y: 0}, Point { x: 2, y: -1}, Point { x: 0, y: -2}, Point { x: 1, y: -2}, Point { x: 2, y: -2}]),
            // #
            // #
            // #
            // #
            Vec::from([Point { x: 0, y: 0}, Point { x: 0, y: -1}, Point { x: 0, y: -2}, Point { x: 0, y: -3}]),
            // ##
            // ##
            Vec::from([Point { x: 0, y: 0}, Point { x: 1, y: 0}, Point { x: 0, y: -1}, Point { x: 1, y: -1}])
        ];
    }
    &SHAPE_DEFINITIONS[shape_id]
}

#[allow(dead_code)]
#[derive(Clone)]
struct Shape {
    location: Point,
    height: i32,
    width: i32,
    relative_locs: &'static Vec<Point>
}

impl Shape {
    pub fn new(loc: &Point, shape_id: usize) -> Shape {
        let locs: &'static Vec<Point> = get_shape(shape_id);
        let width = locs.iter().map(|p| p.x).max().unwrap() + 1;
        let height = num::abs(locs.iter().map(|p| p.y).min().unwrap()) + 1;
        let ret = Shape { location: *loc, relative_locs: locs, width, height };
        ret
    }

    pub fn move_by(&mut self, loc: &Point) {
        self.location = self.location.add(loc);
    }

    pub fn points_if_moved_by(&self, delta: &Point) -> impl Iterator<Item = Point> + '_ {
        let total_delta = self.location.add(delta);
        self.relative_locs.iter().map(move |p| p.add(&total_delta))
    }

    pub fn could_move_by(&self, delta: &Point, chamber: &Chamber) -> bool {
        for p in self.points_if_moved_by(&delta) {
            if p.x < 0 || p.x >= CHAMBER_WIDTH as i32  || !valid_vec_index(&chamber.stones, p.y) {
                return false;
            }
            if chamber.stones[p.y as usize][p.x as usize] {
                return false;
            }
        }
        true
    }

    pub fn turn_to_stone(&self, chamber: &mut Chamber) {
        chamber.first_free_row = cmp::max(chamber.first_free_row, self.location.y as usize + 1);
        for p in self.points_if_moved_by(&Point {x: 0, y: 0}) {
            chamber.stones[p.y as usize][p.x as usize] = true;
        }
    }
}

#[test]
fn test_shapes() {
    let spawn_loc = &Point {x: 2, y: 15};
    let s0 = Shape::new(spawn_loc, 0);
    assert!(s0.width == 4 && s0.height == 1);
    let s1 = Shape::new(spawn_loc, 1);
    assert!(s1.width == 3 && s1.height == 3);
    let s2 = Shape::new(spawn_loc, 2);
    assert!(s2.width == 3 && s2.height == 3);
    let s3 = Shape::new(spawn_loc, 3);
    assert!(s3.width == 1 && s3.height == 4);
    let s4 = Shape::new(spawn_loc, 4);
    assert!(s4.width == 2 && s4.height == 2);
}

const CHAMBER_WIDTH: usize = 7;
struct Chamber {
    stones: Vec<[bool;CHAMBER_WIDTH]>,
    first_free_row: usize,
    bottom_height: usize
}

const MAX_CHAMBER_HEIGHT: usize = 1000000;
impl Chamber {
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const DOWN: Point = Point { x: 0, y: -1 };

    pub fn new() -> Chamber {
        let mut chamber = Chamber { stones: Vec::new(), bottom_height: 0, first_free_row: 0 };
        chamber.stones.reserve(MAX_CHAMBER_HEIGHT);
        chamber
    }

    pub fn grow_to_include_height(&mut self, height_pos: i32) {
        if height_pos >= self.stones.len() as i32 {
            self.stones.resize(height_pos as usize + 1, [false;CHAMBER_WIDTH]);
        }
    }

    // Doesn't ensure allocation of this row
    pub fn compute_first_free_row(&self) -> usize {
        let mut last_empty_row = self.stones.len();
        for (index, row) in self.stones.iter().enumerate().rev() {
            if row.iter().fold(false, |any, &val| any || val) {
                break;
            }
            last_empty_row = index;
        }
        assert!(last_empty_row == self.first_free_row);
        last_empty_row
    }

    pub fn drop_bottom_rows(&mut self) {
        if self.first_free_row > MAX_CHAMBER_HEIGHT - 10 {
            let rows_to_cull = MAX_CHAMBER_HEIGHT - 1000;
            self.bottom_height += rows_to_cull;
            self.stones = self.stones[rows_to_cull..].to_vec();
            self.stones.reserve(MAX_CHAMBER_HEIGHT);
            self.first_free_row -= rows_to_cull;
            println!("{}", self.bottom_height);
        }
    }

    // Doesn't ensure allocation of this row
    pub fn total_tower_height(&self) -> usize {
        self.bottom_height + self.compute_first_free_row()
    }

    pub fn print_top_row(&self) {
        if self.first_free_row > 0 {
            let top_row_with_stones = self.first_free_row - 1;
            println!("{}: |{}|", self.total_tower_height(), self.stones[top_row_with_stones].iter().fold(String::new(), |acc, &v| acc + if v { "#" } else { " " }));
        }
        else {
            println!("0: ---------");
        }
    }

    pub fn print_top_n(&self, n: usize) {
        for row in self.stones.iter().rev().take(n) {
            println!("|{}|", row.iter().fold(String::new(), |acc, &v| acc + if v { "#" } else { " " }));
        }
        println!("---------");
    }

    pub fn set_height(&mut self, new_height: usize) {
        self.bottom_height = new_height - self.compute_first_free_row();
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in self.stones.iter().rev() {
            println!("|{}|", row.iter().fold(String::new(), |acc, &v| acc + if v { "#" } else { " " }));
        }
        println!("---------");
    }
}

#[test]
fn test_chamber() {
    let mut chamber: Chamber = Chamber::new();

    assert!(chamber.compute_first_free_row() == 0);
    
    for i in 0..SHAPE_COUNT {
        let spawn_loc = &Point { x: 2, y: chamber.first_free_row as i32 };
        let mut s = Shape::new(&spawn_loc, i);
        s.move_by(&Point { x: 0, y: 3 + s.height - 1 });
        chamber.grow_to_include_height(s.location.y);
        let dir = if i % 2 == 0 { Chamber::LEFT } else { Chamber::RIGHT };
        while s.could_move_by(&dir, &chamber) {
            s.move_by(&dir);
        }
        while s.could_move_by(&Chamber::DOWN, &chamber) {
            s.move_by(&Chamber::DOWN);
        }
        s.turn_to_stone(&mut chamber);
        println!("First Free Row: {}", chamber.first_free_row);
        chamber.print();
        println!("----------------");
    }
}

fn load_data(path: &str) -> String {
    let file_contents = fs::read(path).expect("Error loading file").lines().map(|l| l.unwrap()).collect_vec();
    assert!(file_contents.len() == 1);
    file_contents[0].clone()
}

fn load_data_and_drop_shapes(path: &str, total_shapes_to_drop: i64) -> usize {
    let mut chamber: Chamber = Chamber::new();
    let mut current_shape: Option<Box<Shape>> = None;
    let mut spawn_shape_index = 0;
    
    let moves_string = load_data(path);
    let moves_buffer = moves_string.chars().collect_vec();
    let mut move_index: usize = 0;
    let mut shape_count:i64 = 0;
    while shape_count < total_shapes_to_drop || current_shape.is_some()  {
        if move_index == 0 { // && spawn_shape_index == 0 {
            println!("{},{}", shape_count, chamber.total_tower_height());
            //chamber.print_top_n(20);
            // Sad, sad Excel voodoo...  y = mx + b...
            //   Shape Count = 1700x + 1691
            //   Height = 2654x + 2627
            // This code jumps from x==13 to x==588235290, which is hard-coded to a few iterations below 1,000,000,000,000... *shrug*
            if shape_count == 23791 {
                assert!(chamber.total_tower_height() == 37129);
                shape_count = 999999994691;
                chamber.set_height(1561176462287);
            }
        }

        if current_shape.is_none() {
            let spawn_loc = &Point { x: 2, y: chamber.first_free_row as i32 };
            let mut new_shape: Box<Shape> = Box::new(Shape::new(&spawn_loc, spawn_shape_index));
            new_shape.move_by(&Point { x: 0, y: 3 + new_shape.height - 1 });
            chamber.grow_to_include_height(new_shape.location.y);
            spawn_shape_index = (spawn_shape_index + 1) % SHAPE_COUNT;

            current_shape = Some(new_shape);
        }
        let dir = match moves_buffer[move_index] {
            '<' => { Chamber::LEFT }
            '>' => { Chamber::RIGHT }
            _ => { panic!() }
        };
        // Lateral movement
        let mut current_shape_mut = current_shape.unwrap();
        if current_shape_mut.could_move_by(&dir, &chamber) {
            current_shape_mut.move_by(&dir);
        }
        if current_shape_mut.could_move_by(&Chamber::DOWN, &chamber) {
            current_shape_mut.move_by(&Chamber::DOWN);
            current_shape = Some(current_shape_mut);
        }
        else {
            shape_count += 1;
            current_shape_mut.turn_to_stone(&mut chamber);
            current_shape = None;

            chamber.drop_bottom_rows()
            //chamber.print();
        }
        move_index = (move_index + 1) % moves_buffer.len();
    }
    chamber.total_tower_height()
}

#[named]
fn part1() {
    let tower_height = load_data_and_drop_shapes("src\\d17\\data.txt", 2022);
    println!("{}: {}", function_name!(), tower_height);
}

#[named]
fn part2() {
    // 1,000,000,000,000
    let shape_count: i64 = 1000000000000;
    let tower_height = load_data_and_drop_shapes("src\\d17\\data.txt", shape_count);
    println!("{}: {}", function_name!(), tower_height);
}

pub fn run() {
    //part1();
    part2();
}

// part1: 3124
// part2: 40
