use std::{fs};
use ::function_name::named;
use itertools::Itertools;

struct Directory {
    pub name: String,
    pub size: u32,
    pub children: Vec<Directory>
}

#[named]
fn part1(is_part_1: bool) {
    let file_contents = fs::read_to_string("src\\d7\\data.txt").expect("Error loading file");
    let mut lines = file_contents.split("\r\n").collect::<Vec<&str>>();
    lines.push("$  ");
    let mut root: Directory = Directory { name: "/".to_string(), size: 0, children: Vec::new() };
    let mut path: Vec<&mut Directory> = Vec::new();
    let mut collecting = false;
    for line in lines {
        assert!(line.len()>2);
        if &line[0..2] == "$ " {
            if collecting {
                if !path.is_empty()
                {
                    println!("{}: {}", path.iter().fold(String::new(), |res, dir| res + "/" + &dir.name), path.last().unwrap().size);
                }
                collecting = false;
            }
            if line == "$  " {
                break;
            }
            match &line[2..4] {
                "cd" => {
                    let dir = &line[4..];
                    match dir {
                        " /" => {
                            path.clear();
                            path.push(&mut root);
                            // assert!(path.len() == 1 && &path[0].name == "/");
                        }
                        " .." => {
                            //let depth = path.len();
                            // if depth > 1 {
                            //     path[depth - 2].size += path[depth - 1].size;
                            // }
                            assert!(!path.is_empty());
                            // if path.last().size <= 100000 {

                            // }
                            path.pop();
                            // if !path.is_empty() {
                            //     println!("{}: {}", path.iter().fold(String::new(), |res, dir| res + "/" + &dir.name), path.last().unwrap().size);
                            // }
                        }
                        _ => {
                            //let depth = path.len();
                            let x: &mut Directory = path.last_mut().unwrap().children.iter_mut().find(|c| c.name == line[5..]).unwrap();
                            path.push(x);
                        }
                    }
                    //println!("{}", path.iter().fold(String::new(), |res, dir| res + "/" + dir));
                }
                "ls" => {
                    collecting = true;
                }
                _ => {
                    panic!()
                }
            }
        }
        else {
            assert!(collecting && !path.is_empty());
            let (a, b) = line.split(" ").collect_tuple().unwrap();
            match a {
                "dir" => {
                    path.last_mut().unwrap().children.push(Directory { name: b.to_string(), size: 0, children: Vec::new() } );
                }
                _ => {
                    path.last_mut().unwrap().size += a.parse::<u32>().unwrap();
                }
            }
        }
    }
    // while !path.is_empty() {
    //     full_list.push(path.pop().unwrap());
    // }

    // for dir in &full_list {
    //     println!("{}: {}", dir.name, dir.size);
    // }

    if is_part_1 {
        println!("{}: {}", function_name!(), 0); //full_list.iter().map(|f| f.size).filter(|s| s<=&100000).fold(0, |acc, size| acc + size));
    }
    else {
        // let mut sorted_sizes = full_list.iter().map(|f| f.size).collect::<Vec<u32>>();
        // sorted_sizes.sort();
        // let used_space = sorted_sizes.last().unwrap();
        // let unused_space = 70000000 - used_space;
        // let space_to_free = 30000000 - unused_space;
        // for size in sorted_sizes {
        //     if size > space_to_free {
        //         println!("{}: {}", "part2", size);
        //         break;
        //     }
        // }
    }
}

//#[named]
// fn part2() {
//     println!("{}: {}", function_name!(), "!");
// }

pub fn run() {
    part1(true);
    part1(false);
}

// part1: 1077191
// part2: 5649896