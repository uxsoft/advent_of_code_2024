use itertools::Itertools;

pub struct Matrix {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

type Coordinate = (usize, usize);
type Line = Vec<Coordinate>;

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = rows.len();
        let width = rows[0].len();

        Self {
            rows,
            width,
            height,
        }
    }

    pub fn rows(&self) -> Vec<Line> {
        (0..self.height)
            .map(|y| (0..self.width).map(|x| (x, y)).collect())
            .collect()
    }

    pub fn columns(&self) -> Vec<Line> {
        (0..self.width)
            .map(|x| (0..self.height).map(|y| (x, y)).collect())
            .collect()
    }

    // pub fn diagonals(&self) -> Vec<Line> {

    // }

    pub fn at(&self, (x, y): Coordinate) -> char {
        self.rows[y][x]
    }

    pub fn matches(&self, line: Line, pattern: &str) -> usize {
        line.windows(pattern.len())
            .filter(|subline| {
                subline
                    .iter()
                    .zip(pattern.chars())
                    .all(|(coord, ch)| ch == self.at(*coord))
            })
            .count()
    }
}

pub fn process(input: &str) -> miette::Result<usize> {
    let matrix = Matrix::new(input);

    let row_count: usize = matrix
        .rows()
        .into_iter()
        .map(|line| matrix.matches(line, "XMAS"))
        .sum();

    let row_rev_count: usize = matrix
        .rows()
        .into_iter()
        .rev()
        .map(|line| matrix.matches(line, "XMAS"))
        .sum();

    let column_count: usize = matrix
        .columns()
        .into_iter()
        .map(|line| matrix.matches(line, "XMAS"))
        .sum();

    let column_rev_count: usize = matrix
        .columns()
        .into_iter()
        .rev()
        .map(|line| matrix.matches(line, "XMAS"))
        .sum();

    let total = row_count + row_rev_count + column_count + column_rev_count;

    println!("{}", input);
    println!("Total {total} = {row_count} + {row_rev_count} + {column_count} + {column_rev_count}");

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = process(input).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input).unwrap();
        assert_eq!(result, 0);
    }
}
