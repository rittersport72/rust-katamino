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
pub const PENTOMINO_P: [[PentoType; 3]; 3] = [[E, P, P], [E, P, P], [E, P, E]];
pub const PENTOMINO_T: [[PentoType; 3]; 3] = [[T, T, T], [E, T, E], [E, T, E]];
pub const PENTOMINO_U: [[PentoType; 3]; 3] = [[U, E, U], [U, U, U], [E, E, E]];
pub const PENTOMINO_V: [[PentoType; 3]; 3] = [[V, E, E], [V, E, E], [V, V, V]];
pub const PENTOMINO_W: [[PentoType; 3]; 3] = [[W, E, E], [W, W, E], [E, W, W]];
pub const PENTOMINO_X: [[PentoType; 3]; 3] = [[E, X, E], [X, X, X], [E, X, E]];
pub const PENTOMINO_Y: [[PentoType; 4]; 4] = [[E, E, Y, E], [E, Y, Y, E], [E, E, Y, E], [E, E, Y, E]];
pub const PENTOMINO_Z: [[PentoType; 3]; 3] = [[Z, Z, E], [E, Z, E], [E, Z, Z]];

pub const GREY_COLOR: [f32; 4] = [0.84, 0.84, 0.70, 1.0];
pub const DARK_GREY_COLOR: [f32; 4] = [0.54, 0.54, 0.44, 1.0];
pub const BROWN_COLOR: [f32; 4] = [0.55, 0.29, 0.29, 1.0];
pub const DARK_BROWN_COLOR: [f32; 4] = [0.34, 0.14, 0.14, 1.0];
pub const RED_BROWN_COLOR: [f32; 4] = [0.52, 0.24, 0.07, 1.0];
pub const OCHRE_COLOR: [f32; 4] = [0.77, 0.61, 0.03, 1.0];