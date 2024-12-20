use std::{
    collections::{HashMap, HashSet},
    ops::ShlAssign,
};

use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Program;

pub fn process_brute_force(input: &str) -> usize {
    let (instructions, _, _, _) = Program::parse(input);

    let pb = ProgressBar::new(236539226447469);

    let result = (0..usize::MAX).into_par_iter().find_any(|i| {
        let mut prg = Program::new(*i, 0, 0, &instructions);
        prg.run_to_end();
        pb.inc(1);
        prg.output == instructions
    });

    return result.unwrap();
}

fn compute(a: usize, instructions: &Vec<u8>) -> u8 {
    let mut prg = Program::new(a, 0, 0, &instructions);
    prg.run_to_end();
    prg.output[0]
}

pub fn process(input: &str) -> usize {
    let (instructions, _, _, _) = Program::parse(input);

    let mut candidates = vec![0];

    for target_instruction in instructions.iter().rev() {
        
        let mut new_candidates = Vec::new();
        for candidate_a in candidates {
            for i in 0..8 {
                let new_a = (candidate_a << 3) + i;

                if compute(new_a, &instructions) == *target_instruction {
                    new_candidates.push(new_a);
                }
            }
        }
        candidates = new_candidates;
    }

    *candidates.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_direct() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        let (instructions, _, _, _) = Program::parse(input);
        let mut prg = Program::new(117440, 0, 0, &instructions);
        prg.run_to_end();
        assert_eq!(prg.output, instructions);
    }

    #[test]
    fn example() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let result = process(input);
        assert_eq!(result, 117440);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, 236539226447469);
    }
}
