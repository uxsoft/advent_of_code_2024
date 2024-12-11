pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(10) as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(8, result.len());
        assert_eq!(8, result[0].len());
    }
}
