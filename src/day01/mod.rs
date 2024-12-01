pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> miette::Result<(Vec<u32>, Vec<u32>)> {
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

    Ok((list1, list2))
}

