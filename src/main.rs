mod pentomino;
mod transpose;
mod types;

use crate::pentomino::Pentomino;
use crate::transpose::{pretty_print_piece, rotate_positive};
use crate::types::PentoType::F;
use crate::types::*;

fn main() {
    println!("F {:?}", PENTOMINO_F);
    pretty_print_piece(&PENTOMINO_F);

    println!("I {:?}", PENTOMINO_I);
    pretty_print_piece(&PENTOMINO_I);

    println!("L {:?}", PENTOMINO_L);
    pretty_print_piece(&PENTOMINO_L);

    println!("N {:?}", PENTOMINO_N);
    pretty_print_piece(&PENTOMINO_N);

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


    let my = Pentomino::<3, 3>::new(F);

    for pento in PentoType::iterator4x4() {
        println!("{:?}", pento);
    }
}
