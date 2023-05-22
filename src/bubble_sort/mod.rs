use std::{cmp::Ordering, fmt::Debug};

pub fn bubble_sort<T>(slice: &mut [T], order: Ordering) -> &mut [T]
where
    T: Ord + Debug,
{
    let mut j = slice.len() - 1;
    while j > 0 {
        let mut i = 0;
        while i < j {
            if order == slice[i].cmp(&slice[i + 1]) {
                slice.swap(i, i + 1);
            }
            i += 1;
        }
        j -= 1;
    }
    slice
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{DataProvider, TestUtils, test_executor};

    use super::{bubble_sort, Ordering};

    #[test]
    fn bubble_test() {
        let mut unsorted_vec: Vec<i32> = DataProvider::get();
        let mut unsorted_char_vec: Vec<char> = DataProvider::get();

        test_executor(bubble_sort, &mut unsorted_vec, Ordering::Greater);
        assert_eq!(unsorted_vec, DataProvider::get_sorted());
        test_executor(bubble_sort, &mut unsorted_char_vec, Ordering::Greater);
        assert_eq!(unsorted_char_vec, DataProvider::get_sorted());
    }
}
