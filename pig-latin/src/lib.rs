pub fn translate(input: &str) -> String {
    let vowel_list: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let phrase: &str = "ay";
    if &input[0..=1] == "xr" || &input[0..=1] == "yt" {
        return input.to_string() + phrase;
    }
    if vowel_list.contains(&input.chars().next().unwrap_or_default()) {
        return input.to_string() + phrase;
    } else {
        let mut suffix = String::new();
        for char in input.chars() {
            if !vowel_list.contains(&char) {
                suffix.push(char);
            }
        }
        if input
            .chars()
            .skip(suffix.len())
            .collect::<String>()
            .starts_with("qu")
        {
            format!(
                "{}{}{}",
                input.chars().skip(suffix.len()).collect::<String>(),
                suffix,
                "qu"
            )
        } else {
            todo!()
        }
    }
}
