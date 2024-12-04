fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[path = "../src/day01/mod.rs"]
mod day01;

#[divan::bench(max_time = 1)]
fn day1() {
    let _ = day01::part2::process(divan::black_box(include_str!("../src/day01/input.txt")));
}

#[path = "../src/day02/mod.rs"]
mod day02;

#[divan::bench(max_time = 1)]
fn day2() {
    let _ = day02::part2::process(divan::black_box(include_str!("../src/day02/input.txt")));
}

#[path = "../src/day03/mod.rs"]
mod day03;

#[divan::bench(max_time = 1)]
fn day3() {
    let _ = day03::part2::process(divan::black_box(include_str!("../src/day03/input.txt")));
}

#[path = "../src/day04/mod.rs"]
mod day04;

#[divan::bench(max_time = 1)]
fn day4() {
    let _ = day04::part2::process(divan::black_box(include_str!("../src/day04/input.txt")));
}