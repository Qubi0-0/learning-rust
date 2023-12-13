use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let sorted_word = sort_word(&word_lower);
    let mut results = HashSet::new();
    for candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        if candidate_lower.len() == word.len() 
        && candidate_lower != word_lower
        && sort_word(&candidate_lower) == sorted_word {
            results.insert(*candidate);
        }
    }
    results
}

pub fn sort_word(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort_unstable();
    sorted_word
}