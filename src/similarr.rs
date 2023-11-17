use std::iter;

use regex::{Captures, Regex, Replacer};

struct NameSwapper;

pub struct ComparisonResult {
    pub expanded_a: String,
    pub expanded_b: String,
    pub result: bool,
}

impl Replacer for NameSwapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let iterations = caps[1].parse::<i32>().unwrap();
        let i = iter::repeat("*")
            .take(iterations as usize);
        dst.extend(i);
    }
}

fn expand2(text: &str) -> String {
    let regex = Regex::new(r"(\d+)+").unwrap();
    regex.replace_all(text, NameSwapper).to_string()
}

pub fn compare(text_a: &str, text_b: &str) -> ComparisonResult {
    let expanded_a = expand2(text_a);
    let expanded_b = expand2(text_b);

    let result = similarr(&expanded_a, &expanded_b);
    ComparisonResult {
        expanded_a,
        expanded_b,
        result
    }
}

fn similarr(text_a: &str, text_b: &str) -> bool {
    println!("Comparing '{}' with '{}'", text_a, text_b);

    if text_a.len() != text_b.len() {
        return false;
    }

    for a in text_a.chars().zip(text_b.chars()) {
        match a {
            ('*', _) | (_, '*') => (),
            (a, b)  if a != b => return false,
            _ => ()
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_truthy() {
        assert!(compare( "ap2e", "a4").result);
        assert!(!compare("ap2e", "a6").result);

        assert!(compare("casa", "ca1a").result);
        assert!(!compare("casa", "ca1x").result);

        assert!(compare("hypothetical", "h11").result);
        assert!(compare("hypothetically", "h12y").result);

        assert!(compare("1or1o", "co1p1").result);
    }
}