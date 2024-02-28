use std::collections::HashMap;

pub fn brackets_are_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    let bracket_pairs: HashMap<char, char> = [('}', '{'), (']', '['), (')', '(')]
        .iter()
        .cloned()
        .collect();

    for ch in s.chars() {
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            '}' | ']' | ')' => {
                if stack.is_empty() {
                    return false;
                }
                if *stack.last().unwrap() != *bracket_pairs.get(&ch).unwrap() {
                    return false;
                }
                stack.pop();
            }
            _ => (),
        }
    }

    stack.is_empty()
}
