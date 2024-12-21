pub mod part1;
pub mod part2;

use chumsky::prelude::*;

pub fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let name = text::ident::<_, _, extra::Err<Simple<char>>>();

    let parts = name.separated_by(just(", ")).collect();

    let patterns = name.separated_by(text::newline()).collect();

    let parser = parts
        .then_ignore(text::newline())
        .then_ignore(text::newline())
        .then(patterns);

    let result = parser.parse(input).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (parts, patterns) = parse(input);
        assert_eq!(parts.len(), 8);
        assert_eq!(patterns.len(), 8);
    }
}
