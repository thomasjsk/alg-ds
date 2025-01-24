pub fn quick_sort(
    arr: &mut Vec<i32>,
    lo: i32, // idx of the lo elem
    hi: i32, // idx of the hi elem
) -> () {
    if lo >= hi {
        return;
    }

    let pivot_idx = pivot(arr, lo, hi);

    quick_sort(arr, lo, pivot_idx - 1); // hi is exclusive pivot
    quick_sort(arr, pivot_idx + 1, hi); // lo is exclusive pivot
}

fn pivot(arr: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
    let pivot = arr[hi as usize];
    let mut idx = lo - 1;

    for i in lo..hi {
        if arr[i as usize] < pivot {
            idx += 1;
            arr.swap(i as usize, idx as usize);
        }
    }

    idx += 1;
    arr.swap(hi as usize, idx as usize);

    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr = vec![9, 3, 7, 4, 69, 420, 42];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [3, 4, 7, 9, 42, 69, 420]);
    }

    #[test]
    fn test_2() {
        let mut arr = vec![2, 5, 7, 3, 9, 1, 8, 6, 4];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
