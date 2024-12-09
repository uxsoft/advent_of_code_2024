pub fn process(input: &str) -> usize {
    let mut array = super::to_array(input);

    let mut left_i = 0;
    let mut right_i = array.len() - 1;

    while left_i < right_i {
        let left_char = array[left_i];
        let right_char = array[right_i];

        if left_char == -1 && right_char >= 0 {
            // do a swap
            array[left_i] = array[right_i];
            array[right_i] = -1;
            left_i += 1;
            right_i -= 1;
        } else if left_char >= 0 {
            left_i += 1;
            // advance the left pointer
        } else if right_char == -1 {
            // advance the right pointer (leftwards)
            right_i -= 1;
        }
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

        assert_eq!(1928, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(6382875730645, result);
    }
}
