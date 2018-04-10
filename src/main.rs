#![feature(duration_extras)]

extern crate strsim;
extern crate unidecode;

mod dev;
mod edit_distance;
mod string_strategy;
mod suggest_item;
mod symspell;

use std::{thread, time};

use string_strategy::{AsciiStringStrategy};
use dev::measure;
use symspell::{SymSpell, Verbosity};

fn main() {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::new(
        2, // max dictionary edit distance
        7, // prefix length
        1, // count threshold
    );
    // symspell.load_dictionary("corpus.txt", 0, 1);

    measure("load_dictionary", || {
        symspell.load_dictionary(
            "new_fdb.txt",
            // "prim-7.0-public-vyv-word-frequency.txt",
            // "corpus.txt",
            0,
            1,
            "\t",
            1_000_000,
        );
    });

    measure("lookup", || {
        let result = symspell.lookup("aleko", Verbosity::All, 2);
        println!("{:?}", result);
    });

    thread::sleep(time::Duration::from_secs(10000000));
}
