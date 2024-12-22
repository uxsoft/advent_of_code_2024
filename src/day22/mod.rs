pub mod part1;
pub mod part2;

pub struct PseudoRandomIterator {
    secret: usize,
}

impl PseudoRandomIterator {
    pub fn new(i: usize) -> Self {
        Self { secret: i }
    }
}

impl Iterator for PseudoRandomIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.secret;

        x = (x * 64) ^ x;
        x %= 16777216;

        x = (x >> 5) ^ x;
        x %= 16777216;

        x = (x * 2048) ^ x;
        x %= 16777216;

        let prev_secret = self.secret;
        self.secret = x;
        
        Some(prev_secret)
    }
}

pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
