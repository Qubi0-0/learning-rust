pub fn translate(input: &str) -> String {
    let vowel_list: [char; 6] = ['a','e','i','o','u','y'];

    let phrase: &str = "ay";

    if vowel_list.contains(&input.chars().nth(0).unwrap_or_default()) {
        return input.to_string() + phrase
    } else {
        let word = input.chars().skip(1).collect::<String>();
        if word.starts_with("qu") {
            format!("{}{}{}",input.chars().next().unwrap_or_default(),word,"qu")
        } else if !word.contains(){
            
        } else {
    let mut chars = word.chars();
    let first_vowel_index = chars.position(|c| "aeiou".contains(c)).unwrap_or_else(|| word.len());
    let (start, end) = word.split_at(first_vowel_index);
    format!("{}{}ay", end, start)
}
    }
}
    







 