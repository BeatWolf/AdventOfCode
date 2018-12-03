use std::io;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let input = File::open("data/day1/input.txt").unwrap();


    let total: i32 = BufReader::new(input).lines().map(|line| {
        line.unwrap().parse::<i32>().unwrap()
    }).sum();

    print!("Current frequency is {}", total)
}
