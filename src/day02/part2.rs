use itertools::Itertools;
use miette::Result;

fn is_safe(report: &Vec<u32>) -> bool {
    fn test_increments(report: &Vec<u32>) -> bool {
        let mut removals_left = 1;
        let mut attempt_removal = false;
        let mut removed_a: u32 = 0;

        for (a, b) in report.into_iter().tuple_windows() {
            if attempt_removal && removed_a < *b && removed_a.abs_diff(*b) <= 3 {
                attempt_removal = false;
                continue;
            } else if !attempt_removal && a < b && a.abs_diff(*b) <= 3 {
                continue;
            } else if removals_left > 0 {
                removals_left -= 1;
                attempt_removal = true;
                removed_a = *a;
                continue;
            } else {
                return false;
            }
        }

        return true;
    }

    fn test_decrements(report: &Vec<u32>) -> bool {
        let mut removals_left = 1;
        let mut attempt_removal = false;
        let mut removed_a: u32 = 0;

        for (a, b) in report.into_iter().tuple_windows() {
            if attempt_removal && removed_a > *b && removed_a.abs_diff(*b) <= 3 {
                attempt_removal = false;
                continue;
            } else if !attempt_removal && a > b && a.abs_diff(*b) <= 3 {
                continue;
            } else if removals_left > 0 {
                removals_left -= 1;
                attempt_removal = true;
                removed_a = *a;
                continue;
            } else {
                return false;
            }
        }

        return true;
    }

    fn test_increments2(report: &Vec<u32>) -> bool {
        let mut a = report[0];
        let mut problems = 0;
        for i in 1..report.len() {
            let b = report[i];

            if a < b && a.abs_diff(b) <= 3 {
                a = b;
                continue;
            } else {
                problems += 1;
            }
        }

        return problems <= 1;
    }

    fn test_decrements2(report: &Vec<u32>) -> bool {
        let mut a = report[0];
        let mut problems = 0;
        for i in 1..report.len() {
            let b = report[i];

            if a > b && a.abs_diff(b) <= 3 {
                a = b;
                continue;
            } else {
                //println!("Removing {b}");
                problems += 1;
            }
        }

        return problems <= 1;
    }

    let inc = test_increments2(report);
    let dec = test_decrements2(report);

    //println!("{report:?} => {inc} || {dec}");

    return inc || dec;
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

pub fn process(input: &str) -> Result<usize> {
    let matrix = super::parse(input)?;

    let safe = matrix.into_iter().filter(is_safe_brute).count();

    Ok(safe)
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
        assert_eq!(4, result.unwrap());
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input).unwrap();
        assert_eq!(373, result);
    }
}
