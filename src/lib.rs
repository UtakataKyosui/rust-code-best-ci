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

// Verify the production function, rather than a separate copy of its logic.
#[cfg(kani)]
mod proofs {
    use super::increment;

    #[kani::proof]
    fn increment_matches_mathematical_successor() {
        let value: u64 = kani::any();
        match increment(value) {
            Some(next) => assert_eq!(u128::from(next), u128::from(value) + 1),
            None => assert_eq!(value, u64::MAX),
        }
        assert_eq!(increment(value).is_none(), value == u64::MAX);
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
