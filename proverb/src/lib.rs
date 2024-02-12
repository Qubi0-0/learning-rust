pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut story = String::new();
    list.iter()
        .skip(1)
        .enumerate()
        .map(|(idx, word)| {
            if idx < list.len() - 1 {
                story.push_str(&format!(
                    "For want of a {} the {} was lost.\n",
                    list[idx], word
                ));
            }
        })
        .for_each(|_| {});
    story.push_str(&format!("And all for the want of a {}.", list[0]));
    story
}
