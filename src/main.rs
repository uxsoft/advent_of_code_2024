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

fn main() {
    let stones = day11::part2::process(include_str!("day11/input.txt"));
    println!("RESULT: {}", stones);
}
