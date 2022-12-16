use std::{fs, cmp::Ordering};

use ::function_name::named;
use itertools::Itertools;

//use itertools::Itertools;
//use lazy_static::lazy_static;

#[derive(PartialEq, PartialOrd, Clone)]
enum Node {
    Num(i32),
    List(Vec<Node>),
    Empty
}

fn parse_line(line_in: &str) -> Node {
    let mut vec_stack: Vec<Vec<Node>> = Vec::new();
    let mut ret: Node = Node::Empty;
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
                    ret = Node::List(currently_finished_list.unwrap());
                    break;
                }
                vec_stack.last_mut().unwrap().push(Node::List(currently_finished_list.unwrap()));
                pos + 1
            }
            '0'..='9' => {
                assert!(!vec_stack.is_empty());
                let start_pos = pos;
                while line_in.chars().nth(pos).unwrap().is_numeric() {
                    pos += 1;
                }
                vec_stack.last_mut().unwrap().push(Node::Num(line_in[start_pos..pos].parse::<i32>().unwrap()));
                pos
            }
            ',' => { pos + 1 }
            _ => { panic!() }
        }
    }

    ret
}

fn compare_nodes(l: &Node, r: &Node) -> std::cmp::Ordering {  // true if right order
    match (l, r) {
        (Node::Num(num_l), Node::Num(num_r)) => {
            let diff = num_l - num_r;
            if diff == 0 { Ordering::Equal }
            else if diff < 0 { Ordering::Less }
            else { Ordering::Greater }
        }
        (Node::Num(num), Node::List(_)) => {
            let mut vec: Vec<Node> = Vec::new();
            vec.push(Node::Num(*num));
            compare_nodes(&Node::List(vec), r)
        }
        (Node::List(_), Node::Num(num)) => {
            let mut vec: Vec<Node> = Vec::new();
            vec.push(Node::Num(*num));
            compare_nodes(l, &Node::List(vec))
        }
        (Node::List(list_l), Node::List(list_r)) => {
            for i in 0..std::cmp::min(list_l.len(), list_r.len()) {
                let res = compare_nodes(&list_l[i], &list_r[i]);
                if res != Ordering::Equal {
                    return res;
                }
            }
            // All equal, which is exhausted?
            if list_l.len() < list_r.len() {
                return Ordering::Less;
            }
            else if list_l.len() > list_r.len() {
                return Ordering::Greater;
            }
            Ordering::Equal
        }
        (_, Node::Empty) | (Node::Empty, _) => { panic!(); }
    }
}

#[allow(dead_code)]
fn print_elements(node_to_print: &Node) {
    match node_to_print {
        Node::Num(number) => { print!("{}", number); }
        Node::List(l) => {
            print!("[");
            for e in l {
                print_elements(e);
                print!(",")
            }
            print!("]");
        }
        Node::Empty => panic!()
    }
}

#[test]
fn test_list_parsing() {
    let mut l = parse_line("[1,2,3]");
    print_elements(&l);
    println!("");
    l = parse_line("[1,2,3,[4]]");
    print_elements(&l);
    println!("");
}

#[named]
fn part1() {
    let mut index = 1;
    let mut sum_of_right_order = 0;
    let file_contents = fs::read_to_string("src\\d13\\data.txt").expect("Error loading file");
    for pair in file_contents.split("\r\n\r\n") {
        let packet_strings = pair.split("\r\n").collect::<Vec<&str>>();
        let (l, r) = packet_strings.iter().map(|p| parse_line(p)).collect_tuple().unwrap();
        // print_elements(&l);
        // print_elements(&r);
        let comp_result = compare_nodes(&l, &r);
        if comp_result == Ordering::Less {
            sum_of_right_order += index;
        }
        index += 1;
    }
    println!("{}: {}", function_name!(), sum_of_right_order);
}

#[named]
fn part2() {
    let div_packet_1 = parse_line("[[2]]");
    let div_packet_2 = parse_line("[[6]]");

    let mut all_packets: Vec<Node> = Vec::new();
    all_packets.push(div_packet_1.clone());
    all_packets.push(div_packet_2.clone());

    let file_contents = fs::read_to_string("src\\d13\\data.txt").expect("Error loading file");
    for line in file_contents.split("\r\n") {
        if !line.is_empty() {
            all_packets.push(parse_line(line));
        }
    }
    all_packets.sort_by(|l,r| compare_nodes(l, r));
    let div_packet_1_index = all_packets.binary_search_by(|n| compare_nodes(n, &div_packet_1)).unwrap();
    let div_packet_2_index = all_packets.binary_search_by(|n| compare_nodes(n, &div_packet_2)).unwrap();

    println!("{}: {}", function_name!(), (div_packet_1_index + 1) * (div_packet_2_index + 1));
}

pub fn run() {
    part1();
    part2();
}

// part1: 4809
// part2: 22600