pub mod part1;
pub mod part2;

use chumsky::prelude::*;

pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let lines = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for line in lines {
        list1.push(line.first().unwrap().parse().unwrap());
        list2.push(line.last().unwrap().parse().unwrap());
    }

    (list1, list2)
}

pub fn parse_chumsky(input: &str) -> Vec<(u32, u32)> {
    let u32 =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<u32>().unwrap());

    let line = u32.then_ignore(text::whitespace()).then(u32);

    let parser = line.separated_by(text::newline()).collect();

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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = parse_chumsky(input);

        println!("{:?}", result);

        assert_eq!(result.len(), 6);
    }
}
