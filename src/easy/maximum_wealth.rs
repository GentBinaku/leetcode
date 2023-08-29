pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|ac| ac.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod test {
    use crate::easy::maximum_wealth;

    #[test]
    fn should_return_true() {
        assert_eq!(
            maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }
}