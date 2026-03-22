# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust port of the SymSpell spelling correction algorithm (Symmetric Delete spelling correction). Provides fast fuzzy string matching via pre-computed deletion-indexed dictionaries. Supports native Rust and WebAssembly targets.

## Commands

```bash
# Build
cargo build

# Test
cargo test
cargo test <test_name>            # Run a single test

# Lint
cargo clippy

# WASM
wasm-pack build --release --target nodejs
wasm-pack test --firefox --headless
```

## Architecture

The crate is generic over `StringStrategy`, which controls how strings are normalized:
- `UnicodeStringStrategy` — preserves Unicode characters
- `AsciiStringStrategy` — transliterates to ASCII via `unidecode` (non-WASM only)
- `UnicodeiStringStrategy` — case-insensitive Unicode

**Core types:**
- `SymSpell<T: StringStrategy>` (`src/symspell.rs`) — main struct holding the deletion-indexed dictionary (`deletes: HashMap<u64, Vec<Box<str>>>`), word frequencies (`words`), and bigram frequencies (`bigrams`). Constructed via `SymSpellBuilder`.
- `Suggestion` (`src/suggestion.rs`) — result type with term, edit distance, and frequency count.
- `Composition` (`src/composition.rs`) — word segmentation result.
- `Verbosity` enum — `Top` (best only), `Closest` (all at min distance), `All` (all within max distance).

**Key methods on `SymSpell`:**
- `load_dictionary` / `load_bigram_dictionary` — load frequency dictionaries from files
- `lookup` — single-word correction
- `lookup_compound` — multi-word sentence correction (handles splits/joins, uses bigrams for ranking)
- `word_segmentation` — segments concatenated words (e.g., "thequickbrownfox" → "the quick brown fox")

**WASM bindings** (`src/wasm.rs`) — `JSSymSpell` wraps `SymSpell<UnicodeStringStrategy>` for JavaScript via `wasm-bindgen`.

## Key Configuration

- `max_dictionary_edit_distance` (default: 2) — maximum edit distance for lookups
- `prefix_length` (default: 7) — prefix length for indexing
- `count_threshold` (default: 1) — minimum word frequency to include
- Clippy cognitive complexity threshold is set to 33 (`.clippy.toml`)

## Test Data

Dictionary files in `data/` are required by tests:
- `frequency_dictionary_en_82_765.txt`
- `frequency_bigramdictionary_en_243_342.txt`
