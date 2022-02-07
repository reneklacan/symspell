use std::cmp::Ordering;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Suggestion {
    pub term: String,
    pub distance: i64,
    pub count: i64,
}

impl Suggestion {
    pub fn empty() -> Suggestion {
        Suggestion {
            term: "".to_string(),
            distance: 0,
            count: 0,
        }
    }

    pub fn new(term: impl Into<String>, distance: i64, count: i64) -> Suggestion {
        Suggestion {
            term: term.into(),
            distance,
            count,
        }
    }
}

impl Ord for Suggestion {
    fn cmp(&self, other: &Suggestion) -> Ordering {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return self.count.cmp(&other.count);
        }
        distance_cmp
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Suggestion) -> Option<Ordering> {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return Some(self.count.cmp(&other.count));
        }
        Some(distance_cmp)
    }
}

impl PartialEq for Suggestion {
    fn eq(&self, other: &Suggestion) -> bool {
        // self.term == other.term
        self.distance == other.distance && self.count == other.count
    }
}
impl Eq for Suggestion {}
