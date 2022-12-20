use std::{fs, io::BufRead, cmp, collections::VecDeque};
use ::function_name::named;

use utilities::{alloc_3d_vec, valid_vec_index};

#[derive(Copy, Clone)]
struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[allow(dead_code)]
impl<T> Point3<T> where T: num::PrimInt {
    pub fn expand_to_include(&mut self, other: &Point3<T>) {
        self.x = cmp::max(self.x, other.x);
        self.y = cmp::max(self.y, other.y);
        self.z = cmp::max(self.z, other.z);
    }

    pub fn neighbors(&self) -> Vec<Point3<T>> {
        let step: T = T::one();
        let ret = vec![
            Point3 { x: self.x - step, y: self.y, z: self.z },
            Point3 { x: self.x + step, y: self.y, z: self.z },
            Point3 { x: self.x, y: self.y - step, z: self.z },
            Point3 { x: self.x, y: self.y + step, z: self.z },
            Point3 { x: self.x, y: self.y, z: self.z - step },
            Point3 { x: self.x, y: self.y, z: self.z + step }
        ];
        ret
    }

    pub fn add(self, other: &Point3<T>) -> Point3<T> { Point3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z } }
}

type PointUsize = Point3<usize>;

fn load_data(path: &str) -> (Vec<PointUsize>, PointUsize) {
    let mut ret: Vec<PointUsize> = Vec::new();
    let file_contents = fs::read(path).expect("Error loading file");
    for line in file_contents.lines().map(|line| line.unwrap()) {
        let mut num_iter = line.split(',').map(|n| n.parse::<usize>().unwrap());
        // Shift everything by 1 to add a buffer on the low end
        let new_point = Point3 { x: num_iter.next().unwrap() + 1, y: num_iter.next().unwrap() + 1, z: num_iter.next().unwrap() + 1 };
        ret.push(new_point);
    }

    let mut max = Point3{ x: 0 as usize, y: 0 as usize, z: 0 as usize };
    for p in &ret {
        max.expand_to_include(p);
    }
    // Expand max by 1 to add a buffer on the high end
    (ret, max.add(&Point3 { x: 1, y: 1, z: 1 }))
}

#[named]
fn part1() {
    let (points, max) = load_data("src\\d18\\data.txt");
    let mut space = alloc_3d_vec(max.x + 1, max.y + 1, max.z + 1, false);
    let mut visible_face_count = 0;
    for p in &points {
        space[p.x][p.y][p.z] = true;
        visible_face_count += 6;
        for neighbor in p.neighbors().iter() {
            if space[neighbor.x][neighbor.y][neighbor.z] {
                visible_face_count -= 2;
            }
        }
    }

    println!("{}: {}", function_name!(), visible_face_count);
}

#[derive(Clone, PartialEq)]
enum PointState {
    Solid,
    Exterior,
    Interior
}

fn process_neighbor(space: &mut Vec<Vec<Vec<PointState>>>, point: &Point3<i32>, to_process: &mut VecDeque<Point3<i32>>) {
    if !valid_vec_index(&space, point.x) || !valid_vec_index(&space[0], point.y) || !valid_vec_index(&space[0][0], point.z) {
        return;
    }
    if space[point.x as usize][point.y as usize][point.z as usize] == PointState::Interior {
        space[point.x as usize][point.y as usize][point.z as usize] = PointState::Exterior;
        for neighbor in point.neighbors().iter() {
            to_process.push_back(*neighbor);
        }
    }
}

#[named]
fn part2() {
    let (points, max) = load_data("src\\d18\\data.txt");
    let mut exterior_face_count = 0;
    let mut space = alloc_3d_vec(max.x + 1, max.y + 1, max.z + 1, PointState::Interior);
    for p in &points {
        space[p.x][p.y][p.z] = PointState::Solid;
    }

    // Start with the a point at the edge (which we know is outside)
    let mut to_process: VecDeque<Point3<i32>> = VecDeque::new();
    to_process.push_back(Point3{ x: 0, y: 0, z: 0 });

    // Do the work
    while !to_process.is_empty() {
        let point = to_process.pop_front().unwrap();
        process_neighbor(&mut space, &point, &mut to_process);
    }

    // Count exterior edges
    for p in &points {
        for neighbor in p.neighbors().iter() {
            if space[neighbor.x][neighbor.y][neighbor.z] == PointState::Exterior {
                exterior_face_count += 1;
            }
        }
    }

    println!("{}: {}", function_name!(), exterior_face_count);
}

pub fn run() {
    part1();
    part2();
}

// part1: 4604
// part2: 2604