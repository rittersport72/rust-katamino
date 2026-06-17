mod transpose;
mod types;

use crate::transpose::{pretty_print_piece, rotate_positive};
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
}
