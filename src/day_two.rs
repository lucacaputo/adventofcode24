use std::fs;

pub fn day_two() {
    let contents = fs::read_to_string("input_day_two.txt").expect("wrong file");
    println!("{:?}", contents)
}
