fn binary_search<T: PartialOrd>(target: &T, collection: &[T]) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = 0;

    while lo < hi {
        let mid: usize = (hi - lo) / 2 + lo;

        if *target == colection[mid] {
            return Some(mid);
        } else if *target < collection[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(None, binary_search(&3.14, &vec![0.0, 1.0, 2.0]));
        assert_eq!(None, binary_search(&4, &[1, 3]));
        assert_eq!(Some(2), binary_search(&2.0, &vec![0.0, 1.0, 2.0]));
        assert_eq!(Some(2), binary_search(&'c', &['a', 'b', 'c']));
        assert_eq!(Some(4), binary_search(&8_i32, &[0, 1, 2, 4, 8, 16]));
    }
}
