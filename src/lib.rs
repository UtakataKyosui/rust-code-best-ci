/// Returns the successor, or `None` if it would overflow.
///
/// ```
/// assert_eq!(rust_code_best_ci::increment(41), Some(42));
/// assert_eq!(rust_code_best_ci::increment(u64::MAX), None);
/// ```
pub fn increment(value: u64) -> Option<u64> {
    if value < u64::MAX {
        Some(value + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_without_overflow() {
        assert_eq!(increment(0), Some(1));
        assert_eq!(increment(41), Some(42));
        assert_eq!(increment(u64::MAX - 1), Some(u64::MAX));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(increment(u64::MAX), None);
    }
}
