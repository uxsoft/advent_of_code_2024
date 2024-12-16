use std::collections::HashSet;

use itertools::Itertools;

fn print(grid_width: i32, grid_height: i32, positions: &HashSet<(i32, i32)>) {
    let str = (0..grid_height)
        .map(|y| {
            (0..grid_width)
                .map(|x| {
                    match positions
                        .iter()
                        .filter(|(px, py)| *px == x && *py == y)
                        .count()
                    {
                        0 => ".".to_string(),
                        d => d.to_string(),
                    }
                })
                .join("")
        })
        .join("\n");

    print!("{}", str);
}

pub fn process(input: &str, width: i32, height: i32) -> i32 {
    let robots = super::parse(input);

    let mut i = 0;
    for t in 0..i32::MAX {
        let positions: HashSet<(i32, i32)> = robots
            .iter()
            .map(|robot| robot.position(width, height, t))
            .collect();

        if positions.len() == robots.len() {
            i += 1;
        }

        if i == 3 {
            return t;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input, 101, 103);

        assert_eq!(6587, result);
    }
}
