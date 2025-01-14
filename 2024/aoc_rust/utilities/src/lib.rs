use std::{cmp, collections::HashSet, fmt::{self, Display}, fs};
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
pub struct PointXY<T> {
    pub x: T,
    pub y: T
}

#[allow(dead_code)]
impl<T> PointXY<T> where T: num::PrimInt + num::Signed + num::Zero + std::ops::Neg<Output = T>  {
    pub fn new() -> PointXY<T> { PointXY { x: T::zero(), y: T::zero() } }

    pub fn min(self, other: &PointXY<T>) -> PointXY<T> { PointXY { x: cmp::min(self.x, other.x), y: cmp::min(self.y, other.y) } }
    pub fn max(self, other: &PointXY<T>) -> PointXY<T> { PointXY { x: cmp::max(self.x, other.x), y: cmp::max(self.y, other.y) } }

    pub fn add(self, other: &PointXY<T>) -> PointXY<T> { PointXY { x: self.x + other.x, y: self.y + other.y } }
    pub fn sub(self, other: &PointXY<T>) -> PointXY<T> { PointXY { x: self.x - other.x, y: self.y - other.y } }
    pub fn mul(self, multiplier: T) -> PointXY<T> { PointXY { x: self.x * multiplier, y: self.y * multiplier } }
    pub fn div(self, divisor: T) -> PointXY<T> { PointXY { x: self.x / divisor, y: self.y / divisor } }

    pub fn neg(self) -> PointXY<T> { PointXY { x: -self.x, y: -self.y } }

    pub fn move_by(&mut self, other: &PointXY<T>) -> &PointXY<T> {
        *self = self.add(&other);
        self
    }

    pub fn manhattan_dist(self, other: &PointXY<T>) -> T {
        let diff = other.sub(&self);
        abs(diff.x) + abs(diff.y)
    }

    pub fn interpolate<F>(self, other: &PointXY<T>, mut callback: F) where F: FnMut(&PointXY<T>)
    {
        let diff = other.sub(&self);
        assert!((diff.x == T::zero() || diff.y == T::zero()) && (diff.x != T::zero()  || diff.y != T::zero()));
        let len = (diff.x + diff.y).abs();
        let step = diff.div(len);
        let mut curr = self;
        for _ in 0..=len.to_i64().unwrap() {
            callback(&curr);
            curr.move_by(&step);
        }
    }
}

impl<T> fmt::Display for PointXY<T> where T: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct PointRC {
    pub r: i32,
    pub c: i32
}

#[allow(dead_code)]
impl PointRC {
    pub fn new() -> PointRC { PointRC { r: 0, c: 0 } }

    pub fn min(self, other: &PointRC) -> PointRC { PointRC { r: cmp::min(self.r, other.r), c: cmp::min(self.c, other.c) } }
    pub fn max(self, other: &PointRC) -> PointRC { PointRC { r: cmp::max(self.r, other.r), c: cmp::max(self.c, other.c) } }

    pub fn add(self, other: &PointRC) -> PointRC { PointRC { r: self.r + other.r, c: self.c + other.c } }
    pub fn sub(self, other: &PointRC) -> PointRC { PointRC { r: self.r - other.r, c: self.c - other.c } }
    pub fn mul(self, multiplier: i32) -> PointRC { PointRC { r: self.r * multiplier, c: self.c * multiplier } }
    pub fn div(self, divisor: i32) -> PointRC { PointRC { r: self.r / divisor, c: self.c / divisor } }

    pub fn neg(self) -> PointRC { PointRC { r: -self.r, c: -self.c } }

    pub fn move_by(&mut self, other: &PointRC) -> &PointRC {
        *self = self.add(&other);
        self
    }

    pub fn manhattan_dist(self, other: &PointRC) -> i32 {
        let diff = other.sub(&self);
        abs(diff.r) + abs(diff.c)
    }

    pub fn neighbors_cardinal(&self) -> Vec<PointRC> {
        vec![
            PointRC { r: self.r - 1, c: self.c },
            PointRC { r: self.r + 1, c: self.c },
            PointRC { r: self.r, c: self.c - 1 },
            PointRC { r: self.r, c: self.c + 1 }
        ]
    }

    pub fn interpolate<F>(self, other: &PointRC, mut callback: F) where F: FnMut(&PointRC)
    {
        let diff = other.sub(&self);
        assert!((diff.r == 0 || diff.c == 0) && (diff.r != 0  || diff.c != 0));
        let len = abs(diff.r + diff.c);
        let step = diff.div(len);
        let mut curr = self;
        for _ in 0..=len {
            callback(&curr);
            curr.move_by(&step);
        }
    }
}

impl fmt::Display for PointRC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}

impl fmt::Debug for PointRC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
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


pub struct Board<T: Clone> {
    pub data: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T> Board<T> where T: Clone {
    pub fn new(height: usize, width: usize, val: T) -> Board<T> where T: Clone {
        Board {
            data: alloc_2d_vec::<T>(height, width, val),
            width,
            height,
        }
    }

    pub fn at(&self, loc: &PointRC) -> &T where T: Clone {
        &self.data[loc.r as usize][loc.c as usize]
    }

    #[allow(dead_code)]
    pub fn print(&self) where T: Display {
        for line in &self.data {
            //println!("{}", line.iter().collect::<String>());
            for v in line.iter() {
                print!("{}", v);
            }
            println!("");
        }
    }

    pub fn find_first(&self, target: T) -> Option<PointRC> where T: Clone + std::cmp::PartialEq {
        for r in 0..self.height {
            for c in 0..self.width {
                if self.data[r][c] == target {
                    return Some(PointRC { r: r as i32, c: c as i32 });
                }
            }
        }
        None
    }

    pub fn find_all(&self, target: T) -> HashSet<PointRC> where T: Clone + std::cmp::PartialEq {
        let mut ret = HashSet::new();
        for r in 0..self.height {
            for c in 0..self.width {
                if self.data[r][c] == target {
                    ret.insert(PointRC { r: r as i32, c: c as i32 });
                }
            }
        }
        ret
    }

    pub fn in_bounds(&self, loc: &PointRC) -> bool {
        loc.r >= 0 && loc.r < self.height as i32 && loc.c >= 0 && loc.c < self.width as i32
    }

    pub fn load_data_chars(path: &str) -> Board<char> {
        let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
        let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
        let height = file_lines.len();
        let width = file_lines[0].len();
        let mut board = Board::<char>::new(height, width, ' ');
        for (row, line) in file_lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                board.data[row][col] = c;
            }
        }
        board
    }

    pub fn load_data_int(path: &str) -> Board<i32> {
        let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
        let file_lines = file_contents_as_string.lines().collect::<Vec<&str>>();
        let height = file_lines.len();
        let width = file_lines[0].len();
        let mut board = Board::<i32>::new(height, width, 0);
        for (row, line) in file_lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let num = c.to_digit(10);
                board.data[row][col] = if num.is_some() { num.unwrap() as i32 } else { -1 };
            }
        }
        board
    }
}
