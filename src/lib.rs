/*!

Spelling correction & Fuzzy search based on Symmetric Delete spelling correction algorithm

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

#![feature(duration_extras)]

extern crate strsim;
#[macro_use]
extern crate derive_builder;
extern crate unidecode;

mod edit_distance;
mod string_strategy;
mod suggestion;
mod symspell;

pub use string_strategy::{AsciiStringStrategy, StringStrategy, UnicodeiStringStrategy};
pub use suggestion::Suggestion;
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};
