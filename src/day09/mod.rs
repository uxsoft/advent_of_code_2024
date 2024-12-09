use itertools::Itertools;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> Vec<(char, char)> {
    format!("{input}0").chars().tuples().collect()
}

pub fn to_array(input: &str) -> Vec<i16> {
    let tuples = parse(input);
    let mut array = Vec::with_capacity(5 * input.len());

    let mut file_id = 0 as i16;
    for (file_char, free_space_char) in tuples {
        let file_value = file_char.to_digit(10).unwrap();
        let free_space_value = free_space_char.to_digit(10).unwrap();

        for _ in 0..file_value {
            array.push(file_id);
        }
        for _ in 0..free_space_value {
            array.push(-1);
        }

        file_id += 1;
    }

    array
}

pub fn checksum(array: &Vec<i16>) -> usize {
    array
        .into_iter()
        .enumerate()
        .map(|(index, file_id)| match *file_id {
            -1 => 0,
            f => index * f as usize,
        })
        .sum()
}

pub fn print(array: &Vec<i16>) {
    println!(
        "{:?}",
        array
            .iter()
            .map(|i| match i {
                -1 => '.'.into(),
                i => i.to_string(),
            })
            .join("")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "2333133121414131402";

        let result = to_array(input);

        println!("{:?}", result.iter().join("|"));

        assert_eq!(42, result.len());
    }
}
