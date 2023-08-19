pub fn is_monotonic(nums: Vec<i32>) -> bool {
    nums.len() == 1 || nums.windows(2).all(|w| w[0] <= w[1]) || nums.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    use super::is_monotonic;

    #[test]
    fn should_return_true() {
        assert_eq!(is_monotonic(vec![1, 2, 2, 3]), true);
    }

    #[test]
    fn equal_true() {
        assert_eq!(is_monotonic(vec![6, 5, 4, 4]), true);
    }
}