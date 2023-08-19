pub fn roman_to_int(s: String) -> i32 {
    /*
        I placed before V or X indicates one less, so four is IV (one less than 5) and 9 is IX (one less than 10).
        X placed before L or C indicates ten less, so forty is XL (10 less than 50) and 90 is XC (ten less than a hundred).
        C placed before D or M indicates a hundred less, so four hundred is CD (a hundred less than five hundred) and nine hundred is CM (a hundred less than a thousand).
   */
    let (mut m, mut c, mut x, mut i) = (0, 0, 0, 0);
    for b in s.as_bytes() {
        match b {
            b'M' => { m += 1000 - c; c = 0; },
            b'D' => { m += 500 - c; c = 0; },
            b'C' => { c += 100 - x; x = 0; },
            b'L' => { c += 50 - x; x = 0; },
            b'X' => { x += 10 - i; i = 0; },
            b'V' => { x += 5 - i; i = 0; },
            _ => i += 1,
        }
    }
    m + c + x + i
}

#[cfg(test)]
mod test {
    use crate::easy::roman_to_int;

    fn should_true(){
        assert_eq!(roman_to_int("III".to_string()), 3);
    }
}