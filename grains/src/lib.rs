pub fn square(s: u32) -> u64 {
    if s > 0 && s < 65 {
        2_u64.pow(s - 1)
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    let s = 64;
    (1..=s).map(|s| 2_u64.pow(s - 1)).sum::<u64>()
}
