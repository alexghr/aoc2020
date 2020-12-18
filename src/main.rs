extern crate aoc2020;

use std::fs;

use aoc2020::*;

fn main() {
    run_day1();
    run_day2();
    run_day3();
}

fn run_day3() {
    let data = read_lines("data/day3.txt");
    println!("======== Day 3 ========");
    println!("Part 1: {}", day3::count_trees_knight_move(&data));
    println!("Part 2: {}", day3::check_slopes(&data));
    println!("");
}

fn run_day2() {
    let data = read_lines("data/day2.txt");
    println!("======== Day 2 ========");
    println!("Part 1: {}", day2::count_sled_passwords(&data));
    println!("Part 2: {}", day2::count_toboggan_passwords(&data));
    println!("");
}

fn run_day1() {
    let data = read_int_data("data/day1.txt");
    println!("======== Day 1 ========");
    println!("Part 1: {}", day1::find_product(&data).expect("must work"));
    println!("Part 2: {}", day1::find_product_triplet(&data).expect("must work"));
    println!("");
}

fn read_int_data(filename: &str) -> Vec<i32> {
    read_lines(&filename)
        .iter()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .split_terminator('\n')
        .map(|line| String::from(line))
        .collect()
}
