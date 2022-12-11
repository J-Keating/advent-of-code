use std::{collections::{VecDeque}};

use ::function_name::named;
//use itertools::Itertools;
//use lazy_static::lazy_static;

type NumberType = i64;

#[derive(Clone)]
struct Monkey {
    pub items: VecDeque<NumberType>,
    pub operation : fn(NumberType) -> NumberType,
    pub test : fn(NumberType) -> usize,
    pub count: NumberType
}

const TEST_MONKEY_COUNT: usize = 4;
fn get_monkeys_test() -> ([Monkey;TEST_MONKEY_COUNT], NumberType) { 
    let x: [Monkey;TEST_MONKEY_COUNT] = [
        Monkey {
            items: VecDeque::from([79, 98]),
            operation: |x| x*19,
            test: |x| if x % 23 == 0 { 2 } else { 3 },
            count: 0
        },
        Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: |x| x+6,
            test: |x| if x % 19 == 0 { 2 } else { 0 },
            count: 0
        },
        Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: |x| x*x,
            test: |x| if x % 13 == 0 { 1 } else { 3 },
            count: 0
        },
        Monkey {
            items: VecDeque::from([74]),
            operation: |x| x+3,
            test: |x| if x % 17 == 0 { 0 } else { 1 },
            count: 0
        },
    ];
    (x.clone(), 23*19*13*17)
}

const MONKEY_COUNT: usize = 8;
fn get_monkeys() -> ([Monkey;MONKEY_COUNT], NumberType) { 
    let x: [Monkey;MONKEY_COUNT] = [
        Monkey { // 0
            items: VecDeque::from([89, 84, 88, 78, 70]),
            operation: |x| x*5,
            test: |x| if x % 7 == 0 { 6 } else { 7 },
            count: 0
        },
        Monkey { // 1
            items: VecDeque::from([76, 62, 61, 54, 69, 60, 85]),
            operation: |x| x+1,
            test: |x| if x % 17 == 0 { 0 } else { 6 },
            count: 0
        },
        Monkey { // 2
            items: VecDeque::from([83, 89, 53]),
            operation: |x| x+8,
            test: |x| if x % 11 == 0 { 5 } else { 3 },
            count: 0
        },
        Monkey { // 3
            items: VecDeque::from([95, 94, 85, 57]),
            operation: |x| x+4,
            test: |x| if x % 13 == 0 { 0 } else { 1 },
            count: 0
        },
        Monkey { // 4
            items: VecDeque::from([82, 98]),
            operation: |x| x+7,
            test: |x| if x % 19 == 0 { 5 } else { 2 },
            count: 0
        },
        Monkey { // 5
            items: VecDeque::from([69]),
            operation: |x| x+2,
            test: |x| if x % 2 == 0 { 1 } else { 3 },
            count: 0
        },
        Monkey { // 6
            items: VecDeque::from([82, 70, 58, 87, 59, 99, 92, 65]),
            operation: |x| x*11,
            test: |x| if x % 5 == 0 { 7 } else { 4 },
            count: 0
        },
        Monkey { // 7
            items: VecDeque::from([91, 53, 96, 98, 68, 82]),
            operation: |x| x * x,
            test: |x| if x % 3 == 0 { 4 } else { 2 },
            count: 0
        },
    ];
    (x.clone(), 7*17*11*13*19*2*5*3)
}

#[named]
fn part1() {
    let (mut monkeys, _) = get_monkeys_test();

    for _ in 1..21 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut c = monkeys[i].items.pop_front().unwrap();
                c = (monkeys[i].operation)(c) / 3;
                let dest_index = (monkeys[i].test)(c);
                monkeys[dest_index].items.push_back(c);
                monkeys[i].count += 1;
            }
        }
    }

    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<NumberType>>();
    counts.sort();
    let length = counts.len();

    println!("{}: {}", function_name!(), counts[length-2] * counts[length-1]);
}

#[named]
fn part2() {
    let (mut monkeys, magic_number) = get_monkeys();

    for iteration_number in 1..10001 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut c = monkeys[i].items.pop_front().unwrap();
                c = (monkeys[i].operation)(c);
                c -= c/magic_number*magic_number;
                let dest_index = (monkeys[i].test)(c);
                monkeys[dest_index].items.push_back(c);
                monkeys[i].count += 1;
            }
        }
        if vec![1,20,1000,2000,3000,4000,5000].iter().find(|&&i| i==iteration_number).is_some() {
            let counts = monkeys.iter().map(|m| m.count).collect::<Vec<NumberType>>();
            println!("{}:{}", iteration_number, counts.iter().fold(String::new(), |acc,c| acc + "|" + &c.to_string()));
        }
    }

    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<NumberType>>();
    counts.sort();
    let length = counts.len();

    println!("{}: {}", function_name!(), counts[length-2] * counts[length-1]);
}

pub fn run() {
    part1();
    part2();
}
  
// part1: 10605
// 1:|5|7|3|7|2|1|17|15
// 20:|45|218|241|244|191|12|219|76
// 1000:|1165|11538|12107|12092|11106|262|11385|1413
// part2: 14636993466