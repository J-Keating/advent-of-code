use function_name::named;
use itertools::Itertools;
use std::fs;

const DAY: &str = "d22";

#[allow(dead_code)]
mod test_data {
    pub const FILENAME: &str = r"src\d22\data_test2.txt";
}
#[allow(dead_code)]
mod real_data {
    pub const FILENAME: &str = r"src\d22\data.txt";
}

fn load_data(path: &str) -> Vec<i64> {
    let file_contents_as_string = fs::read_to_string(path).expect("Error loading file");
    file_contents_as_string.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn mix(secret: i64, given: i64) -> i64 {
    secret ^ given
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut next = prune(mix(secret, secret * 64));
    next = prune(mix(next, next / 32));
    prune(mix(next, next * 2048))
}

#[test]
fn test_stuff() {
    assert!(mix(42, 15) == 37);
    assert!(prune(100000000) == 16113920);
    assert!(next_secret(123) == 15887950);
    assert!(next_secret(15887950) == 16495136);
}

#[named]
fn part1() {
    use real_data as data;
    let numbers = load_data(data::FILENAME);
    let final_sum = numbers.iter().map(|&n| {
        let mut curr = n;
        for _ in 0..2000 {
            curr = next_secret(curr);
        }
        curr
    }).sum::<i64>();
    println!("{}: {:?}", function_name!(), final_sum);
}

fn encode(a: i32, b: i32, c: i32, d: i32) -> i32 {
    assert!(a >= -9 && a <= 9);
    assert!(b >= -9 && b <= 9);
    assert!(c >= -9 && c <= 9);
    assert!(d >= -9 && d <= 9);
    ((a + 9) * 19 * 19 * 19) + ((b + 9) * 19 * 19) + ((c + 9) * 19) + (d + 9)
}

fn decode(n: i32) -> (i32, i32, i32, i32) {
    let a = n / (19 * 19 * 19);
    let b = (n / (19 * 19)) % 19;
    let c = (n / 19) % 19;
    let d = n % 19;
    (a - 9, b - 9, c - 9, d - 9)
}

#[named]
fn part2() {
    use real_data as data;
    let mut scores = Vec::new();
    scores.resize(19*19*19*19, 0);
    let mut seen_already = Vec::new();
    seen_already.resize(19*19*19*19, false);
    let numbers: Vec<i64> = load_data(data::FILENAME);
    for n in numbers.iter() {
        seen_already.fill(false);
        let mut curr = *n;
        // Sequence with 2000 elements, including the start number
        let prices = std::iter::once(curr).chain((0..2000).map(|_| {
            curr = next_secret(curr);
            curr
        })).map(|s| (s % 10) as i32).collect::<Vec<i32>>();
        // Vector with all differences
        let diffs = prices.iter().tuple_windows().map(|(a,b)| b-a).collect::<Vec::<i32>>();
        assert!(diffs.len() == 2000 && prices.len() == 2001);
        // Looking at 4 diffs at a time means that the first window is at the fifth element of prices
        let mut price_index = 4;
        diffs.iter().tuple_windows().for_each(|(a,b,c,d)| {
            let n = encode(*a, *b, *c, *d);
            assert!((*a,*b,*c,*d) == decode(n));
            if !seen_already[n as usize] {
                scores[n as usize] += prices[price_index];
                seen_already[n as usize] = true;
            }
            price_index += 1;
        });
//        println!("{}: {:?}", function_name!(), diffs);
    }
    let mut max_index = 0;
    let mut max_score = 0;
    scores.iter().enumerate().for_each(|(i,s)| {
        if *s > max_score {
            max_score = *s;
            max_index = i;
        }
    });
    let (a,b,c,d) = decode(max_index as i32);
    println!("{}: {},{},{},{} => {}", function_name!(), a,b,c,d, max_score);
}

pub fn run() {
    println!("{}:", DAY);
    part1();
    part2();
}

// part1: 13234715490
// part2: 1,-1,0,1 => 1490
