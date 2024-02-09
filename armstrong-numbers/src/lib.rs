pub fn is_armstrong_number(num: u32) -> bool {
    let binding = num.to_string();
    let num_powered: u64 = binding
        .chars()
        .map(|x| {
            x.to_digit(10)
                .unwrap()
                .pow(binding.len().try_into().unwrap()) as u64
        })
        .sum();
    let mut result = false;
    if num_powered == num as u64 {
        result = true;
    }

    result
}
