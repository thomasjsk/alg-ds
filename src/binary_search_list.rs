pub fn binary_search_list(haystack: [u32; 10], needle: u32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let v = haystack[m];

        if v == needle {
            return true;
        } else if v > needle {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*; // Import the parent module

    #[test]
    fn test() {
        assert_eq!(
            binary_search_list([1, 2, 3, 4, 5, 6, 7, 12, 13, 33], 5),
            true
        );
        assert_eq!(
            binary_search_list([1, 2, 3, 4, 5, 6, 7, 12, 13, 33], 100),
            false
        );
        assert_eq!(
            binary_search_list([1, 2, 3, 4, 5, 6, 7, 12, 13, 33], 0),
            false
        );
    }
}
