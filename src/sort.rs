//! All kinds of Sorting Algorithm
/// quicksort algorithm
/// assert!(hi > lo);
use std::mem::take;
/// quick sort algorithm
pub fn quicksort<T: Ord + Default>(slice: &mut [T], lo: usize, hi: usize) {
    if lo + 2 > hi {
        return;
    }

    let mut mi = lo;

    for k in lo + 1..hi {
        if slice[k] < slice[lo] {
            mi += 1;
            let temp_mi = take(&mut slice[mi]);
            let temp_k = take(&mut slice[k]);
            slice[mi] = temp_k;
            slice[k] = temp_mi;
        }
    }

    let pivot = take(&mut slice[lo]);
    let temp_mi = take(&mut slice[mi]);
    slice[lo] = temp_mi;
    slice[mi] = pivot;

    quicksort(slice, lo, mi);
    quicksort(slice, mi + 1, hi);
}

/// find maj number or median number
pub fn find_majory<T: Ord + Default + Copy>(slice: &mut [T]) -> T {
    let mut maj = T::default();
    let mut c = 0;
    for i in 0..slice.len() {
        if c == 0 {
            maj = slice[i];
            c = 1;
        } else {
            if maj == slice[i] {
                c += 1;
            } else {
                c -= 1;
            }
        }
    }

    maj
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quicksort() {
        let mut array = [3, 4, 2, 5, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (0, array.len());
        quicksort(&mut array, lo, hi);
        assert_eq!([2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390], array);

        array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (3, array.len() - 5);
        quicksort(&mut array, lo, hi);
        assert_eq!([3, 4, 5, 2, 12, 19, 39, 43, 48, 390, 8, 9, 10, 7, 5], array);

        array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (3, array.len());
        quicksort(&mut array, lo, hi);
        assert_eq!([3, 4, 5, 2, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390], array);

        let mut array = [3, 4, 5, -3, -10, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (0, array.len());
        quicksort(&mut array, lo, hi);
        assert_eq!(
            [-10, -3, 2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390],
            array
        );

        let mut move_array = [
            format!("{}", 3),
            format!("{}", 4),
            format!("{}", 5),
            format!("{}", 2),
            format!("{}", 48),
            format!("{}", 19),
            format!("{}", 39),
            format!("{}", 390),
            format!("{}", 43),
            format!("{}", 12),
            format!("{}", 8),
            format!("{}", 9),
            format!("{}", 10),
            format!("{}", 7),
            format!("{}", 5),
        ];
        let (lo, hi) = (0, move_array.len());
        quicksort(&mut move_array, lo, hi);
        assert_eq!(
            [
                format!("{}", 10),
                format!("{}", 12),
                format!("{}", 19),
                format!("{}", 2),
                format!("{}", 3),
                format!("{}", 39),
                format!("{}", 390),
                format!("{}", 4),
                format!("{}", 43),
                format!("{}", 48),
                format!("{}", 5),
                format!("{}", 5),
                format!("{}", 7),
                format!("{}", 8),
                format!("{}", 9),
            ],
            move_array
        );
    }
}
