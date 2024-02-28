pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut values = vec![];
    for factor in factors {
        if *factor == 0 {
            continue;
        }
        let mut multiplier = *factor;
        while multiplier < limit {
            values.push(multiplier);
            multiplier += factor;
        }
    }
    values.sort();
    values.dedup();
    values.iter().sum::<u32>()
}
