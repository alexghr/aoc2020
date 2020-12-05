extern crate aoc2020;

use std::fs;

use aoc2020::day1;

fn main() {
    run_day1();
}

fn run_day1() {
    let data = read_int_data("data/day1.txt");
    println!("{}", day1::find_product(&data).expect("must work"));
}

fn read_int_data(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}