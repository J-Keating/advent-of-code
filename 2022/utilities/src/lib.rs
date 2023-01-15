use std::{cmp, fmt};
use ::num::abs;

pub fn valid_vec_index<T>(v: &Vec<T>, index: i32) -> bool {
    0 <= index && index < v.len() as i32
}

pub fn alloc_2d_vec<T: Clone>(height: usize, width: usize, val: T) -> Vec<Vec<T>> {
    let mut ret: Vec<Vec<T>> = Vec::new();
    ret.resize(height, Vec::new());
    for i in 0..height {
        ret[i].resize(width, val.clone());
    }
    ret
}

pub fn alloc_3d_vec<T: Clone>(max_x: usize, max_y: usize, max_z: usize, val: T) -> Vec<Vec<Vec<T>>> {
    let mut ret: Vec<Vec<Vec<T>>> = Vec::new();
    ret.resize(max_x, Vec::new());
    for x in 0..max_x {
        ret[x].resize(max_y, Vec::new());
        for y in 0..max_y {
            ret[x][y].resize(max_z, val.clone());
        }
    }
    ret
}

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[allow(dead_code)]
impl Point {
    pub fn new() -> Point { Point { x: 0, y: 0 } }

    pub fn min(self, other: &Point) -> Point { Point { x: cmp::min(self.x, other.x), y: cmp::min(self.y, other.y) } }
    pub fn max(self, other: &Point) -> Point { Point { x: cmp::max(self.x, other.x), y: cmp::max(self.y, other.y) } }

    pub fn add(self, other: &Point) -> Point { Point { x: self.x + other.x, y: self.y + other.y } }
    pub fn sub(self, other: &Point) -> Point { Point { x: self.x - other.x, y: self.y - other.y } }
    pub fn div(self, divisor: i32) -> Point { Point { x: self.x / divisor, y: self.y / divisor } }

    pub fn neg(self) -> Point { Point { x: -self.x, y: -self.y } }

    pub fn move_by(&mut self, other: &Point) -> &Point {
        *self = self.add(&other);
        self
    }

    pub fn manhattan_dist(self, other: &Point) -> i32 {
        let diff = other.sub(&self);
        abs(diff.x) + abs(diff.y)
    }

    pub fn interpolate<F>(self, other: &Point, mut callback: F) where F: FnMut(&Point)
    {
        let diff = other.sub(&self);
        assert!((diff.x == 0 || diff.y == 0) && (diff.x != 0  || diff.y != 0));
        let len = abs(diff.x + diff.y);
        let step = diff.div(len);
        let mut curr = self;
        for _ in 0..=len {
            callback(&curr);
            curr.move_by(&step);
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Copy, Clone)]
pub struct Point3<T> {
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