use super::shared::coordinate::Coordinate;
use chumsky::prelude::*;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<Coordinate> {
    let usize =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<usize>().unwrap());

    let coordinate = usize
        .then_ignore(just(","))
        .then(usize)
        .map(|(x, y)| Coordinate::new(x, y));

    let parser = coordinate.separated_by(text::newline()).collect();

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        let result = parse(input);
        assert_eq!(result.len(), 25);
    }
}
