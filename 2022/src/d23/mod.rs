use std::{fs, cmp};

use ::function_name::named;
use itertools::Itertools;
use utilities::{Point, alloc_2d_vec};
use lazy_static::lazy_static;

struct TestDirection {
    primary_step: Point,
    side_steps: [Point;2]
}

const TEST_DIRECTION_COUNT: usize = 4;
fn get_test_direction(index: usize) -> &'static TestDirection { 
    lazy_static! {
            static ref TEST_DIRECTIONS: [TestDirection; TEST_DIRECTION_COUNT] = [
            // North
            TestDirection{ primary_step: Point { x: 0, y: -1 }, side_steps: [ Point { x: -1, y: -1 }, Point { x: 1, y: -1 } ] },
            // South
            TestDirection{ primary_step: Point { x: 0, y: 1 }, side_steps: [ Point { x: -1, y: 1 }, Point { x: 1, y: 1 } ] },
            // West
            TestDirection{ primary_step: Point { x: -1, y: 0 }, side_steps: [ Point { x: -1, y: -1 }, Point { x: -1, y: 1 } ] },
            // East
            TestDirection{ primary_step: Point { x: 1, y: 0 }, side_steps: [ Point { x: 1, y: -1 }, Point { x: 1, y: 1 } ] },
        ];
    }
    &TEST_DIRECTIONS[index % TEST_DIRECTION_COUNT]
}


#[derive(Clone, PartialEq)]
enum ProposedMoveState {
    Open,
    ElfFrom(Point),
    Jammed
}

#[derive(Clone)]
struct LocationData {
    pub has_elf: bool,
    pub proposed_move: ProposedMoveState,
}

const BOARD_BUFFER: usize = 100;
struct Board {
    pub data: Vec<Vec<LocationData>>,
    pub height: usize,
    pub width: usize,
    pub some_move: bool
}

impl Board {
    pub fn new(height: usize, width: usize) -> Board {
        Board {
            data: alloc_2d_vec::<LocationData>(height, width, LocationData { has_elf: false, proposed_move: ProposedMoveState::Open}),
            width,
            height,
            some_move: false
        }
    }
    pub fn data_at_point(&self, loc: &Point) -> &LocationData {
        &self.data[loc.y as usize][loc.x as usize]
    }
    pub fn data_at_point_mut(&mut self, loc: &Point) -> &mut LocationData {
        &mut self.data[loc.y as usize][loc.x as usize]
    }
    pub fn clear_boundary(&self, test_loc: &Point) -> bool {
        for y in test_loc.y-1..=test_loc.y+1 {
            for x in test_loc.x-1..=test_loc.x+1 {
                if self.data[y as usize][x as usize].has_elf && (x != test_loc.x || y != test_loc.y) {
                    return false
                }
            }
        }
        true
    }
    pub fn proposed_move_result(&self, test_loc: &Point, direction_index: usize) -> Option<(Point, ProposedMoveState)> {
        assert!(self.data_at_point(test_loc).has_elf);
        let test_direction = get_test_direction(direction_index);
        let move_to_location = test_loc.add(&test_direction.primary_step);
        if  !self.data_at_point(&move_to_location).has_elf && 
            !self.data_at_point(&test_loc.add(&test_direction.side_steps[0])).has_elf && 
            !self.data_at_point(&test_loc.add(&test_direction.side_steps[1])).has_elf {
            // No elves in the way.  Want to move...
            let new_state = match self.data_at_point(&move_to_location).proposed_move {
                ProposedMoveState::Open => { ProposedMoveState::ElfFrom(*test_loc) }
                ProposedMoveState::ElfFrom(_) => { ProposedMoveState::Jammed }
                ProposedMoveState::Jammed => { ProposedMoveState::Jammed }
            };
            return Some((move_to_location, new_state));
        }
        None
    }
    // This only works if you don't want to write any internal data in the callback.
    fn for_each_elf(&self, mut callback: impl FnMut(&Point)) {
        for row_index in 0..self.height {
            for col_index in 0..self.width {
                if self.data[row_index][col_index].has_elf {
                    callback(&Point { x: col_index as i32, y: row_index as i32 });
                }
            }
        }
    }
    // This allows for writing loops over elf positions which mutate the data (unlike the above "callback" approach)
    // I feel like it should be done as an iterator, but can't be bothered...
    pub fn elf_locations(&self) -> Vec<Point> {
        let mut ret: Vec<Point> = Vec::new();
        for row_index in 0..self.height {
            for col_index in 0..self.width {
                if self.data[row_index][col_index].has_elf {
                    ret.push(Point { x: col_index as i32, y: row_index as i32 });
                }
            }
        }
        ret
    }
    pub fn phase1(&mut self, first_direction_index: usize) {
        self.some_move = false;
        for loc in self.elf_locations() {
            if !self.clear_boundary(&loc) {
                for direction_index in first_direction_index..first_direction_index+TEST_DIRECTION_COUNT {
                    match self.proposed_move_result(&loc, direction_index) {
                        Some((loc_to_change, state_to_set)) => {
                            self.data_at_point_mut(&loc_to_change).proposed_move = state_to_set;
                            break;
                        }
                        None => { }
                    }
                }
            }
        }
    }
    pub fn phase2(&mut self) {
        for row_index in 0..self.height {
            for col_index in 0..self.width {
                match self.data[row_index][col_index].proposed_move {
                    ProposedMoveState::Open | ProposedMoveState::Jammed => {}
                    ProposedMoveState::ElfFrom(from_loc) => { 
                        assert!(self.data[from_loc.y as usize][from_loc.x as usize].has_elf);
                        self.data[from_loc.y as usize][from_loc.x as usize].has_elf = false;
                        self.data[row_index][col_index].has_elf = true;
                        self.some_move = true;
                     }
                }
                self.data[row_index][col_index].proposed_move = ProposedMoveState::Open;
            }
        }
    }

    pub fn bounds(&self) -> (Point, Point) {
        let mut min = Point { x: self.data[0].len() as i32 - 1, y: self.data.len() as i32 - 1 };
        let mut max = Point { x: 0, y: 0 };
        self.for_each_elf(|loc| {
            min.x = cmp::min(min.x, loc.x);
            min.y = cmp::min(min.y, loc.y);
            max.x = cmp::max(max.x, loc.x);
            max.y = cmp::max(max.y, loc.y);
        });
        (min, max)
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        let (min, max) = self.bounds();
        for row in &self.data[min.y as usize..=max.y as usize] {
            println!("{}", row[min.x as usize..=max.x as usize].iter().fold(String::new(), |acc, ld| acc + &(if ld.has_elf { "#" } else { "." }).to_string()));
        }
    }
    pub fn count_empty_spaces(&self) -> usize {
        let mut empty_count: usize = 0;
        let (min, max) = self.bounds();
        for row in &self.data[min.y as usize..=max.y as usize] {
            empty_count += row[min.x as usize..=max.x as usize].iter().filter(|ld| !ld.has_elf).count();
        }
        empty_count
    }
}

fn load_data(path: &str) -> Board {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let file_contents = file_contents_as_string.split("\r\n").collect_vec();
    let height = BOARD_BUFFER + file_contents.len() + BOARD_BUFFER;
    let width = BOARD_BUFFER + file_contents[0].len() + BOARD_BUFFER;
    let mut board = Board::new(height, width);
    for (row, line) in file_contents.iter().enumerate() {
        for (col, state) in line.chars().enumerate() {
            board.data[BOARD_BUFFER + row][BOARD_BUFFER + col].has_elf = match state {
                '.' => { false }
                '#' => { true }
                _ => { panic!() } 
            };
        }
    }
    board
}

#[named]
fn part1() {
    let mut board = load_data("src\\d23\\data.txt");
    let mut direction_index: usize = 0;
    for _ in 0..10 {
        board.phase1(direction_index);
        board.phase2();
        direction_index += 1;
    }
    let (min, max) = board.bounds();
    println!("{}: ({},{}) -> ({},{})  Empty: {}", function_name!(), min.y, min.x, max.y, max.x, board.count_empty_spaces());
}

#[named]
fn part2() {
    let mut board = load_data("src\\d23\\data.txt");
    let mut direction_index: usize = 0;
    loop {
        board.phase1(direction_index);
        board.phase2();
        direction_index += 1;
        if !board.some_move {
            break;
        }
    }
    let (min, max) = board.bounds();
    println!("{}: ({},{}) -> ({},{})  Iterations: {}", function_name!(), min.y, min.x, max.y, max.x, direction_index);
}

pub fn run() {
    part1();
    part2();
}

// Day23:
// part1: (96,96) -> (177,176)  Empty: 3923
// part2: (87,86) -> (225,225)  Iterations: 1019