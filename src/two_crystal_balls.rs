pub fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jmp = (breaks.len() as f64).sqrt().floor() as i32;
    let mut i: i32 = jmp;

    while i < breaks.len() as i32 {
        if breaks[i as usize] {
            break;
        }

        i += jmp;
    }

    i = i - jmp;

    while i < breaks.len() as i32 {
        if breaks[i as usize] {
            return i;
        }
        i += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(two_crystal_balls(&[false, false, true, true]), 2);
        assert_eq!(two_crystal_balls(&[false, false, false, true, true]), 3);
        assert_eq!(
            two_crystal_balls(&[false, true, true, true, true, true, true, true]),
            1
        );
        assert_eq!(
            two_crystal_balls(&[false, false, false, false, false, false, false, true]),
            7
        );
    }
}
