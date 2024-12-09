use std::{
    collections::HashMap,
    ops::{Div, Sub},
};

fn find(arr: &Vec<i16>, length: usize) -> Option<usize> {
    let mut start_i = 0;
    let mut l = 0;
    for i in 0..arr.len() {
        match arr[i] {
            -1 => l += 1,
            _ => {
                start_i = i + 1;
                l = 0
            }
        }
        if l >= length {
            return Some(start_i);
        }
    }

    None
}

fn swap(arr: &mut Vec<i16>, i1: usize, i2: usize, length: usize) {
    for i in 0..length {
        arr.swap(i1 + i, i2 + i);
    }
}

pub fn process(input: &str) -> usize {
    let pairs = super::parse(input);
    let mut array = super::to_array(input);

    let lengths: HashMap<i16, usize> = super::parse(input)
        .iter()
        .enumerate()
        .map(|(index, (b, _))| (index as i16, b.to_digit(10).unwrap() as usize))
        .collect();

    let (indexes, _): (HashMap<i16, usize>, _) = pairs.iter().enumerate().fold(
        (HashMap::new(), 0 as usize),
        |(mut map, index), (id, (b, fs))| {
            map.insert(id as i16, index);
            (
                map,
                index + b.to_digit(10).unwrap() as usize + fs.to_digit(10).unwrap() as usize,
            )
        },
    );

    let mut file_id = input.len().sub(1).div(2) as i16;

    while file_id > 0 {
        // find free space (fs_index) with length >= length[current_file_id]
        // replace fs_index..length[current_file_id] with current_file_id
        // replace index[current_file_id]..length[current_file_id]
        // OR even better:
        // swap

        let length = lengths[&file_id];
        if let Some(fs_index) = find(&array, length) {
            let b_index = indexes[&file_id];
            if fs_index < b_index {
                // Only move leftwards
                swap(&mut array, fs_index, b_index, length);
            }
        }
        file_id -= 1;
    }

    super::checksum(&array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";

        let result = process(input);

        assert_eq!(2858, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(6420913943576, result);
    }
}
