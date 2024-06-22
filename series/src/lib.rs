pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];
    if digits.len() >= len {
        let mut i = 0;
        while i <= digits.len() - len {
            result.push(digits[i..i + len].to_string());
            i += 1;
        }
    }
    result
}
