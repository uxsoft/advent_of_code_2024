pub mod part1;
pub mod part2;

use miette::*;

pub fn parse(input: &str) -> miette::Result<Vec<Vec<u32>>> {
    let matrix = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    Ok(matrix)
}

pub fn parse_chumsky(input: &str) -> miette::Result<Vec<Vec<u32>>> {
    use chumsky::prelude::*;

    let u32 =
        text::int::<_, _, extra::Err<Simple<char>>>(10).map(|s: &str| s.parse::<u32>().unwrap());

    let line = u32
        .separated_by(just(" "))
        .collect();

    let parser = line
        .separated_by(text::newline()).collect();

    let result = parser
        .parse(input)
        .into_result()
        .map_err(|e| miette!("Failed to parse the input, {:?}", e));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        
        let result = parse_chumsky(input);
        
        println!("{:?}", result);

        assert!(result.is_ok());
    }
}
