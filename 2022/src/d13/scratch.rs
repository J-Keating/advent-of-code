use std::{fs, collections::VecDeque, time::Instant};

use ::function_name::named;
use itertools::Itertools;
use lazy_static::lazy_static;

enum node {
    num(i32),
    list(Vec<node>),
    Empty,
    ERROR
}


// fn add_elements_to_list(line_in: &str, list: &mut Vec<Node>) {
//     let mut line = line_in.to_string();
//     while !line.is_empty() {
//         if line.chars().nth(0) == Some('[') {
//             let end_pos = line.rfind(']').unwrap();
//             let mut v: Vec<Node> = Vec::new();
//             add_elements_to_list(&line[1..end_pos], &mut v);
//             list.push(Node::List(v));
//             line = line[end_pos+1..].to_string();
//         }
//         else {
//             let comma_pos_search = line.find(',');
//             if comma_pos_search.is_some() {
//                 let n = line[0..comma_pos_search.unwrap()].parse::<i32>().unwrap();
//                 line = line[comma_pos_search.unwrap()+1..].to_string();
//                 list.push(Node::Num(n));
//                 println!("XXX Num:{},  line:{}", n, line);
//             }
//             else {
//                 let n = line.parse::<i32>().unwrap();
//                 line = String::new();
//                 list.push(Node::Num(n));
//                 println!("XXX Num:{},  line:{}", n, line);
//             }
//         }
//     }
// }

fn add_elements_to_list(line_in: &str, list: &mut Vec<node>) {
    let mut line = line_in.to_string();
    while !line.is_empty() {
        if line.chars().nth(0) == Some('[') {
            let end_pos = line.rfind(']').unwrap();
            let mut v: Vec<node> = Vec::new();
            add_elements_to_list(&line[1..end_pos], &mut v);
            list.push(node::list(v));
            line = line[end_pos+1..].to_string();
        }
        else {
            let comma_pos_search = line.find(',');
            if comma_pos_search.is_some() {
                let n = line[0..comma_pos_search.unwrap()].parse::<i32>().unwrap();
                line = line[comma_pos_search.unwrap()+1..].to_string();
                list.push(node::num(n));
                println!("XXX num:{},  line:{}", n, line);
            }
            else {
                let n = line.parse::<i32>().unwrap();
                line = String::new();
                list.push(node::num(n));
                println!("XXX num:{},  line:{}", n, line);
            }
        }
    }
}

fn parse_line(line_in: &str) -> node {
    let mut vec_stack: Vec<Vec<node>> = Vec::new();
    let mut ret: node = node::Empty;
    let mut pos: usize = 0;
    while pos < line_in.len() {
        pos = match line_in.chars().nth(pos).unwrap() {
            '[' => {
                vec_stack.push(Vec::new());
                pos + 1
            }
            ']' => {
                assert!(!vec_stack.is_empty());
                let currently_finished_list = vec_stack.pop();
                if vec_stack.is_empty() {
                    ret = node::list(currently_finished_list.unwrap());
                    break;
                }
                vec_stack[0].push(node::list(currently_finished_list.unwrap()));
                pos + 1
            }
            '0'..='9' => {
                assert!(!vec_stack.is_empty());
                let start_pos = pos;
                while line_in.chars().nth(pos).unwrap().is_numeric() {
                    pos += 1;
                }
                vec_stack[0].push(node::num(line_in[start_pos..pos].parse::<i32>().unwrap()));
                pos
            }
            ',' => { pos + 1 }
            _ => { panic!() }
        }
    }

    ret
}

fn print_elements(list: &Vec<node>) {
    for element in list.iter() {
        match element {
            node::num(n) => { print!("{},", n); }
            node::list(v) => {
                print!("[");
                print_elements(v);
                print!("]");
            }
            node::Empty | node::ERROR => panic!()
        }
    }
    println!("");
}

#[test]
fn test_list_parsing() {
    let mut l: Vec<node> = Vec::new();
    add_elements_to_list("[1,2,3]", &mut l);
    print_elements(&l);
    println!("");
    l.clear();
    add_elements_to_list("[1,2,3,[4]]", &mut l);
    print_elements(&l);
    println!("");
}

#[named]
fn part1() {
    let file_contents = fs::read_to_string("src\\d13\\data_test.txt").expect("Error loading file");
    for line in file_contents.split("\r\n") {
        if !line.is_empty() {
            let mut l: Vec<node> = Vec::new();
            add_elements_to_list(line, &mut l);
            print_elements(&l);
            println!("");
        }
    }
}

#[named]
fn part2() {
    let now = Instant::now();

    // Any 'a' in the grid can be a valid start location
    let valid_start_loc_test: Option<fn(char) -> bool> = Some(|c| c == 'a');
    let shortest_path = find_shortest_path(valid_start_loc_test);
    println!("{}: {} ({} ms)", function_name!(), shortest_path, now.elapsed().as_micros() as f32 / 1000.0);
}

pub fn run() {
    part1();
    part2();
}

part1: 350
part2: 349


        //let number_list = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        //while !line.is_empty() {
            let comma_pos_search = line.find(',');
            match comma_pos_search {
                Some(comma_pos) => {
                    let num = line[0..comma_pos].parse::<i32>().unwrap();
                    line = line[comma_pos+1..].to_string();
                    println!("XXX num:{},  line:{}", num, line);
                }
                None => {
                    let num = line.parse::<i32>().unwrap();
                    println!("XXX num:{},  line:{}", num, line);
                }
            }
        //}

        fn add_elements_to_list(line_in: &str, list: &mut Vec<node>) -> node {
            let mut line = line_in.to_string();
            let mut ret: node = node::ERROR;
            let length = line.len();
            let curr_pos: usize = 0;
            while curr_pos < length {
                if line.chars().nth(curr_pos) == Some('[') {
                    assert!(line.chars().nth(length-1) == Some(']'));
                    let end_pos = line.rfind(']').unwrap();
                    let v: Vec<node> = Vec::new();
                    add_elements_to_list(&line[0..end_pos], &mut v);
                    ret = node::list(v);
                }
                else if length > 0{
                    let mut comma_pos_search = line.find(',');
                    while comma_pos_search.is_some() {
                        let num = line[0..comma_pos_search.unwrap()].parse::<i32>().unwrap();
                        line = line[comma_pos_search.unwrap()+1..].to_string();
                        println!("XXX num:{},  line:{}", num, line);
                        comma_pos_search = line.find(',');
                    }
                    let num = line.parse::<i32>().unwrap();
                    println!("XXX num:{},  line:{}", num, line);
                    line = 
                }
            }
            ret
        }        


        for i in 0..std::cmp::min(list_l.len(), list_r.len()) {
            if list_l[i] < list_r[i] {
                -1
            }
            else if list_l[i] > list_r[i] {
                1
            }
        }
        All equal, which is exhausted?
        if list_l.len() < list_r.len() {
            -1
        }
        else if 


//        10 ..####B######################..
