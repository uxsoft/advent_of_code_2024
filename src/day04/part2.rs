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

    pub fn points(&self) -> Vec<Coordinate> {
        (1..(self.height - 1))
            .flat_map(|y| (1..(self.width - 1)).map(move |x| (x, y)))
            .collect()
    }

    pub fn at(&self, (x, y): Coordinate) -> char {
        self.rows[y][x]
    }

    pub fn is_mas_center(&self, (x, y): Coordinate) -> bool {
        let tl = self.at((x - 1, y - 1));
        let tr = self.at((x + 1, y - 1));
        let br = self.at((x + 1, y + 1));
        let bl = self.at((x - 1, y + 1));

        match (tl, tr, br, bl) {
            ('M', 'S', 'S', 'M') => true,
            ('M', 'M', 'S', 'S') => true,
            ('S', 'M', 'M', 'S') => true,
            ('S', 'S', 'M', 'M') => true,
            _ => false,
        }
    }
}

pub fn process(input: &str) -> miette::Result<usize> {
    let matrix = Matrix::new(input);

    let total = matrix
        .points()
        .into_iter()
        .filter(|p| matrix.at(*p) == 'A')
        .filter(|p| matrix.is_mas_center(*p))
        .count();

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "M.S
.A.
M.S";
        let result = process(input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
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
        assert_eq!(result, 9);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input).unwrap();
        assert_eq!(result, 1978);
    }
}
