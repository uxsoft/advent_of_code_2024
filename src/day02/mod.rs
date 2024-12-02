pub mod part1;
pub mod part2;

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
