#[cfg(not(target_arch = "wasm32"))]
use unidecode::unidecode;

pub trait StringStrategy: Clone + Default {
    fn new() -> Self;
    fn prepare(&self, s: &str) -> String;
    fn len(&self, s: &str) -> usize;
    fn remove(&self, s: &str, index: usize) -> String;
    fn slice(&self, s: &str, start: usize, end: usize) -> String;
    fn suffix(&self, s: &str, start: usize) -> String;
    fn at(&self, s: &str, i: isize) -> Option<char>;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub struct AsciiStringStrategy {}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
impl StringStrategy for AsciiStringStrategy {
    fn new() -> Self {
        Self {}
    }

    fn prepare(&self, s: &str) -> String {
        unidecode(s)
    }

    fn len(&self, s: &str) -> usize {
        s.len()
    }

    fn remove(&self, s: &str, index: usize) -> String {
        let mut x = s.to_string();
        x.remove(index);
        x
    }

    fn slice(&self, s: &str, start: usize, end: usize) -> String {
        unsafe { s.get_unchecked(start..end) }.to_string()
    }

    fn suffix(&self, s: &str, start: usize) -> String {
        self.slice(s, start, s.len())
    }

    fn at(&self, s: &str, i: isize) -> Option<char> {
        if i < 0 || i >= s.len() as isize {
            return None;
        }

        Some(s.as_bytes()[i as usize] as char)
    }
}

// backward compatibility on typo
pub type UnicodeiStringStrategy = UnicodeStringStrategy;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub struct UnicodeStringStrategy {}

impl StringStrategy for UnicodeStringStrategy {
    fn new() -> Self {
        Self {}
    }

    fn prepare(&self, s: &str) -> String {
        s.to_string()
    }

    fn len(&self, s: &str) -> usize {
        s.chars().count()
    }

    fn remove(&self, s: &str, index: usize) -> String {
        s.chars()
            .enumerate()
            .filter(|(ii, _)| ii != &index)
            .map(|(_, ch)| ch)
            .collect()
    }

    fn slice(&self, s: &str, start: usize, end: usize) -> String {
        s.chars().skip(start).take(end - start).collect()
    }

    fn suffix(&self, s: &str, start: usize) -> String {
        s.chars().skip(start).collect::<String>()
    }

    fn at(&self, s: &str, i: isize) -> Option<char> {
        if i < 0 || i >= s.len() as isize {
            return None;
        }

        s.chars().nth(i as usize)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare() {
        assert_eq!(AsciiStringStrategy::new().prepare("čičina"), "cicina");
    }

    #[test]
    fn ascii_slice_prefix() {
        assert_eq!(AsciiStringStrategy::new().slice("daleko", 0, 3), "dal");
    }

    #[test]
    fn ascii_slice_suffix() {
        assert_eq!(AsciiStringStrategy::new().slice("daleko", 3, 6), "eko");
    }

    #[test]
    fn ascii_remove() {
        assert_eq!(AsciiStringStrategy::new().remove("daleko", 2), "daeko");
    }

    #[test]
    fn ascii_at_negative() {
        assert_eq!(AsciiStringStrategy::new().at("daleko", -2), None);
    }

    #[test]
    fn ascii_at_correct() {
        assert_eq!(AsciiStringStrategy::new().at("daleko", 3), Some('e'));
    }

    #[test]
    fn ascii_at_over_limit() {
        assert_eq!(AsciiStringStrategy::new().at("daleko", 6), None);
    }

    #[test]
    fn unicodei_strategy() {
        assert_eq!(UnicodeiStringStrategy::new().prepare("ciccio"), "ciccio");
    }
}
