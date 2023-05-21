use std::{fmt::Debug, cmp::Ordering};

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

    fn test_executor<F, T>(f: F, data: &mut [T], order: Ordering) -> ()
    where
        F: Fn(&mut [T], Ordering) -> &mut [T],
        T: Ord,
    {
        f(data, order);
    }

    trait Utils<T> {
        fn get() -> Vec<T>;
        fn get_sorted() -> Vec<T>;
    }
    struct DataProvider {}
    impl Utils<i32> for DataProvider {
        fn get() -> Vec<i32> {
            vec![5, 2, 3, 10, 1, -20, 20, -2, 4, 2]
        }
        fn get_sorted() -> Vec<i32> {
            vec![-20, -2, 1, 2, 2, 3, 4, 5, 10, 20]
        }
    }

    impl Utils<char> for DataProvider {
        fn get() -> Vec<char> {
            vec!['a', '🔥', 'B', 'A', 'Z', 'c', 'z']
        }

        fn get_sorted() -> Vec<char> {
            vec!['A', 'B', 'Z', 'a', 'c', 'z', '🔥']
        }
    }
}
