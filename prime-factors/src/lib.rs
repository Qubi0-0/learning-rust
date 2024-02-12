pub fn factors(n: u64) -> Vec<u64> {
    let mut divider: u64 = 2;
    let mut dividers: Vec<u64> = vec![];
    let mut value = n;
    while value != 1 {
        if value % divider == 0 {
            dividers.push(divider);
            value = value / divider;
            divider = 2;
        } else {
            divider = divider + 1;
        }
    }
    dividers
}
