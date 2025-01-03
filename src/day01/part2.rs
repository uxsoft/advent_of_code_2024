use std::collections::BTreeMap;

pub fn process(input: &str) -> u32 {
    let lists = super::parse_chumsky(input);
    let (list1, list2): (Vec<_>, Vec<_>) = lists.into_iter().unzip();

    let mut frequency: BTreeMap<u32, u32> = BTreeMap::new();
    for word in list2 {
        *frequency.entry(word).or_insert(0) += 1;
    }

    let result = list1
        .into_iter()
        .map(|i| i * frequency.get(&i).unwrap_or(&0))
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";

        let result = process(input);
        assert_eq!(31, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);
        assert_eq!(21328497, result);
    }
}
