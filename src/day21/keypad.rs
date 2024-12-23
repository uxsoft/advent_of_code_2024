use glam::{ivec2, IVec2};
use itertools::Itertools;
use std::{collections::HashMap, ops::Neg};
use KeyPad::*;

pub enum KeyPad {
    NumPad { keys: HashMap<char, IVec2> },
    ArrowPad { keys: HashMap<char, IVec2> },
    Chain(Box<KeyPad>, Box<KeyPad>),
}

impl KeyPad {
    pub fn numpad() -> KeyPad {
        let keys = HashMap::from([
            ('7', ivec2(0, 0)),
            ('8', ivec2(1, 0)),
            ('9', ivec2(2, 0)),
            ('4', ivec2(0, 1)),
            ('5', ivec2(1, 1)),
            ('6', ivec2(2, 1)),
            ('1', ivec2(0, 2)),
            ('2', ivec2(1, 2)),
            ('3', ivec2(2, 2)),
            ('0', ivec2(1, 3)),
            ('A', ivec2(2, 3)),
        ]);
        NumPad { keys }
    }

    pub fn arrows() -> KeyPad {
        let keys = HashMap::from([
            ('^', ivec2(1, 0)),
            ('A', ivec2(2, 0)),
            ('<', ivec2(0, 1)),
            ('v', ivec2(1, 1)),
            ('>', ivec2(2, 1)),
        ]);

        ArrowPad { keys }
    }

    pub fn locate(&self, key: char) -> IVec2 {
        match self {
            NumPad { keys } => keys[&key],
            ArrowPad { keys } => keys[&key],
            Chain(from, _) => from.locate(key),
        }
    }

    fn translate_numpad_to_arrows(keys: &HashMap<char, IVec2>, sequence: Vec<char>) -> Vec<char> {
        let mut sequence = sequence;
        sequence.insert(0, 'A');
        sequence.push('A');

        sequence
            .iter()
            .tuple_windows()
            .flat_map(|(a, b, c)| {
                let a_pos = keys[a];
                let b_pos = keys[b];

                let delta_x = b_pos.x - a_pos.x;
                let delta_y = b_pos.y - a_pos.y;

                let mut result = String::new();

                if delta_y.is_negative() {
                    result.push_str(&"^".repeat(delta_y.neg() as usize));
                } else if delta_y.is_positive() {
                    result.push_str(&"v".repeat(delta_y as usize));
                }

                if delta_x.is_negative() {
                    result.push_str(&"<".repeat(delta_x.neg() as usize));
                } else if delta_x.is_positive() {
                    result.push_str(&">".repeat(delta_x as usize));
                }

                result.push_str("A");
                result.chars().collect::<Vec<_>>()
            })
            .collect()
    }

    fn translate_arrows_to_arrows(keys: &HashMap<char, IVec2>, sequence: Vec<char>) -> Vec<char> {
        let mut sequence = sequence;
        sequence.insert(0, 'A');
        sequence
            .iter()
            .tuple_windows()
            .flat_map(|(a, b)| {
                let a_pos = keys[a];
                let b_pos = keys[b];

                let delta_x = b_pos.x - a_pos.x;
                let delta_y = b_pos.y - a_pos.y;

                let mut result = String::new();

                if delta_x.is_negative() {
                    result.push_str(&"<".repeat(delta_x.neg() as usize));
                } 
                if delta_y.is_negative() {
                    result.push_str(&"^".repeat(delta_y.neg() as usize));
                }
                if delta_y.is_positive() {
                    result.push_str(&"v".repeat(delta_y as usize));
                }
                if delta_x.is_positive() {
                    result.push_str(&">".repeat(delta_x as usize));
                }

                result.push_str("A");
                result.chars().collect::<Vec<_>>()
            })
            .collect()
    }

    fn translate_chain(from: &Box<KeyPad>, to: &Box<KeyPad>, sequence: Vec<char>) -> Vec<char> {
        let intermediate = from.translate(sequence);
        let output = to.translate(intermediate);
        output
    }

    pub fn translate(&self, sequence: Vec<char>) -> Vec<char> {
        match self {
            NumPad { keys } => KeyPad::translate_numpad_to_arrows(keys, sequence),
            ArrowPad { keys } => KeyPad::translate_arrows_to_arrows(keys, sequence),
            Chain(from, to) => KeyPad::translate_chain(from, to, sequence),
        }
    }

    pub fn to(self, other: KeyPad) -> KeyPad {
        Chain(Box::new(self), Box::new(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn numpad_to_arrows() {
        let input = "029A".chars().collect_vec();

        let output = KeyPad::numpad().translate( input);
        let output_str = output.into_iter().join("");

        assert_eq!(output_str, "<A^A^^>AvvvA");
    }

    //#[test]
    fn numpad_to_arrows_to_arrows() {
        let input = "029A".chars().collect_vec();

        let n = KeyPad::numpad().translate(input);
        let an = KeyPad::arrows().translate(n);

        let output_str = an.into_iter().join("");
        let correct= "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        assert_eq!(output_str.len(), correct.len());
        assert_eq!(output_str, correct);
    }

    //#[test]
    fn k2() {
        let chain = KeyPad::numpad()
            .to(KeyPad::arrows())
            .to(KeyPad::arrows())
            .to(KeyPad::arrows());


        //chain.translate("")
    }
}
