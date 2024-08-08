use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut count_map = HashMap::new();
    for str_slice in sentence.chars() {
        if str_slice.is_alphabetic() {
            count_map.entry(str_slice.to_ascii_lowercase()).or_insert(0);
        }
    }
    count_map.len() == 26
}
