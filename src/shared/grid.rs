use std::fmt::Display;

use itertools::Itertools;

use super::coordinate::Coordinate;

pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn parse<F>(input: &str, mut parse_cell: F) -> Grid<T>
    where
        T: Sized + Clone + Copy,
        F: FnMut(char) -> T,
    {
        let rows = input
            .lines()
            .map(|line| line.chars().map(&mut parse_cell).collect())
            .collect();

        Grid { rows }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn find<F>(&self, mut predicate: F) -> Option<Coordinate>
    where
        F: FnMut(&T) -> bool,
    {
        self.rows.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .find_position(|c| predicate(c))
                .map(|(x, _)| Coordinate::new(x, y))
        })
    }

    pub fn get(&self, at: &Coordinate) -> &T {
        &self.rows[at.y][at.x]
    }

    pub fn set(&mut self, at: &Coordinate, value: T) {
        self.rows[at.y][at.x] = value;
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for i in row {
                write!(f, "{i}")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
