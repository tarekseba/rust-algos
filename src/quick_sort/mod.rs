use std::cmp::Ordering;

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

fn sort<T>(slice: &mut [T]) -> usize
where
    T: Ord + std::fmt::Debug,
{
    let index = (slice.len() - 1) / 2;
    slice.swap(index, slice.len() - 1);
    let mut i = 0;
    let mut j = slice.len() - 1;
    while i + 1 <= j {
        while i < slice.len() - 1 && slice[i].cmp(&slice[slice.len() - 1]) != Ordering::Greater {
            i += 1;
        }
        while j > 0 && slice[j - 1].cmp(&slice[slice.len() - 1]) != Ordering::Less {
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

pub fn quick_sort<T: Ord + std::fmt::Debug>(slice: &mut [T]) -> () {
    if slice.len() >= 2 {
        let index = sort(slice);
        let (first_half, second_half): (&mut [T], &mut [T]) = split(slice, index);
        quick_sort(first_half);
        quick_sort(second_half);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_executor, DataProvider, TestUtils};

    use super::{quick_sort, Ordering};

    //TODO: Add Ordering to quick_sort function
    fn quick_sort_wrapper<T: Ord + std::fmt::Debug>(
        slice: &mut [T],
        dummy_for_now: Ordering,
    ) -> &mut [T] {
        quick_sort(slice);
        slice
    }

    #[test]
    fn quick_sort_test() {
        let mut unsorted_vec: Vec<i32> = DataProvider::get();
        let mut unsorted_char_vec: Vec<char> = DataProvider::get();

        test_executor(quick_sort_wrapper, &mut unsorted_vec, Ordering::Greater);
        assert_eq!(unsorted_vec, DataProvider::get_sorted());
        test_executor(
            quick_sort_wrapper,
            &mut unsorted_char_vec,
            Ordering::Greater,
        );
        assert_eq!(unsorted_char_vec, DataProvider::get_sorted());
    }
}
