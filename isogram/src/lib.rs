use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut count_map = HashMap::new();
    let mut letter_count = 0;
    for ch in candidate.chars() {
        if ch.is_alphabetic() {
            count_map.entry(ch.to_ascii_lowercase()).or_insert(0);
            letter_count += 1;
        }
    }
    count_map.len() == letter_count
}
