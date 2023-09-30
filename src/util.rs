pub fn dummy_util() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_util_test() {
        assert!(dummy_util());
    }
}
