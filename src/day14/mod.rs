pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    pub fn position(&self, grid_width: i32, grid_height: i32, time: i32) -> (i32, i32) {
        let x = (self.px + time * self.vx).rem_euclid(grid_width);
        let y = (self.py + time * self.vy).rem_euclid(grid_height);
        (x, y)
    }
}

pub fn parse(input: &str) -> Vec<Robot> {
    use chumsky::prelude::*;

    let i32 = (just::<_, _, extra::Err<Simple<char>>>("-").or_not())
        .then(text::int(10).map(|s: &str| s.parse::<i32>().unwrap()))
        .map(|(minus, int)| if minus.is_some() { -int } else { int });

    let robot_def = just("p=")
        .ignore_then(i32)
        .then_ignore(just(","))
        .then(i32)
        .then_ignore(just(" v="))
        .then(i32)
        .then_ignore(just(","))
        .then(i32)
        .map(|(((px, py), vx), vy)| Robot { px, py, vx, vy });

    let parser = robot_def.separated_by(text::newline()).collect();

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
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
        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(12, result.len());
    }
}
