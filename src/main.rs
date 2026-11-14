mod pentomino;
mod transpose;
mod types;

use crate::pentomino::Pentomino;
use crate::transpose::{pretty_print_piece, rotate_positive};
use crate::types::PentoType::*;
use crate::types::*;

fn main() {
    println!("F {:?}", PENTOMINO_F);
    pretty_print_piece(&PENTOMINO_F);

    println!("T {:?}", PENTOMINO_T);
    let mut piece = PENTOMINO_T;
    println!("rotate");
    piece = rotate_positive(&piece);
    pretty_print_piece(&piece);
    println!("rotate");
    piece = rotate_positive(&piece);
    pretty_print_piece(&piece);
    println!("rotate");
    piece = rotate_positive(&piece);
    pretty_print_piece(&piece);


    let abc: Pentomino<3, 3> = Pentomino::new(PentoType::T);

    println!("I {:?}", PENTOMINO_I);
    pretty_print_piece(&PENTOMINO_I);
}
