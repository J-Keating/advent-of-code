use std::{fs, collections::HashMap, collections::HashSet };

use ::function_name::named;
use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    fn get_direction(dir_str: &str) -> Point {
        lazy_static! {
            static ref DIRECTION_MAP: HashMap<&'static str, Point> = HashMap::from([
                ("U", Point { x:0, y:1 } ),
                ("D", Point { x:0, y:-1 } ),
                ("L", Point { x:-1, y:0 } ),
                ("R", Point { x:1, y:0 } ),
            ]);
        }
        
        DIRECTION_MAP.get(dir_str).unwrap().clone( )
    }

    fn move_by(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn move_toward(&mut self, other: &Point) {
        let (x, y) = (other.x - self.x, other.y - self.y);
        if num::abs(x) > 1 || num::abs(y) > 1 {
            let step = Point { x: num::clamp(x, -1, 1), y: num::clamp(y, -1, 1) };
            self.move_by(&step);
        }
    }
}

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d9\\data.txt").expect("Error loading file");
    let file_lines = file_contents.split("\r\n").collect::<Vec<&str>>();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::new();

    for line in file_lines {
        let mut tokens = line.split(' ');
        let dir = Point::get_direction(tokens.next().unwrap());
        let num = tokens.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..num {
            head.move_by(&dir);
            tail.move_toward(&head);
            visited.insert(tail);
        }
    }

    println!("{}: {}", function_name!(), visited.len());
}

#[named]
fn part2() {
    let file_contents = fs::read_to_string("src\\d9\\data.txt").expect("Error loading file");
    let file_lines = file_contents.split("\r\n").collect::<Vec<&str>>();

    let mut snake: [Point; 10] = [ Point { x:0, y:0 }; 10];
    let mut visited: HashSet<Point> = HashSet::new();

    for line in file_lines {
        let (dir_str, num_str) = line.split(' ').collect_tuple().unwrap();
        let dir = Point::get_direction(dir_str);
        let num = num_str.parse::<i32>().unwrap();
        for _ in 0..num {
            snake[0].move_by(&dir);
            for i in 1..10 {
                snake[i].move_toward(&snake[i-1].clone());
            }
            visited.insert(snake[9]);
        }
    }

    println!("{}: {}", function_name!(), visited.len());
}

pub fn run() {
    part1();
    part2();
}

// part1: 6087
// part2: 2493