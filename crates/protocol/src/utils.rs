//! Utility methods used by protocol types.

/// Returns if the given `value` is a deposit transaction.
pub fn is_deposit<B>(value: &B) -> bool
where
    B: AsRef<[u8]>,
{
    !value.as_ref().is_empty() && value.as_ref()[0] == 0x7E
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_deposit() {
        assert!(is_deposit(&[0x7E]));
        assert!(!is_deposit(&[]));
        assert!(!is_deposit(&[0x7F]));
    }
}
