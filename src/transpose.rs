use crate::types::PentoType;
use crate::types::PentoType::E;

// Rotate the passed matrix counterclockwise
pub fn rotate_positive<const C: usize, const R: usize>(piece: &[[PentoType; C]; R]) -> [[PentoType; R]; C] {
    let mut out = [[E; R]; C];

    for r in 0..R {
        for c in 0..C {
            out[C - 1 - c][r] = piece[r][c];
        }
    }

    out
}

// Reflect the passed matrix horizontally
pub fn reflect_horizontal<const R: usize, const C: usize>(
    piece: &[[PentoType; C]; R],
) -> [[PentoType; C]; R] {
    let mut out = *piece;
    for r in 0..R {
        for c in 0..(C / 2) {
            out[r].swap(c, C - 1 - c);
        }
    }
    out
}

// Reflect the passed matrix vertically
pub fn reflect_vertical<const R: usize, const C: usize>(
    piece: &[[PentoType; C]; R],
) -> [[PentoType; C]; R] {
    let mut out = *piece;
    for r in 0..(R / 2) {
        out.swap(r, R - 1 - r);
    }
    out
}
