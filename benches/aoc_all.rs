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

#[path = "../src/day05/mod.rs"]
mod day05;
#[divan::bench(max_time = 1)]
fn day5() {
    let _ = day05::part2::process(divan::black_box(include_str!("../src/day05/input.txt")));
}

#[path = "../src/day06/mod.rs"]
mod day06;
#[divan::bench(max_time = 1)]
fn day6() {
    let _ = day06::part2::process(divan::black_box(include_str!("../src/day06/input.txt")));
}

#[path = "../src/day07/mod.rs"]
mod day07;
#[divan::bench(max_time = 1)]
fn day7() {
    let _ = day07::part2::process(divan::black_box(include_str!("../src/day07/input.txt")));
}

#[path = "../src/day08/mod.rs"]
mod day08;
#[divan::bench(max_time = 1)]
fn day8() {
    let _ = day08::part2::process(divan::black_box(include_str!("../src/day08/input.txt")));
}

#[path = "../src/day09/mod.rs"]
mod day09;
#[divan::bench(max_time = 1)]
fn day9() {
    let _ = day09::part2::process(divan::black_box(include_str!("../src/day09/input.txt")));
}

#[path = "../src/day10/mod.rs"]
mod day10;
#[divan::bench(max_time = 1)]
fn day10() {
    let _ = day10::part2::process(divan::black_box(include_str!("../src/day10/input.txt")));
}

#[path = "../src/day11/mod.rs"]
mod day11;
#[divan::bench(max_time = 1)]
fn day11() {
    let _ = day11::part2::process(divan::black_box(include_str!("../src/day11/input.txt")));
}

#[path = "../src/day12/mod.rs"]
mod day12;
#[divan::bench(max_time = 1)]
fn day12() {
    let _ = day12::part2::process(divan::black_box(include_str!("../src/day12/input.txt")));
}

#[path = "../src/day13/mod.rs"]
mod day13;
#[divan::bench(max_time = 1)]
fn day13() {
    let _ = day13::part2::process(divan::black_box(include_str!("../src/day13/input.txt")));
}

#[path = "../src/day14/mod.rs"]
mod day14;
#[divan::bench(max_time = 1)]
fn day14() {
    let _ = day14::part2::process(divan::black_box(include_str!("../src/day14/input.txt")), 101, 103);
}