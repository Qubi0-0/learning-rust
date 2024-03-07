pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut size = array.len();
    let mut base: usize = 0;
    while size > 0 {
        let half = size / 2;
        let mid = base + half;
        let cmp = &array[mid];
        match cmp.cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => {
                base = mid + 1;
                size -= half + 1;
            }
            std::cmp::Ordering::Greater => size = half,
        }
    }
    None
}
