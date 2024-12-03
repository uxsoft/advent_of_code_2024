pub fn process(input: &str) -> miette::Result<u32> {
    let lists = super::parse_chumsky(input)?;
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = lists.into_iter().unzip();

    list1.sort();
    list2.sort();

    let result = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_example() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";

        let result = process(input);
        assert_eq!(11, result.unwrap());
    }

    #[test]
    fn day01_part1_prod() {
        let input = include_str!("input.txt");

        let result = process(input);
        assert_eq!(2742123, result.unwrap());
    }
}
