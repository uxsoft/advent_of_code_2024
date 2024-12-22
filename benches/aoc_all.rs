fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[path = "../src/main.rs"]
mod aoc;

#[divan::bench(max_time = 1)]
fn day1() {
    let _ = aoc::day01::part2::process(divan::black_box(include_str!("../src/day01/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day2() {
    let _ = aoc::day02::part2::process(divan::black_box(include_str!("../src/day02/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day3() {
    let _ = aoc::day03::part2::process(divan::black_box(include_str!("../src/day03/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day4() {
    let _ = aoc::day04::part2::process(divan::black_box(include_str!("../src/day04/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day5() {
    let _ = aoc::day05::part2::process(divan::black_box(include_str!("../src/day05/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day6() {
    let _ = aoc::day06::part2::process(divan::black_box(include_str!("../src/day06/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day7() {
    let _ = aoc::day07::part2::process(divan::black_box(include_str!("../src/day07/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day8() {
    let _ = aoc::day08::part2::process(divan::black_box(include_str!("../src/day08/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day9() {
    let _ = aoc::day09::part2::process(divan::black_box(include_str!("../src/day09/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day10() {
    let _ = aoc::day10::part2::process(divan::black_box(include_str!("../src/day10/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day11() {
    let _ = aoc::day11::part2::process(divan::black_box(include_str!("../src/day11/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day12() {
    let _ = aoc::day12::part2::process(divan::black_box(include_str!("../src/day12/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day13() {
    let _ = aoc::day13::part2::process(divan::black_box(include_str!("../src/day13/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day14() {
    let _ = aoc::day14::part2::process(
        divan::black_box(include_str!("../src/day14/input.txt")),
        101,
        103,
    );
}

#[divan::bench(max_time = 1)]
fn day15() {
    let _ = aoc::day15::part2::process(divan::black_box(include_str!("../src/day15/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day16() {
    let _ = aoc::day16::part2::process(divan::black_box(include_str!("../src/day16/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day17() {
    let _ = aoc::day17::part2::process(divan::black_box(include_str!("../src/day17/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day18() {
    let _ = aoc::day18::part2::process(
        divan::black_box(include_str!("../src/day18/input.txt")),
        aoc::shared::coordinate::Coordinate::new(70, 70),
        1024,
    );
}

#[divan::bench(max_time = 1)]
fn day19() {
    let _ = aoc::day19::part2::process(divan::black_box(include_str!("../src/day19/input.txt")));
}

#[divan::bench(max_time = 1)]
fn day20() {
    let _ = aoc::day20::part2::process(
        divan::black_box(include_str!("../src/day20/input.txt")),
        20,
        100,
    );
}

#[divan::bench(max_time = 1)]
fn day22() {
    let _ = aoc::day22::part2::process(divan::black_box(include_str!("../src/day22/input.txt")));
}