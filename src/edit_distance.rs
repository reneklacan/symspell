use strsim::damerau_levenshtein;

pub fn distance(string: &str, other: &str, max_distance: i64) -> i64 {
    let distance = damerau_levenshtein(string, other);

    if distance <= max_distance as usize {
        distance as i64
    } else {
        -1
    }
}
