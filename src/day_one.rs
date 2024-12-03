use std::fs;

use regex::Regex;

pub fn part_one() {
    let contents = fs::read_to_string("input_day_one.txt").expect("wrong file");
    let full_list = Regex::split(&Regex::new(r"\s+").unwrap(), &contents)
        .map(|el| el.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut side_a = full_list.iter().step_by(2usize).collect::<Vec<_>>();
    let mut side_b = full_list.iter().skip(1).step_by(2usize).collect::<Vec<_>>();

    side_a.sort();
    side_b.sort();

    let sum = side_a
        .into_iter()
        .zip(side_b.into_iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("{}", sum);
}

pub fn part_two() {
    let contents = fs::read_to_string("input_day_one.txt").expect("wrong file");
    let full_list = Regex::split(&Regex::new(r"\s+").unwrap(), &contents)
        .map(|el| el.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let side_a = full_list.iter().step_by(2usize).collect::<Vec<_>>();
    let side_b = full_list.iter().skip(1).step_by(2usize).collect::<Vec<_>>();

    let pd = side_a
        .iter()
        .map(|&item| {
            side_b
                .iter()
                .filter_map(move |&second_item| {
                    if item == second_item {
                        return Some(1);
                    }
                    None
                })
                .sum::<u32>()
                * item
        })
        .sum::<u32>();

    println!("{pd}")
}
