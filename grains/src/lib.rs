pub fn square(s: u32) -> u64 {
    if s > 0 && s < 65 {
        2_u64.pow(s - 1)
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    2 * (2_u64.pow(63) - 1) + 1
}