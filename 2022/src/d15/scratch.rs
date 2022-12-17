use std::{fs, fmt, iter::Product, collections::HashSet, cmp::Ordering};
use ::num::abs;

use ::function_name::named;
use itertools::{Itertools, all};

//use itertools::Itertools;
//use lazy_static::lazy_static;

use num::iter::Range;
use utilities::{alloc_2d_vec, Point};

struct SensorBeaconPair {
    pub sensor: Point,
    pub beacon: Point,
    pub distance: i32
}

impl SensorBeaconPair {
    fn count_row_overlap(&self, row: i32) -> i32 {
        let row_dist = abs(self.sensor.y - row);
        if row_dist > self.distance {
            return 0;
        }
        let overlap_per_side = self.distance - row_dist;
        2 * overlap_per_side + 1
    }

    fn get_row_overlap_bounds(&self, row: i32) -> Option<(i32, i32)> {
        let row_dist = abs(self.sensor.y - row);
        if row_dist > self.distance {
            return None
        }
        let overlap_per_side = self.distance - row_dist;
        Some(( self.sensor.x - overlap_per_side, self.sensor.x + overlap_per_side))
    }
}

#[test]
fn test_row_overlap() {
    let sensor = Point { x: 8, y: 7 };
    let beacon = Point { x: 2, y: 10 };
    let distance = sensor.manhattan_dist(&beacon);
    assert!(distance == 9);
    let sb = SensorBeaconPair { sensor, beacon, distance };
    assert!(sb.count_row_overlap(7) == 19);
    assert!(sb.count_row_overlap(-2) == 1);
    assert!(sb.count_row_overlap(-1) == 3);
    assert!(sb.count_row_overlap(16) == 1);
    assert!(sb.count_row_overlap(15) == 3);
}

fn load_data(path: &str) -> Vec<SensorBeaconPair> {
    let mut sensor_data: Vec<SensorBeaconPair> = Vec::new();
    let file_contents = fs::read_to_string(path).expect("Error loading file");
    for path_line in file_contents.split("\r\n") {
        let tokenized = path_line.split(['=', ',', ':']).collect_vec();
        assert!(tokenized.len() == 8 && tokenized[0] == "Sensor at x" && tokenized[6] == " y");
        let sensor = Point { x: tokenized[1].parse::<i32>().unwrap(), y: tokenized[3].parse::<i32>().unwrap() };
        let beacon = Point { x: tokenized[5].parse::<i32>().unwrap(), y: tokenized[7].parse::<i32>().unwrap() };
        let distance = sensor.manhattan_dist(&beacon);
        assert!(distance == beacon.manhattan_dist(&sensor));
        
        sensor_data.push(SensorBeaconPair { sensor, beacon, distance });
    }
    sensor_data
}

#[named]
fn part1() {
    let test_row = 10;
    let data = load_data("src\\d15\\data_test.txt");
    let mut unique_points: HashSet<i32> = HashSet::new();
    for s in &data {
        //println!("{:?} -> {:?} ({})", s.sensor.to_string(), s.beacon.to_string(), s.distance);
        match s.get_row_overlap_bounds(test_row) {
            Some((min,max)) => {
                //println!(": [{}..={}] ({})", min, max, max - min + 1);
                for p in min..=max {
                    unique_points.insert(p);
                    //println!("{:?}", unique_points)
                }
            }
            None => {}
        }
    }
    for s in &data {
        if s.beacon.y == test_row { unique_points.remove(&s.beacon.x); }
        if s.sensor.y == test_row { unique_points.remove(&s.sensor.x); }
    }
    println!("{}: {}", function_name!(), unique_points.len());
}

#[derive(PartialEq, PartialOrd)]
enum MinOrMax {
    Min,
    Max
}

struct ScannerBoundary {
    min_or_max: MinOrMax,
    value: i32
}

#[named]
fn part2() {
    let mut final_frequency = 0;
    let data = load_data("src\\d15\\data.txt");
    for test_row in 0..=4000000 {
        let mut boundary_crossings: Vec<ScannerBoundary> = Vec::new();
        for s in &data {
            //println!("{:?} -> {:?} ({})", s.sensor.to_string(), s.beacon.to_string(), s.distance);
            match s.get_row_overlap_bounds(test_row) {
                Some((min,max)) => {
                    //println!(": [{}..={}] ({})", min, max, max - min + 1);
                    boundary_crossings.push(ScannerBoundary { min_or_max: MinOrMax::Min, value: min });
                    boundary_crossings.push(ScannerBoundary { min_or_max: MinOrMax::Max, value: max });
                }
                None => {}
            }
        }
        boundary_crossings.sort_by(|a, b| {
            let value_test = a.value.cmp(&b.value);
            if value_test != Ordering::Equal { value_test } else {
                match (&a.min_or_max, &b.min_or_max) {
                    (MinOrMax::Min, MinOrMax::Min) | (MinOrMax::Max, MinOrMax::Max) => { Ordering::Equal }
                    (MinOrMax::Min, MinOrMax::Max) => { Ordering::Less }
                    (MinOrMax::Max, MinOrMax::Min) => { Ordering::Greater }
                }
            }
        });
        let (min, max) = (boundary_crossings.first().unwrap().value - 1, boundary_crossings.last().unwrap().value + 1);
        //println!("min={}, max={}", min, max);
        let mut next_crossing = 0;
        assert!(min < boundary_crossings[next_crossing].value);
        let mut clear_count = 0;
        let mut count_of_current_sensors_sensing = 0;
        for i in min..=max {
            while next_crossing < boundary_crossings.len() && i == boundary_crossings[next_crossing].value && boundary_crossings[next_crossing].min_or_max == MinOrMax::Min {
                count_of_current_sensors_sensing += 1;
                next_crossing += 1;
            }
            if count_of_current_sensors_sensing > 0 {
                clear_count += 1;
            }
            else if 0 <= i && i <= 4000000 {
                println!("!!!!{}: ({}, {})", function_name!(), i, test_row);
                final_frequency = i * 4000000 + test_row;
            }
            while next_crossing < boundary_crossings.len() && i == boundary_crossings[next_crossing].value && boundary_crossings[next_crossing].min_or_max == MinOrMax::Max {
                count_of_current_sensors_sensing -= 1;
                next_crossing += 1;
            }
        }
    }
    println!("{}: {}", function_name!(), final_frequency);
}

pub fn run() {
    part1();
    part2();
}

// part1: 5040643
// part2: 0