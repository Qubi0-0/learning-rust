pub fn build_proverb(list: &[&str]) -> String {
    let mut story = String::new();
    for (idx, word) in list.iter().skip(1).enumerate() {
        if idx < list.len() - 1 {
            story.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[idx], word
            ));
        }
    }
    story.push_str(&format!("And all for the want of a {}.", list[0]));
    story
}
