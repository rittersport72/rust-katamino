use crate::types::*;
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
            forms: Self::create_forms(pento),
            pento,
            color: Self::create_color(pento),
        }
    }

    fn create_forms(pento: PentoType) -> HashSet<[[PentoType; C]; R]> {
        //let mut hs = HashSet::new();
        //hs.insert(PENTOMINO_P);
        //hs
        HashSet::new()
    }
    fn create_color(pento: PentoType) -> [f32; 4] {
        match pento {
            PentoType::N | PentoType::Y => DARK_GREY_COLOR,
            PentoType::V | PentoType::W => BROWN_COLOR,
            PentoType::F | PentoType::Z => DARK_BROWN_COLOR,
            PentoType::L | PentoType::P => RED_BROWN_COLOR,
            PentoType::T | PentoType::U => OCHRE_COLOR,
            _ => GREY_COLOR,
        }
    }
}
