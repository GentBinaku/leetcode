pub fn is_robot_bounded(instructions: String) -> bool {
    let mut d = (0, 1);
    let mut p = (0, 0);

    for c in instructions.chars() {
        match c {
            'G' => {
                p.0 += d.0;
                p.1 += d.1;
            },
            'L' => {
                d = (-d.1, d.0);
            },
            'R' => {
                d = (d.1, -d.0);
            },
            _ => (),
        }
    }
    p == (0, 0) || d != (0, 1)
}


#[cfg(test)]
mod test {
    use crate::medium::is_robot_bounded;

    #[test]
    fn should_return_true() {
        assert_eq!(is_robot_bounded("GGLLGG".to_string()), true);
    }
}
