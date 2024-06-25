use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut bt_tree: BTreeMap<char, i32> = BTreeMap::new();
    for (key, vec) in h.iter() {
        for letter in vec {
            bt_tree.entry(letter.to_ascii_lowercase()).or_insert(*key);
        }
    }
    bt_tree
}
