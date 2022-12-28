use std::{fs, io::BufRead, collections::HashMap};

use ::function_name::named;
use itertools::Itertools;
//use lazy_static::lazy_static;

#[derive(Clone)]
struct Equation {
    pub left: String,
    pub operation: char,
    pub right: String,
}

#[derive(Clone)]
enum Action {
    Equation(Equation),
    Number(i64)
}

struct Troop {
    pub monkeys: HashMap<String, Action>
}

impl Troop {
    pub fn new() -> Troop { Troop { monkeys: HashMap::new() } }
    pub fn add_monkey(&mut self, name: &String, action: &String) {
        let action_tokens = action.trim().split(' ').collect_vec();
        match action_tokens.len() {
            1 => { 
                self.monkeys.insert(name.clone(), Action::Number(action.parse::<i64>().unwrap()));
            }
            3 => {
                let eq = Equation {
                    left: action_tokens[0].to_string(),
                    operation: action_tokens[1].chars().nth(0).unwrap(),
                    right: action_tokens[2].to_string()
                };
                self.monkeys.insert(name.clone(), Action::Equation(eq.clone()));
            }
            _ => { panic!() }
        }
    }

    pub fn eval_monkey(&mut self, name: &String) -> i64 {
        let monkey_action = self.monkeys.get(name).unwrap().clone();
        match monkey_action {
            Action::Equation(eq) => {
                let left_num = self.eval_monkey(&eq.left);
                let right_num = self.eval_monkey(&eq.right);
                match eq.operation {
                    '+' => { left_num + right_num }
                    '-' => { left_num - right_num }
                    '*' => { left_num * right_num }
                    '/' => { left_num / right_num }
                    _ => { panic!() }
                }
            }
            Action::Number(num) => {
                num
            }
        }
    }

    pub fn eval_monkey_human(&mut self, name: &String) -> (i64, bool) {
        let monkey_action = self.monkeys.get(name).unwrap().clone();
        match monkey_action {
            Action::Equation(eq) => {
                let num;
                let (left_num, left_human) = self.eval_monkey_human(&eq.left);
                let (right_num, right_human) = self.eval_monkey_human(&eq.right);
                if name == "root" {
                    num = if left_num < right_num { -1 } else { if left_num > right_num { 1 } else { 0 } };
                    // if num == 0 {
                    //     println!("SOLVED!!!!");
                    // }
                    //println!("root: {}({}) {} {}({})", left_num, left_human, if num == -1 { '<' } else { if num == 1 { '>' } else { '=' } }, right_num, right_human);
                }
                else {
                    num = match eq.operation {
                        '+' => { left_num + right_num }
                        '-' => { left_num - right_num }
                        '*' => { left_num * right_num }
                        '/' => { left_num / right_num }
                        _ => { panic!() }
                    };
                }
                (num, left_human || right_human)
            }
            Action::Number(num) => {
                // if name == "humn" {
                //     print!("humn: {} ==> ", num);
                // }
                (num, name == "humn")
            }
        }
    }

    pub fn test_human_value(&mut self, human_value: i64) -> i64 {
        self.monkeys.insert("humn".to_string(), Action::Number(human_value));
        self.eval_monkey_human(&"root".to_string()).0
    }

    pub fn solve_for_human_value(&mut self) -> i64 {
        //let human_action = self.monkeys.get("humn").unwrap();
        let (mut low, mut high) = (i64::MIN/100, i64::MAX/100);
        let (mut low_result, mut high_result) = (self.test_human_value(low), self.test_human_value(high));
        let mut human_value;
        //let mut human_value_last;
        loop {
            assert!(low_result != high_result && low_result != 0 && high_result != 0);

             human_value = low + (high - low) / 2;
            let result = self.test_human_value(human_value);
            if result == 0 {
                break;
            }
            //let temp = human_value;
            if result == low_result {
                low = human_value;
                low_result = result;
            }
            else {
                high = human_value;
                high_result = result;
            }
        }
        human_value
    }
}

fn load_data(path: &str) -> Troop {
    let mut troop = Troop::new();
    let file_contents = fs::read(path).expect("Error loading file");
    for line_tokens in file_contents.lines().map(|line| line.unwrap().split(':').map(|t| t.trim().to_string()).collect_vec()) {
        assert!(line_tokens.len() == 2);
        troop.add_monkey(&line_tokens[0], &line_tokens[1]);
    }
    troop
}

#[named]
fn part1() {
    let mut troop = load_data("src\\d21\\data.txt");
    println!("{}: {}", function_name!(), troop.eval_monkey(&"root".to_string()));
}

#[named]
fn part2() {
    let mut troop = load_data("src\\d21\\data.txt");

    println!("{}: {}", function_name!(), troop.solve_for_human_value());
}

pub fn run() {
    part1();
    part2();
}

// Day21:
// part1: 63119856257960
// part2: 3006709232464