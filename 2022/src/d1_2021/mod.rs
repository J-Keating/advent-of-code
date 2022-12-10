use ::function_name::named;
mod data;

#[named]
fn part1() {
    let data = data::get_data();
    let mut inc_count: i32 = 0;
    for i in 1..data.len() {
        if data[i] - data[i-1] > 0 {
            inc_count += 1;
        }
    }
    println!("{}: {}", function_name!(), inc_count);
}

#[named]
fn part2() {
    let data = data::get_data();
    let mut inc_count: i32 = 0;
    for i in 4..data.len() {
        if (data[i-2] + data[i-1] + data[i-0]) - (data[i-3] + data[i-2] + data[i-1]) > 0 {
            inc_count += 1;
        }
    }
    println!("{}: {}", function_name!(), inc_count);
}

pub fn run() {
    part1();
    part2();
}
 