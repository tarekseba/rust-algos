use std::fmt::Debug;

pub fn bubble_sort<T>(slice: &mut [T]) -> &mut [T]
where
    T: Ord + Debug,
{
    let mut j = slice.len() - 1;
    while j > 0 {
        let mut i = 0;
        while i < j {
            if let std::cmp::Ordering::Greater = slice[i].cmp(&slice[i + 1]) {
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
    use super::bubble_sort;
    #[test]
    fn bubble_sort_test() {
        let mut unsorted_vec: Vec<i32> = DataProvider::get();
        let mut unsorted_char_vec: Vec<char> = DataProvider::get();

        test_executor(bubble_sort, &mut unsorted_vec);
        assert_eq!(unsorted_vec, DataProvider::get_sorted());
        test_executor(bubble_sort, &mut unsorted_char_vec);
        assert_eq!(unsorted_char_vec, DataProvider::get_sorted());
    }

    fn test_executor<F, T>(f: F, data: &mut [T]) -> ()
    where
        F: Fn(&mut [T]) -> &mut [T],
        T: Ord,
    {
        f(data);
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
            vec!['A', 'B', 'a', 'j', 'z', 'z', '🔥']
        }

        fn get_sorted() -> Vec<char> {
            vec!['A', 'B', 'a', 'j', 'z', 'z', '🔥']
        }
    }
}
