pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<usize> {
    use chumsky::prelude::*;

    let usize =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<usize>().unwrap());

    let parser = usize
        .separated_by(just(" "))
        .collect();

    let result = parser
        .parse(input)
        .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "0 1 10 99 999";

        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(5, result.len());
    }
}
