pub mod part1;
pub mod part2;

type Key = Vec<u8>;
type Lock = Vec<u8>;

pub fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for pattern in input.split("\n\n") {
        let mut first_line_count = 0;
        let mut last_line_count = 0;

        let mut device = vec![0; 5];

        for (y, line) in pattern.lines().enumerate() {
            for (x, val) in line.char_indices() {
                match val {
                    '#' => {
                        *device.get_mut(x).unwrap() += 1;
                        if y == 0 {
                            first_line_count += 1;
                        } else if y == 5 {
                            last_line_count += 1;
                        }
                    }
                    _ => (),
                }
            }
        }

        if first_line_count > last_line_count {
            locks.push(device);
        } else {
            keys.push(device);
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let (locks, keys) = parse(input);
        assert_eq!(locks.len(), 2);
        assert_eq!(keys.len(), 3);
    }
}
