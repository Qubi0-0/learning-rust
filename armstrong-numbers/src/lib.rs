pub fn is_armstrong_number(num: u32) -> bool {
    let result = num.to_string();

    result
        .chars()
        .map(|x| {
            x.to_digit(10)
                .unwrap()
                .pow(result.len().try_into().unwrap()) as u64
        })
        .sum::<u64>()
        == num as u64
}
