#![feature(duration_extras)]

extern crate strsim;
#[macro_use]
extern crate derive_builder;
extern crate unidecode;

mod edit_distance;
mod string_strategy;
mod suggest_item;
mod symspell;

pub use string_strategy::{AsciiStringStrategy, StringStrategy, UnicodeiStringStrategy};
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};
