[![Documentation](https://docs.rs/symspell/badge.svg)](https://docs.rs/symspell)

# SymSpell

Rust implementation of brilliant [SymSpell](https://github.com/wolfgarbe/SymSpell) originally written in C# by [@wolfgarbe](https://github.com/wolfgarbe).

## Usage

```rust
extern crate symspell;

use symspell::{AsciiStringStrategy, SymSpell, Verbosity};

fn main() {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");

    let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
    println!("{:?}", suggestions);

    let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him"
    let compound_suggestions = symspell.lookup_compound(sentence, 2);
    println!("{:?}", compound_suggestions);
}
```

## Advanced Usage

### Using Custom Settings

```rust
let mut symspell: SymSpell<AsciiStringStrategy> = SymSpellBuilder::default()
    .max_dictionary_edit_distance(2)
    .prefix_length(7)
    .count_threshold(1)
    .build()
    .unwrap()
```

### String Strategy

String strategy is abstraction for string manipulation, for example preprocessing.

There are two strategies included:
* `UnicodeiStringStrategy`
    * Doesn't do any prepocessing and handles strings as they are.
* `AsciiStringStrategy`
    * Transliterates strings into ASCII only characters.
    * Useful when you are working with accented languages and you don't want to care about accents, etc

To configure string strategy just pass it as a type parameter:

```rust
let mut ascii_symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
let mut unicode_symspell: SymSpell<UnicodeiStringStrategy> = SymSpell::default();
```

### Javascript Bindings

This crate can be compiled against wasm32 target and exposes a SymSpell Class that can be used from Javascript as follow.

```javascript
const rust = require('./pkg');
let spell_checker = new rust.SymSpell({ is_ascii: false, max_edit_distance: 2,  prefix_length: 7,  count_threshold: 1});
speller.load_dictionary(arraybuffer, { term_index: 0,  count_index: 1, separator: " "});
speller.lookup_compound(sentence, 1);
```

It can be compiled using `wasm-pack` (eg. `wasm-pack build --release --target nodejs`)
