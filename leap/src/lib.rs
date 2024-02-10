pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 {
        if year % 400 == 0 {
            true
        } else {
            false
        }
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}
