#![allow(dead_code)]
#![allow(unused_imports)]

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod shared;

fn main() {
    let result = day16::part2::process(include_str!("day16/input.txt"));
    println!("RESULT: {}", result);
}
