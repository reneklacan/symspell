use strsim::damerau_levenshtein;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DistanceAlgorithm {
    Damerau,
}

pub struct EditDistance {
    base_string: String,
    algorithm: DistanceAlgorithm,
}

impl EditDistance {
    pub fn new(base_string: &str, distance_algorithm: DistanceAlgorithm) -> EditDistance {
        EditDistance {
            base_string: base_string.to_string(),
            algorithm: distance_algorithm,
        }
    }

    pub fn compare(&self, other: &str, max_distance: i64) -> i64 {
        let distance = match self.algorithm {
            DistanceAlgorithm::Damerau => damerau_levenshtein(&self.base_string, other),
        };

        if distance <= max_distance as usize {
            distance as i64
        } else {
            -1
        }
    }
}