/*!

Spelling correction & Fuzzy search based on Symmetric Delete spelling correction algorithm.

# Basic Example

```
let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");

let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
println!("{:?}", suggestions);

let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him"
let compound_suggestions = symspell.lookup_compound(sentence, 2);
println!("{:?}", compound_suggestions);
```
*/

extern crate strsim;
#[macro_use]
extern crate derive_builder;
extern crate unidecode;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
extern crate wasm_bindgen_test;

mod edit_distance;
mod string_strategy;
mod suggestion;
mod symspell;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use string_strategy::{AsciiStringStrategy, StringStrategy, UnicodeiStringStrategy};
pub use suggestion::Suggestion;
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};

#[cfg(target_arch = "wasm32")]
pub use wasm::JSSymSpell;
