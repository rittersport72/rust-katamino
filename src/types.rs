use std::fmt::Display;
use PentoType::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PentoType {
    E,
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Display for PentoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            E => write!(f, "."),
            F => write!(f, "F"),
            I => write!(f, "I"),
            L => write!(f, "L"),
            N => write!(f, "N"),
            P => write!(f, "P"),
            T => write!(f, "T"),
            U => write!(f, "U"),
            V => write!(f, "V"),
            W => write!(f, "W"),
            X => write!(f, "X"),
            Y => write!(f, "Y"),
            Z => write!(f, "Z"),
        }
    }
}

pub const PENTOMINO_F: [[PentoType; 3]; 3] = [[E, F, F], [F, F, E], [E, F, E]];
pub const PENTOMINO_I: [[PentoType; 5]; 1] = [[I, I, I, I, I]];
pub const PENTOMINO_L: [[PentoType; 4]; 4] = [[E, L, E, E], [E, L, E, E], [E, L, E, E], [E, L, L, E]];
pub const PENTOMINO_N: [[PentoType; 4]; 4] = [[E, E, N, E], [E, N, N, E], [E, N, E, E], [E, N, E, E]];
pub const PENTOMINO_P: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_T: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_U: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_V: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_W: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_X: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
pub const PENTOMINO_Y: [[PentoType; 4]; 4] = [[E, E, L, E], [E, E, L, E], [L, L, L, E], [L, E, L, L]];
pub const PENTOMINO_Z: [[PentoType; 3]; 3] = [[E, E, L], [E, E, L], [L, L, L]];
