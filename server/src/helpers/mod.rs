use std::ops::Deref;

pub struct TruncatedString(String);

impl TruncatedString {
    pub fn new(s: String, max_length: usize) -> Self {
        let mut result = Self(s);

        if result.0.len() > max_length {
            result.0.truncate(max_length);
            result.0.push_str("...");
        }

        result
    }
}

impl Deref for TruncatedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Truncatable {
    fn truncate_with_dots(&self, max_length: usize) -> String;
}

impl Truncatable for String {
    fn truncate_with_dots(&self, max_length: usize) -> String {
        TruncatedString::new(self.clone(), max_length).0
    }
}