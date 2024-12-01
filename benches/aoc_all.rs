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