pub fn collatz(n: u64) -> Option<u64> {
    if n > 0 {
        let mut iter = 0;
        let mut number = n;
        while number > 1 {
            if number % 2 == 0 {
                number /= 2;
                iter += 1;
            } else {
                number = number * 3 + 1;
                iter += 1;
            }
        }
        Some(iter)
    } else {
        None
    }
}
