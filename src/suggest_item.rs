use std::cmp::Ordering;

#[derive(Debug)]
pub struct SuggestItem {
    pub term: String,
    pub distance: i64,
    pub count: i64,
}

impl SuggestItem {
    pub fn new(term: &str, distance: i64, count: i64) -> SuggestItem {
        SuggestItem {
            term: term.to_string(),
            distance: distance,
            count: count,
        }
    }
}

impl Ord for SuggestItem {
    fn cmp(&self, other: &SuggestItem) -> Ordering {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return self.count.cmp(&other.count);
        }
        distance_cmp
    }
}

impl PartialOrd for SuggestItem {
    fn partial_cmp(&self, other: &SuggestItem) -> Option<Ordering> {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return Some(self.count.cmp(&other.count));
        }
        Some(distance_cmp)
    }
}

impl PartialEq for SuggestItem {
    fn eq(&self, other: &SuggestItem) -> bool {
        // self.term == other.term
        self.distance == other.distance && self.count == other.count
    }
}
impl Eq for SuggestItem {}