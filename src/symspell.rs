use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;
use unidecode::unidecode;

use edit_distance::{EditDistance, DistanceAlgorithm};
use string_strategy::{StringStrategy};
use suggest_item::SuggestItem;


#[derive(Eq, PartialEq, Debug)]
pub enum Verbosity {
    Top,
    Closest,
    All,
}

pub struct SymSpell<T: StringStrategy> {
    max_dictionary_edit_distance: i64,
    prefix_length: i64,
    count_threshold: i64,
    max_length: i64,
    deletes: HashMap<u64, Vec<String>>,
    words: HashMap<String, i64>,
    distance_algorithm: DistanceAlgorithm,
    string_strategy: T,
}

impl<T: StringStrategy> SymSpell<T> {
    pub fn new(
        max_dictionary_edit_distance: i64,
        prefix_length: i64,
        count_threshold: i64,
    ) -> SymSpell<T> {
        SymSpell {
            max_dictionary_edit_distance: max_dictionary_edit_distance,
            prefix_length: prefix_length,
            count_threshold: count_threshold,
            max_length: 0,
            deletes: HashMap::new(),
            words: HashMap::new(),
            distance_algorithm: DistanceAlgorithm::Damerau,
            string_strategy: T::new(),
        }
    }

    pub fn new_with_defaults() -> SymSpell<T> {
        SymSpell {
            max_dictionary_edit_distance: 2,
            prefix_length: 7,
            count_threshold: 1,
            max_length: 0,
            deletes: HashMap::new(),
            words: HashMap::new(),
            distance_algorithm: DistanceAlgorithm::Damerau,
            string_strategy: T::new(),
        }
    }

    pub fn lookup(
        &self,
        input: &str,
        verbosity: Verbosity,
        max_edit_distance: i64,
    ) -> Vec<SuggestItem> {
        if max_edit_distance > self.max_dictionary_edit_distance {
            panic!("max_edit_distance is bigger than max_dictionary_edit_distance");
        }

        let mut suggestions: Vec<SuggestItem> = Vec::new();

        let input = &unidecode(input);
        let input_len = self.string_strategy.len(input) as i64;

        if input_len - self.max_dictionary_edit_distance > self.max_length {
            return suggestions;
        }

        let mut hashset1: HashSet<String> = HashSet::new();
        let mut hashset2: HashSet<String> = HashSet::new();

        if self.words.contains_key(input) {
            let suggestion_count = self.words[input];
            suggestions.push(SuggestItem::new(input, 0, suggestion_count));

            if verbosity != Verbosity::All {
                return suggestions;
            }
        }

        hashset2.insert(input.to_string());

        let mut max_edit_distance2 = max_edit_distance;
        let mut candidate_pointer = 0;
        let mut candidates = Vec::new();

        let mut input_prefix_len = input_len;

        if input_prefix_len > self.prefix_length {
            input_prefix_len = self.prefix_length;
            candidates.push(
                self.string_strategy
                    .slice(input, 0, input_prefix_len as usize),
            );
        } else {
            candidates.push(input.to_string());
        }

        let distance_comparer = EditDistance::new(input, self.distance_algorithm.clone());

        while candidate_pointer < candidates.len() {
            let candidate = &candidates.get(candidate_pointer).unwrap().clone();
            candidate_pointer += 1;
            let candidate_len = self.string_strategy.len(candidate) as i64;
            let length_diff = input_prefix_len - candidate_len;

            if length_diff > max_edit_distance2 {
                if verbosity == Verbosity::All {
                    continue;
                }
                break;
            }

            if self.deletes.contains_key(&self.get_string_hash(&candidate)) {
                let dist_suggestions = &self.deletes[&self.get_string_hash(&candidate)];

                for suggestion in dist_suggestions {
                    let suggestion_len = self.string_strategy.len(suggestion) as i64;

                    if suggestion == input {
                        continue;
                    }

                    if (suggestion_len - input_len).abs() > max_edit_distance2
                        || suggestion_len < candidate_len
                        || (suggestion_len == candidate_len && suggestion != candidate)
                    {
                        continue;
                    }

                    let sugg_prefix_len = cmp::min(suggestion_len, self.prefix_length);

                    if sugg_prefix_len > input_prefix_len
                        && sugg_prefix_len - candidate_len > max_edit_distance2
                    {
                        continue;
                    }

                    let mut distance;

                    let input_suggestion_len_min = cmp::min(input_len, suggestion_len) as i64;

                    if candidate_len == 0 {
                        distance = cmp::max(input_len, suggestion_len);

                        if distance > max_edit_distance2 || hashset2.contains(suggestion) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());
                    } else if suggestion_len == 1 {
                        distance = if !input
                            .contains(suggestion.chars().take(1).collect::<String>().as_str())
                        {
                            input_len
                        } else {
                            input_len - 1
                        };

                        if distance > max_edit_distance2 || hashset2.contains(suggestion) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());
                    } else if (self.prefix_length - max_edit_distance == candidate_len)
                        && (((input_suggestion_len_min - self.prefix_length) > 1)
                            && (self.string_strategy
                                .suffix(input, (input_len + 1 - input_suggestion_len_min) as usize)
                                != self.string_strategy.suffix(
                                    suggestion,
                                    (suggestion_len + 1 - input_suggestion_len_min) as usize,
                                )))
                        || ((input_suggestion_len_min > 0)
                            && (self.string_strategy
                                .at(input, (input_len - input_suggestion_len_min) as isize)
                                != self.string_strategy.at(
                                    input,
                                    (suggestion_len - input_suggestion_len_min) as isize,
                                ))
                            && ((self.string_strategy
                                .at(input, (input_len - input_suggestion_len_min - 1) as isize)
                                != self.string_strategy.at(
                                    input,
                                    (suggestion_len - input_suggestion_len_min) as isize,
                                ))
                                || (self.string_strategy
                                    .at(input, (input_len - input_suggestion_len_min) as isize)
                                    != self.string_strategy.at(
                                        input,
                                        (suggestion_len - input_suggestion_len_min - 1) as isize,
                                    )))) {
                        continue;
                    } else {
                        if verbosity != Verbosity::All
                            && self.delete_in_suggestion_prefix(
                                candidate,
                                candidate_len,
                                suggestion,
                                suggestion_len,
                            ) {
                            continue;
                        }

                        if hashset2.contains(suggestion) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());

                        distance = distance_comparer.compare(suggestion, max_edit_distance2);

                        if distance < 0 {
                            continue;
                        }
                    }

                    if distance <= max_edit_distance2 {
                        let suggestion_count = self.words[suggestion];
                        let si = SuggestItem::new(suggestion, distance, suggestion_count);

                        if suggestions.len() > 1 {
                            match verbosity {
                                Verbosity::Closest => {
                                    if distance < max_edit_distance2 {
                                        suggestions.clear();
                                    }
                                    break;
                                }
                                Verbosity::Top => {
                                    if distance < max_edit_distance2
                                        || suggestion_count > suggestions[0].count
                                    {
                                        max_edit_distance2 = distance;
                                        suggestions[0] = si;
                                    }
                                    continue;
                                }
                                _ => (),
                            }
                        }

                        if verbosity != Verbosity::All {
                            max_edit_distance2 = distance;
                        }

                        suggestions.push(si);
                    }
                }
            }

            if (length_diff < max_edit_distance) && (candidate_len <= self.prefix_length) {
                if verbosity != Verbosity::All && length_diff >= max_edit_distance2 {
                    continue;
                }

                for i in 0..candidate.chars().count() {
                    let delete = self.string_strategy.remove(candidate, i);

                    if !hashset1.contains(&delete) {
                        hashset1.insert(delete.clone());
                        candidates.push(delete);
                    }
                }
            }
        }

        if suggestions.len() > 1 {
            suggestions.sort();
        }

        suggestions
    }

    fn delete_in_suggestion_prefix(
        &self,
        delete: &str,
        delete_len: i64,
        suggestion: &str,
        suggestion_len: i64,
    ) -> bool {
        if delete_len == 0 {
            return true;
        }
        let suggestion_len = if self.prefix_length < suggestion_len {
            self.prefix_length
        } else {
            suggestion_len
        };
        let mut j = 0;
        for i in 0..delete_len {
            let del_char = delete.chars().nth(i as usize).unwrap();
            while j < suggestion_len && del_char != suggestion.chars().nth(j as usize).unwrap() {
                j += 1;
            }
            if j == suggestion_len {
                return false;
            }
        }
        true
    }

    pub fn load_dictionary(
        &mut self,
        corpus: &str,
        term_index: i64,
        count_index: i64,
        separator: &str,
        max_records_count: usize,
    ) -> bool {
        if !Path::new(corpus).exists() {
            return false;
        }

        let file = File::open(corpus).expect("file not found");
        let sr = BufReader::new(file);

        for (i, line) in sr.lines().enumerate() {
            if i == max_records_count {
                break;
            }

            if i % 10_000 == 0 {
                println!("progress: {}", i);
            }
            let line_str = line.unwrap();
            let line_parts: Vec<&str> = line_str.split(separator).collect();

            if line_parts.len() >= 2 {
                // let key = unidecode(line_parts[term_index as usize]);
                let key = self.string_strategy
                    .prepare(line_parts[term_index as usize]);
                let count = line_parts[count_index as usize].parse::<i64>().unwrap();

                self.create_dictionary_entry(key, count);
            }
        }

        println!("deletes.len(): {}", self.deletes.len());
        println!("words.len(): {}", self.words.len());

        true
    }

    fn create_dictionary_entry(&mut self, key: String, count: i64) -> bool {
        if count < self.count_threshold {
            return false;
        }

        self.words.insert(key.clone(), count);

        let key_len = self.string_strategy.len(&key);

        if key_len as i64 > self.max_length {
            self.max_length = key_len as i64;
        }

        let edits = self.edits_prefix(&key);

        for delete in edits {
            let delete_hash = self.get_string_hash(&delete);

            if self.deletes.contains_key(&delete_hash) {
                let suggestions = self.deletes.get_mut(&delete_hash).unwrap();
                suggestions.push(key.clone());
            } else {
                self.deletes.insert(delete_hash, vec![key.to_string()]);
            }
        }

        true
    }

    fn edits_prefix(&self, key: &str) -> HashSet<String> {
        let mut hash_set = HashSet::new();

        let key_chars_count = self.string_strategy.len(key) as i64;

        if key_chars_count <= self.max_dictionary_edit_distance {
            hash_set.insert("".to_string());
        }

        if key_chars_count > self.prefix_length {
            hash_set.insert(
                self.string_strategy
                    .slice(key, 0, self.prefix_length as usize),
            );
        } else {
            hash_set.insert(key.to_string());
        }

        self.edits(key, 0, &mut hash_set);

        hash_set
    }

    fn edits(&self, word: &str, edit_distance: i64, delete_words: &mut HashSet<String>) {
        let edit_distance = edit_distance + 1;
        let word_len = self.string_strategy.len(word);

        if word_len > 1 {
            for i in 0..word_len {
                let delete = self.string_strategy.remove(word, i);

                if !delete_words.contains(&delete) {
                    delete_words.insert(delete.clone());

                    if edit_distance < self.max_dictionary_edit_distance {
                        self.edits(&delete, edit_distance, delete_words);
                    }
                }
            }
        }
    }

    fn get_string_hash(&self, s: &String) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
}