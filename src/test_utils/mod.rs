use crate::quick_sort::Sorting;

pub trait TestUtils<T> {
    fn get() -> Vec<T>;
    fn get_sorted() -> Vec<T>;
}

pub struct DataProvider {}

impl TestUtils<i32> for DataProvider {
    fn get() -> Vec<i32> {
        vec![5, 2, 3, 10, 1, -20, 20, -2, 4, 2]
    }
    fn get_sorted() -> Vec<i32> {
        vec![-20, -2, 1, 2, 2, 3, 4, 5, 10, 20]
    }
}

impl TestUtils<char> for DataProvider {
    fn get() -> Vec<char> {
        vec!['a', '🔥', 'B', 'A', 'Z', 'c', 'z']
    }

    fn get_sorted() -> Vec<char> {
        vec!['A', 'B', 'Z', 'a', 'c', 'z', '🔥']
    }
}

pub fn test_executor<F, T>(f: F, data: &mut [T], order: Sorting) -> ()
where
    F: Fn(&mut [T], Sorting) -> &mut [T],
    T: Ord,
{
    f(data, order);
}
