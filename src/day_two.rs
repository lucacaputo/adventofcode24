use std::fs;

use regex::Regex;

fn is_increasing(lst: &[u32]) -> bool {
    for i in 1..lst.len() {
        if lst.get(i - 1).unwrap() <= lst.get(i).unwrap() {
            return false;
        }
    }
    true
}

fn is_decreasing(lst: &[u32]) -> bool {
    for i in 1..lst.len() {
        if lst.get(i - 1).unwrap() >= lst.get(i).unwrap() {
            return false;
        }
    }
    true
}

pub fn day_two() {
    let contents = fs::read_to_string("input_day_two.txt").expect("wrong file");
    let re = Regex::new(r"\n").unwrap();
    let values = Regex::split(&re, &contents);

    let valid_values = values
        .map(|item| {
            item.split_whitespace()
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|series| is_decreasing(&series[..]) || is_increasing(&series[..]))
        .collect::<Vec<_>>();

    let safe_pairs_count = valid_values
        .iter()
        .map(|series| series.iter().zip(series.iter().skip(1)).collect::<Vec<_>>())
        .filter(|series| {
            series.iter().all(|(&a, &b)| {
                let diff = a.abs_diff(b);
                diff <= 3 && diff > 0
            })
        })
        .count();

    println!("{:?}", safe_pairs_count);
}
