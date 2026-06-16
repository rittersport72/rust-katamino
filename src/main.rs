mod types;
mod transpose;

use crate::types::{PentoType, *};

fn main() {
    println!("L {:?}", PENTOMINO_L);
    pretty_print_piece(&PENTOMINO_L);

    println!("L {:?}", PENTOMINO_N);
    pretty_print_piece(&PENTOMINO_N);
}

pub fn pretty_print_piece<const R: usize, const C: usize>(piece: &[[PentoType; C]; R]) {
    // for row in piece {
    //     let s = row
    //         .iter()
    //         .map(|c| format!("{}", c))
    //         .collect::<Vec<_>>()
    //         .join(" ");
    //     println!("{}", s);
    // } // <- GPT-5
    for r in 0..R {
        for c in 0..C {
            if c + 1 < C {
                print!("{} ", piece[r][c]);
            } else {
                print!("{}", piece[r][c]);
            }
        }
        println!();
    }
}
