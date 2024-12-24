use itertools::Itertools;

#[derive(PartialEq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn is_safe(report: &Vec<u32>) -> bool {
    fn check(report: &Vec<u32>, i: usize, a: u32, trend: Trend, skips_remaining: u32) -> bool {
        if i >= report.len() {
            return true;
        }

        let b = report[i];

        if trend == Trend::Increasing && a < b && a.abs_diff(b) <= 3 {
            // good, continue
            return check(report, i + 1, b, trend, skips_remaining);
        } else if trend == Trend::Decreasing && a > b && a.abs_diff(b) <= 3 {
            // good, continue
            return check(report, i + 1, b, trend, skips_remaining);
        } else if skips_remaining > 0 {
            // use a skip
            return check(report, i + 1, a, trend, skips_remaining - 1);
        } else {
            return false;
        }
    }

    let decision = check(report, 1, report[0], Trend::Increasing, 1)
        || check(report, 2, report[1], Trend::Increasing, 0)
        || check(report, 1, report[0], Trend::Decreasing, 1)
        || check(report, 2, report[1], Trend::Decreasing, 0);
        
    // let correct = is_safe_brute(report);

    // if decision != correct {
    //     println!("Miss! {report:?} => {incr_0} || {incr_1} || {decr_0} || {decr_1}");
    // }

    return decision;
}

fn is_safe_brute(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let r = report
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != i)
            .map(|(_, item)| *item)
            .collect_vec();
        if super::part1::is_safe(&r) {
            return true;
        }
    }
    return false;
}

pub fn process(input: &str) -> usize {
    let matrix = super::parse(input);

    let safe = matrix.into_iter().filter(is_safe).count();

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = process(input);
        assert_eq!(4, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);
        assert_eq!(373, result);
    }
}
