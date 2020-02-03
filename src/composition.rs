#[derive(Debug, Clone)]
pub struct Composition {
    pub segmented_string: String,
    pub distance_sum: i64,
    pub prob_log_sum: f64,
}

impl Composition {
    pub fn empty() -> Self {
        Self {
            segmented_string: "".to_string(),
            distance_sum: 0,
            prob_log_sum: 0.0,
        }
    }
}
