

pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    // input.chars().rev().collect()
    let mut result = String::new();
    for char in input.chars().rev() {
        result.push(char);
    }
    result

}
