pub fn linear_search(haystack: [String; 10], needle: &String) -> bool {
    for i in 0..haystack.len() {
        if haystack[i] == *needle {
            return true;
        }
    }

    false
}
