use glam::{ivec2, IVec2};
use itertools::Itertools;
use std::{collections::HashMap, ops::Neg};
use KeyPad::*;

pub enum KeyPad {
    NumPad { keys: HashMap<char, IVec2> },
    ArrowPad { keys: HashMap<char, IVec2> },
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
        }
    }

    pub fn keys(&self) -> &HashMap<char, IVec2> {
        match self {
            NumPad { keys } => keys,
            ArrowPad { keys } => keys,
        }
    }

    pub fn shortest_paths(&self) -> HashMap<(char, char), Vec<String>> {
        self.keys()
            .keys()
            .cartesian_product(self.keys().keys())
            .map(|(a, b)| {
                let ap = self.locate(*a);
                let bp = self.locate(*b);

                let delta_x = bp.x - ap.x;
                let delta_y = bp.y - ap.y;

                let vertical = if delta_y.is_negative() {
                    "^".repeat(delta_y.neg() as usize)
                } else {
                    "v".repeat(delta_y as usize)
                };

                let horizontal = if delta_x.is_negative() {
                    "<".repeat(delta_x.neg() as usize)
                } else {
                    ">".repeat(delta_x as usize)
                };

                let paths = match self {
                    NumPad { keys: _ } if ap.x == 0 && bp.y == 3 => {
                        vec![format!("{}{}A", horizontal, vertical)]
                    }
                    NumPad { keys: _ } if bp.x == 0 && ap.y == 3 => {
                        vec![format!("{}{}A", vertical, horizontal)]
                    }
                    ArrowPad { keys: _ } if ap.x == 0 && bp.y == 0 => {
                        vec![format!("{}{}A", horizontal, vertical)]
                    }
                    ArrowPad { keys: _ } if bp.x == 0 && ap.y == 0 => {
                        vec![format!("{}{}A", vertical, horizontal)]
                    }
                    _ if delta_x == 0 || delta_y == 0 => {
                        vec![format!("{}{}A", vertical, horizontal)]
                    }
                    _ => {
                        vec![
                            format!("{}{}A", vertical, horizontal),
                            format!("{}{}A", horizontal, vertical),
                        ]
                    }
                };

                ((*a, *b), paths)
            })
            .collect()
    }
}
