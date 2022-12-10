use ::function_name::named;

mod d1_2021;

#[named]
fn part1() {
    let data = d1_2021::get_data();
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
    let data = d1_2021::get_data();
    let mut inc_count: i32 = 0;
    for i in 4..data.len() {
        if (data[i-2] + data[i-1] + data[i-0]) - (data[i-3] + data[i-2] + data[i-1]) > 0 {
            inc_count += 1;
        }
    }
    println!("{}: {}", function_name!(), inc_count);
}

fn main() {
    part1();
    part2();
}
 