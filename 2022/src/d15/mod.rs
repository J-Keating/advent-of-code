use std::{fs, collections::HashSet, cmp::Ordering, time::Instant};
use ::num::abs;

use ::function_name::named;
use itertools::{Itertools};

//use itertools::Itertools;
//use lazy_static::lazy_static;

use utilities::{Point};

struct SensorBeaconPair {
    pub sensor: Point,
    pub beacon: Point,
    pub distance: i32
}

#[allow(dead_code)]
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
    let now = Instant::now();

    let test_row = 2000000;
    let data = load_data("src\\d15\\data.txt");
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
    println!("{}: {} ({:.3} sec)", function_name!(), unique_points.len(), now.elapsed().as_secs_f32());
}

#[derive(PartialEq, PartialOrd, Debug)]
enum MinOrMax {
    Min,
    Max
}

#[derive(Debug)]
struct ScannerBoundary {
    min_or_max: MinOrMax,
    value: i32
}

#[named]
fn part2() {
    let now = Instant::now();

    let mut final_frequency: i64 = 0;
    let scan_bounds = 4000000;
    let data = load_data("src\\d15\\data.txt");
    for test_row in 0..=scan_bounds {
        let mut boundary_crossings: Vec<ScannerBoundary> = Vec::new();
        for s in &data {
            match s.get_row_overlap_bounds(test_row) {
                Some((min,max)) => {
                    //println!(": [{}..={}] ({})", min, max, max - min + 1);
                    boundary_crossings.push(ScannerBoundary { min_or_max: MinOrMax::Min, value: min });
                    boundary_crossings.push(ScannerBoundary { min_or_max: MinOrMax::Max, value: max + 1 });
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
        assert!(boundary_crossings[0].min_or_max == MinOrMax::Min);
        let mut count_of_current_sensors_sensing = 0;
        for crossing in &boundary_crossings {
            count_of_current_sensors_sensing += match crossing.min_or_max {
                MinOrMax::Min => { 1 }
                MinOrMax::Max => { -1 }
            };
            if crossing.value > scan_bounds {
                break;
            }
            if count_of_current_sensors_sensing == 0 {
                println!("!!!!{}: ({}, {})", function_name!(), crossing.value, test_row);
                final_frequency = (crossing.value as i64) * 4000000 + test_row as i64;
            }
        }
    }
    println!("{}: {} ({:.3} sec)", function_name!(), final_frequency, now.elapsed().as_secs_f32());
}

pub fn run() {
    part1();
    part2();
}

// part1: 5040643
// !!!!part2: (2754143, 3214126)
// part2: 11016575214126