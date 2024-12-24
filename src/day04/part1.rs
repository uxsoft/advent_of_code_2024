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
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .collect()
    }

    pub fn rays(&self, (x, y): Coordinate, length: usize) -> Vec<Line> {
        let mut rays = Vec::new();

        let can_right = x + length <= self.width;
        let can_left = x >= length - 1;
        let can_down = y + length <= self.height;
        let can_up = y >= length - 1;

        if can_right {
            rays.push((0..length).map(|d| (x + d, y)).collect());
        }
        if can_left {
            rays.push((0..length).map(|d| (x - d, y)).collect());
        }
        if can_down {
            rays.push((0..length).map(|d| (x, y + d)).collect());
        }
        if can_up {
            rays.push((0..length).map(|d| (x, y - d)).collect());
        }
        if can_right && can_up {
            rays.push((0..length).map(|d| (x + d, y - d)).collect());
        }
        if can_right && can_down {
            rays.push((0..length).map(|d| (x + d, y + d)).collect());
        }
        if can_left && can_down {
            rays.push((0..length).map(|d| (x - d, y + d)).collect());
        }
        if can_left && can_up {
            rays.push((0..length).map(|d| (x - d, y - d)).collect());
        }

        rays
    }

    pub fn at(&self, (x, y): Coordinate) -> char {
        self.rows[y][x]
    }

    pub fn is_match(&self, line: &Line, pattern: &str) -> bool {
        let result = line
            .iter()
            .zip(pattern.chars())
            .all(|(point, val)| self.at(*point) == val);

        // if result {
        //     println!("MATCHING LINE: {:?}", line);
        // }
        result
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

pub fn process(input: &str) -> usize {
    let matrix = Matrix::new(input);

    let total = matrix
        .points()
        .into_iter()
        .filter(|p| matrix.at(*p) == 'X')
        .flat_map(|p| matrix.rays(p, 4))
        .filter(|line| matrix.is_match(line, "XMAS"))
        .count();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn rays() {
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

        let matrix = Matrix::new(input);

        let test = matrix.rays((3, 9), 4);
        assert_eq!(5, test.len())
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
        let result = process(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, 2583);
    }
}
