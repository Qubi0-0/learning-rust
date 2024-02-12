pub fn build_proverb(list: &[&str]) -> String {
    let mut story = String::new();
    for (idx,word) in list.iter().enumerate() {
        if idx < list.len() {
        story.push_str(&format!("For want of a {} the {} was lost \n",word, list[idx+1]));
        }
        else {
            story.push_str(&format!("And all for the want of a {}.", list[0]));
        }
    }
    story
}
 