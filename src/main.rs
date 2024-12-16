#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    let result = day14::part2::process(include_str!("day14/input.txt"), 101, 103);
    println!("RESULT: {}", result);
}
