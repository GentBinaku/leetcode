pub fn repeated_substring_pattern(s: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let divisors = find_all_divisors(s.len());
    for d in divisors {
        let sub = &s[..d];
        if check(&s, sub) {
            return true;
        }
    }
    return false;
}

fn find_all_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn check(s: &Vec<char>, sub: &[char]) -> bool {
    for i in (0..s.len()).step_by(sub.len()) {
        if s[i..i + sub.len()] != sub[..] {
            return false;
        }
    }
    return true
}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    #[test]
    fn should_return_true() {
        assert_eq!(repeated_substring_pattern("abab".to_string()), true);
    }
}