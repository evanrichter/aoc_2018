#![allow(dead_code)]
extern crate gif;
extern crate indicatif;
extern crate regex;
use std::env;

mod day1;
mod day10;
mod day16;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    match env::args().skip(1).next() {
        Some(filename) => {
            use day19::solve;
            solve(filename);
        }
        None => println!("Error: specify input file"),
    }
}
