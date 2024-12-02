use itertools::Itertools;
use miette::Result;

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
pub fn is_safe(report: &Vec<u32>) -> bool {
    let decreasing = report
        .into_iter()
        .tuple_windows()
        .all(|(a, b)| a > b && a.abs_diff(*b) <= 3);
    
    let increasing = report
        .into_iter()
        .tuple_windows()
        .all(|(a, b)| a < b && a.abs_diff(*b) <= 3);

    // println!(" {increasing} {decreasing}");
    return decreasing || increasing;
}

pub fn process(input: &str) -> Result<usize> {
    let matrix = super::parse(input)?;

    let safe = matrix.into_iter().filter(is_safe).count();

    Ok(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1_example() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = process(input);
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn day02_part1_prod() {
        let input = include_str!("input.txt");

        let result = process(input);
        assert_eq!(314, result.unwrap());
    }
}
