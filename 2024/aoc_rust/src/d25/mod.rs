use function_name::named;
use std::fs;

const DAY: &str = "d25";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d25\data_test.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d25\data.txt";
}

fn load_data(path: &str) -> Vec<[[char; 5]; 7]> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    let grid_chunks = file_contents_as_string.split("\r\n\r\n").collect::<Vec<&str>>();
    grid_chunks.iter().map(|chunk| {
        let mut grid = [[' '; 5]; 7];
        let lines = chunk.lines().collect::<Vec<&str>>();
        assert!(lines.len() == 7);
        for (i, line) in lines.iter().enumerate() {
            assert!(line.len() == 5);
            grid[i] = line.chars().collect::<Vec<char>>()[0..5].try_into().unwrap();
        }
        grid
    }).collect()
}

fn compute_heights(schematics: [[char; 5]; 7]) -> (bool, [i32; 5]) {
    let mut heights = [0; 5];
    for r in schematics.iter() {
        r.iter().enumerate().filter(|(_, &c)| c == '#').for_each(|(c, _)| heights[c] += 1);
    }
    // Its a lock if the top row is all '#'
    (schematics[0][0] == '#', heights)
}

fn key_fits_lock(key: [i32; 5], lock: [i32; 5]) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 7)
}

#[named]
fn part1() {
    use real_data as data;
    let mut lock_heights = Vec::<[i32; 5]>::new();
    let mut keys_heights = Vec::<[i32; 5]>::new();
    let schematics = load_data(data::FILENAME);
    // Sort heights into locks and keys
    for schematic in schematics.iter() {
        let (is_lock, heights) = compute_heights(*schematic);
        match is_lock {
            true => &mut lock_heights,
            false => &mut keys_heights,
        }.push(heights)
    }
    // Count all combinations of keys that fit into locks    
    let fit_count = lock_heights.iter().map(|&lock| {
        keys_heights.iter().filter(|&&key| key_fits_lock(key, lock)).count()
    }).sum::<usize>();

    println!("{}: {}", function_name!(), fit_count);
}

#[named]
fn part2() {
    println!("{}: {}", function_name!(), "Wheeee!");
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 3508
// part2: Wheeee!
