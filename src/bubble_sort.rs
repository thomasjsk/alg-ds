pub fn bubble_sort(arr: &mut [u32]) {
    for i in (0..=arr.len()).rev() {
        for j in 1..i {
            if arr[j - 1] > arr[j] {
                let temp = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut arr = [9, 3, 7, 4, 69, 420, 42];
        bubble_sort(&mut arr);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 420]);
    }
}
