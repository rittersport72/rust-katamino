use crate::types::{PentoType, GREY_COLOR};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Pentomino<const C: usize, const R: usize> {
    forms: HashSet<[[PentoType; C]; R]>,
    pento: PentoType,
    color: [f32; 4],
}

impl<const C: usize, const R: usize> Pentomino<C, R> {
    pub fn new(pento: PentoType) -> Self {
        Pentomino {
            forms: HashSet::new(),
            pento,
            color: GREY_COLOR,
        }
        // create all pentominos
    }

    fn create_forms(pento: PentoType) -> HashSet<[[PentoType; C]; R]> {
        // TODO
        HashSet::new()
    }
    fn create_color(pento: PentoType) -> [f32; 4] {
        // TODO
        GREY_COLOR
    }
}
