[![Documentation](https://docs.rs/symspell/badge.svg)](https://docs.rs/symspell)
[![Crates.io](https://img.shields.io/crates/v/symspell.svg)](https://crates.io/crates/symspell)

# SymSpell

A fast spelling correction and fuzzy search library for Rust. Given a misspelled word like `"roket"`, it finds the correct spelling `"rocket"`. It can also fix entire sentences, split concatenated words, and rank suggestions by how common each word is.

Rust implementation of the [SymSpell](https://github.com/wolfgarbe/SymSpell) algorithm originally written in C# by [@wolfgarbe](https://github.com/wolfgarbe). It works by pre-computing all possible deletions of dictionary words and storing them in a hash map, which allows O(1) average lookup time at query time — orders of magnitude faster than traditional approaches like BK-trees or Norvig's algorithm.

## Features

- **Single-word correction** (`lookup`) — find the closest matching dictionary word within a given edit distance
- **Multi-word correction** (`lookup_compound`) — correct entire sentences, handling word splits, joins, and replacements. Supports bigram frequency dictionaries for context-aware ranking
- **Word segmentation** (`word_segmentation`) — split concatenated words into their components (e.g. `"thequickbrownfox"` → `"the quick brown fox"`)
- **WebAssembly support** — compile to WASM for use in JavaScript/browser environments
- **Configurable string handling** — pluggable string strategies for ASCII transliteration or full Unicode support

## Usage

```rust
use symspell::{AsciiStringStrategy, SymSpell, Verbosity};

fn main() {
    let mut symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();

    symspell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    symspell.load_bigram_dictionary(
      "./data/frequency_bigramdictionary_en_243_342.txt",
      0,
      2,
      " "
    );

    let suggestions = symspell.lookup("roket", Verbosity::Top, 2);
    println!("{:?}", suggestions);

    let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";
    let compound_suggestions = symspell.lookup_compound(sentence, 2);
    println!("{:?}", compound_suggestions);

    let sentence = "whereisthelove";
    let segmented = symspell.word_segmentation(sentence, 2);
    println!("{:?}", segmented);
}
```

N.B. the dictionary entries have to be lowercase

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
* `UnicodeStringStrategy`
    * Doesn't do any preprocessing and handles strings as they are.
* `AsciiStringStrategy`
    * Transliterates strings into ASCII only characters.
    * Useful when you are working with accented languages and you don't want to care about accents, etc

To configure string strategy just pass it as a type parameter:

```rust
let mut ascii_symspell: SymSpell<AsciiStringStrategy> = SymSpell::default();
let mut unicode_symspell: SymSpell<UnicodeStringStrategy> = SymSpell::default();
```

### Javascript Bindings

This crate can be compiled against wasm32 target and exposes a SymSpell Class that can be used from Javascript as follow.
Only `UnicodeStringStrategy` is exported, meaning that if someone wants to manipulate ASCII only strings the dictionary and the sentences must be prepared in advance from JS.

```javascript
const fs = require('fs');
const rust = require('./pkg');

let dictionary = fs.readFileSync('data/frequency_dictionary_en_82_765.txt');
let sentence = "whereis th elove hehad dated forImuch of thepast who couqdn'tread in sixtgrade and ins pired him";

let symspell = new rust.SymSpell({ max_edit_distance: 2,  prefix_length: 7,  count_threshold: 1});
symspell.load_dictionary(dictionary.buffer, { term_index: 0,  count_index: 1, separator: " "});
symspell.load_bigram_dictionary(bigram_dict.buffer, { term_index: 0,  count_index: 2, separator: " "});
symspell.lookup_compound(sentence, 1);
```

It can be compiled using `wasm-pack` (eg. `wasm-pack build --release --target nodejs`)
