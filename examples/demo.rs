#![feature(duration_extras)]

extern crate symspell;

use symspell::{AsciiStringStrategy, SymSpell, Verbosity};

fn main() {
    main_en();
    // main_sk();
}

#[allow(dead_code)]
fn main_en() {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    measure("load_dictionary", || {
        symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    });

    measure("lookup", || {
        let result = symspell.lookup("roket", Verbosity::Top, 2);
        println!("{:?}", result);
    });

    measure("lookup_compound", || {
        let result = symspell.lookup_compound("whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him", 2);
        println!("{:?}", result);
    });

    measure("lookup_compound", || {
        let result = symspell.lookup_compound("the bigjest playrs in te strogsommer film slatew ith plety of funn", 2);
        println!("{:?}", result);
    });
}

#[allow(dead_code)]
fn main_sk() {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    measure("load_dictionary", || {
        symspell.load_dictionary(
            "new_fdb.txt",
            0,
            1,
            "\t",
        );
    });

    measure("lookup", || {
        let result = symspell.lookup("aleko", Verbosity::All, 2);
        println!("{:?}", result);
    });

    measure("lookup_compound", || {
        let result = symspell.lookup_compound("pekníchlapi", 2);
        println!("{:?}", result);
    });
}

use std::time::Instant;

pub fn measure<F>(name: &str, mut f: F)
where
    F: FnMut() -> (),
{
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    let elapsed_ms = (elapsed.as_secs() * 1000000 + elapsed.subsec_micros() as u64) as f64 / 1000.0;

    if elapsed_ms < 1000.0 {
        println!("{} took {} ms", name, elapsed_ms);
    } else {
        println!("{} took {} s", name, elapsed_ms as u64 as f64 / 1000.0);
    }
}
