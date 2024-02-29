pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let phrase = phrase.replace("-", " ");
    let phrase = phrase
        .chars()
        .filter(|ch| ch.is_alphabetic() || ch.is_whitespace())
        .collect::<String>();
    let phrases: Vec<&str> = phrase.split_whitespace().collect();
    
    for phrase in phrases {
        acronym.push(phrase.trim_start().chars().next().unwrap())
    }
    acronym.to_uppercase()
}
