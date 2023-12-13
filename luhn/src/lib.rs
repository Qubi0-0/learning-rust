/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if !code.chars().all(|c| c.is_digit(10)) {
        return false;
    }
    if code.len() > 1 {
        let doubled_code: Vec<u32> = code
            .chars()
            .rev()
            .enumerate()
            .map(|(index, digit)| {
                if let Some(digit) = digit.to_digit(10) {
                    if index % 2 == 1 {
                        let doubled = digit * 2;
                        if doubled > 9 {
                            doubled - 9
                        } else {
                            doubled
                        }
                    } else {
                        digit
                    }
                } else {
                    0
                }
            })
            .collect();
        let sum = doubled_code.into_iter().sum::<u32>();
        if sum % 10 == 0 {
            return true;
        }
    }
    return false;
}
