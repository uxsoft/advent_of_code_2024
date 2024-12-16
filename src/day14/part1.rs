use itertools::Itertools;

fn print(grid_width: i32, grid_height: i32, positions: &Vec<(i32, i32)>) {
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

pub fn process(input: &str, width: i32, height: i32) -> usize {
    let robots = super::parse(input);
    let time = 100;
    let positions: Vec<(i32, i32)> = robots
        .iter()
        .map(|robot| robot.position(width, height, time))
        .collect();

    let q1 = positions
        .iter()
        .filter(|(x, y)| *x < (width / 2) && *y < (height / 2))
        .count();

    let q2 = positions
        .iter()
        .filter(|(x, y)| *x > (width / 2) && *y < (height / 2))
        .count();

    let q3 = positions
        .iter()
        .filter(|(x, y)| *x < (width / 2) && *y > (height / 2))
        .count();

    let q4 = positions
        .iter()
        .filter(|(x, y)| *x > (width / 2) && *y > (height / 2))
        .count();

    q1 * q2 * q3 * q4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        let result = process(input, 11, 7);

        assert_eq!(12, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input, 101, 103);

        assert_eq!(211692000, result);
    }
}
