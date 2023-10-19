use ::function_name::named;
use std::{fs, str::FromStr};
use regex::Regex;

const DAY: i32 = 15;
const FEATURE_COUNT: usize = 4;

#[derive(Debug, Clone)]
struct Ingredient {
    features: [i64; FEATURE_COUNT],
    calories: i64,
}

fn load_data(filename: &str) -> Vec<Ingredient> {
    let data = fs::read_to_string(filename).unwrap();
    let mut reindeers = Vec::new();
    let re = Regex::new(r"(.+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
        
    for line in data.lines() {
        for cap in re.captures_iter(line) {
            let _name: String = cap[1].to_string();
            let capacity = i64::from_str(&cap[2]).unwrap();
            let durability = i64::from_str(&cap[3]).unwrap();
            let flavor = i64::from_str(&cap[4]).unwrap();
            let texture = i64::from_str(&cap[5]).unwrap();
            let calories = i64::from_str(&cap[6]).unwrap();
            reindeers.push(Ingredient { features: [ capacity, durability, flavor, texture ], calories: calories });
        }
    }
    reindeers
}

fn compute_score_two_ingredients(ing1: &Ingredient, w1: i64, ing2: &Ingredient, w2: i64) -> i64 {
    let mut score = 1;
    for i in 0..FEATURE_COUNT {
        let s = w1 * ing1.features[i] + w2 * ing2.features[i];
        if s <= 0 {
            return 0;
        }
        score *= s;
    }
    score
}

fn compute_score(ingredients: &Vec<Ingredient>, weights: &Vec<i64>) -> (i64, i64) {
    //assert!(ingredients.len() == 4);
    let mut score = 1;
    for i in 0..FEATURE_COUNT {
        let mut s = 0;
        for j in 0..ingredients.len() {
            s += weights[j] * ingredients[j].features[i];
        }
        if s <= 0 {
            return (0, 0);
        }
        score *= s;
    }
    let mut calories = 0;
    for j in 0..ingredients.len() {
        calories += weights[j] * ingredients[j].calories;
    }
    (score, calories)
}

#[named]
fn part1_test() {
    let ingredients = load_data(&format!("src\\d{}\\data_test.txt", DAY));
    let mut max_score = 0;
    for i in 0..=100 {
        let score = compute_score_two_ingredients(&ingredients[0], i, &ingredients[1], 100 - i);
        if score > max_score {
            max_score = score;
        }
    }
    println!("{}: {}", function_name!(), max_score);
}

#[named]
fn part1() {
    let ingredients = load_data(&format!("src\\d{}\\data.txt", DAY));
    assert!(ingredients.len() <= 4);
    let mut max_score = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                for l in 0..=100 {
                    if (i + j + k + l) == 100 {
                        let (score, _) = compute_score(&ingredients, &[i, j, k, l].to_vec());
                        if score > max_score {
                            max_score = score;
                        }
                    }
                }
            }
        }
    }
    println!("{}: {}", function_name!(), max_score);
}

#[named]
fn part2() {
    let ingredients = load_data(&format!("src\\d{}\\data.txt", DAY));
    assert!(ingredients.len() <= 4);
    let mut max_score = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                for l in 0..=100 {
                    if (i + j + k + l) == 100 {
                        let (score, calories) = compute_score(&ingredients, &[i, j, k, l].to_vec());
                        if score > max_score && calories == 500 {
                            //println!("{}: {} {} {} {} {} {}", function_name!(), i, j, k, l, score, calories);
                            max_score = score;
                        }
                    }
                }
            }
        }
    }
    println!("{}: {}", function_name!(), max_score);
}

pub fn run() {
    println!("Day {}:", DAY);
    part1_test();
    part1();
    part2();
}

// part1_test: 62842880
// part1: 222870
// part2: 117936