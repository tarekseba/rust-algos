use std::{cmp::Ordering, slice};

pub enum Sorting {
    Ascending,
    Descending,
}

impl Sorting {
    fn flip(&self) -> Sorting {
        match &self {
            Sorting::Ascending => Sorting::Descending,
            Sorting::Descending => Sorting::Ascending,
        }
    }
}

impl From<&Sorting> for Ordering {
    fn from(value: &Sorting) -> Self {
        match value {
            Sorting::Descending => Self::Less,
            _ => Self::Greater,
        }
    }
}

pub fn vec_from_raw_ptr<T>(ptr: usize, len: usize) -> ()
where
    T: Ord + std::fmt::Debug,
{
    let ptr = ptr as *mut T;
    let slice_data = unsafe { slice::from_raw_parts_mut(ptr, len) };
    println!("{:?}", slice_data);
}

fn split<'a, T>(slice: &'a mut [T], index: usize) -> (&'a mut [T], &'a mut [T])
where
    T: Ord + std::fmt::Debug,
{
    let length: usize = slice.len();
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, index),
            std::slice::from_raw_parts_mut(ptr.add(index + 1), length - (index + 1)),
        )
    }
}

fn sort<T>(slice: &mut [T], sorting: &Sorting) -> usize
where
    T: Ord + std::fmt::Debug,
{
    let left_order = Ordering::from(sorting);
    let index = (slice.len() - 1) / 2;
    let right_order = Ordering::from(&sorting.flip());
    slice.swap(index, slice.len() - 1);
    let mut i = 0;
    let mut j = slice.len() - 1;
    while i + 1 <= j {
        while i < slice.len() - 1 && slice[i].cmp(&slice[slice.len() - 1]) != left_order {
            i += 1;
        }
        while j > 0 && slice[j - 1].cmp(&slice[slice.len() - 1]) != right_order {
            j -= 1;
        }
        if let Some(x) = j.checked_sub(1) {
            if i < x {
                slice.swap(i, x);
                i += 1;
                j -= 1;
            }
        } else {
            break;
        }
    }
    slice.swap(i, slice.len() - 1);
    i
}

pub fn quick_sort<T: Ord + std::fmt::Debug>(slice: &mut [T], order: &Sorting) -> () {
    if slice.len() >= 2 {
        let index = sort(slice, order);
        let (first_half, second_half): (&mut [T], &mut [T]) = split(slice, index);
        quick_sort(first_half, order);
        quick_sort(second_half, order);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_executor, DataProvider, TestUtils};

    use super::{quick_sort, Sorting};

    fn quick_sort_wrapper<T: Ord + std::fmt::Debug>(
        slice: &mut [T],
        dummy_for_now: Sorting,
    ) -> &mut [T] {
        quick_sort(slice, &dummy_for_now);
        slice
    }

    #[test]
    fn quick_sort_test() {
        let mut unsorted_vec: Vec<i32> = DataProvider::get();
        let mut unsorted_char_vec: Vec<char> = DataProvider::get();

        test_executor(quick_sort_wrapper, &mut unsorted_vec, Sorting::Ascending);
        assert_eq!(unsorted_vec, DataProvider::get_sorted());
        test_executor(
            quick_sort_wrapper,
            &mut unsorted_char_vec,
            Sorting::Ascending,
        );
        assert_eq!(unsorted_char_vec, DataProvider::get_sorted());
    }
}
