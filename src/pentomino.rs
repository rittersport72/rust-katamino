use crate::pentomino::Piece::{Piece3x3, Piece4x4, Piece5x1};
use crate::types::*;
use crate::transpose::rotate_positive;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
    Piece3x3([[PentoType; 3]; 3]),
    Piece4x4([[PentoType; 4]; 4]),
    Piece5x1([[PentoType; 5]; 1]),
}

#[derive(Debug, Clone)]
pub struct Pentomino<const C: usize, const R: usize> {
    forms: Vec<Piece>,
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

    fn create_forms(pento: PentoType) -> Vec<Piece> {
        let mut hs = Vec::new();

        // Start from the base shape for `pento`
        // match pento {
        //     PentoType::P => hs.insert(Piece3x3(PENTOMINO_P)),
        //     PentoType::T => hs.insert(Piece3x3(PENTOMINO_T)),
        //     _ => hs.insert(Piece4x4(PENTOMINO_P)),
        // }

        hs.push(Piece3x3(rotate_positive(&PENTOMINO_T)));
        hs.push(Piece3x3(rotate_positive(&PENTOMINO_T)));
        hs.push(Piece3x3(rotate_positive(&PENTOMINO_T)));
        hs.push(Piece3x3(rotate_positive(&PENTOMINO_T)));

        hs
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
