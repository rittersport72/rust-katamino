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


    let mut v3x3 = Vec::new();

    for pento in PentoType::iterator3x3() {
        v3x3.push(Pentomino::<3, 3>::new(*pento));
        println!("{:?}", pento);
    }

    let mut v4x4 = Vec::new();

    for pento in PentoType::iterator4x4() {
        v4x4.push(Pentomino::<4, 4>::new(*pento));
        println!("{:?}", pento);
    }

    println!("I {:?}", PENTOMINO_I);
    pretty_print_piece(&PENTOMINO_I);

}
