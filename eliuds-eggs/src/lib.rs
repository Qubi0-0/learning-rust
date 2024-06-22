pub fn egg_count(display_value: u32) -> usize {
    let binary = score_to_binary(display_value);

    binary.iter().filter(|x| **x == 1).sum::<u32>() as usize
}

fn score_to_binary(score: u32) -> Vec<u32> {
    let mut binary = Vec::new();
    for i in 0..32 {
        binary.push((score >> i) & 1_u32);
    }
    binary
}
