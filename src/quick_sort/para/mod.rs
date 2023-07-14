use std::thread::JoinHandle;

use crate::utils::Sorting;

use super::mono::{sort, split};

trait IsTrue {
    fn is_true(&self) -> bool;
}
impl IsTrue for (bool, bool) {
    fn is_true(&self) -> bool {
        self.0 && self.1
    }
}

fn spawn_thread<T>(
    slice: &mut [T],
    threads: &mut Vec<JoinHandle<()>>,
    counter: &mut u32,
    sorting: Sorting,
) where
    T: Ord + std::fmt::Debug,
{
    *counter += 1;
    let ptr: usize = slice.as_mut_ptr() as usize;
    let length = slice.len();
    let cloned_sorting = sorting.clone();
    threads.push(std::thread::spawn(move || {
        qs_para::<T>(ptr, length, cloned_sorting);
        ()
    }));
}

pub fn quick_sort_aux<T>(
    array: &mut [T],
    threads: &mut Vec<JoinHandle<()>>,
    counter: &mut u32,
    sorting: &Sorting,
) where
    T: Ord + std::fmt::Debug,
{
    let mut _break = (false, false);
    while array.len() > 1 && *counter < 8 && !_break.is_true() {
        let index = sort(array, sorting);
        let (first_half, second_half): (&mut [T], &mut [T]) = split(array, index);
        if first_half.len() > 5 && *counter < 8 {
            spawn_thread(first_half, threads, counter, sorting.clone())
        } else {
            _break.0 = true;
            quick_sort_aux(first_half, threads, counter, sorting)
        }

        if second_half.len() > 5 && *counter < 8 {
            spawn_thread(second_half, threads, counter, sorting.clone())
        } else {
            _break.1 = true;
            quick_sort_aux(second_half, threads, counter, sorting)
        }
    }
}

pub fn quick_sort<T>(array: &mut [T], sorting: &Sorting)
where
    T: Ord + std::fmt::Debug,
{
    let mut threads: Vec<JoinHandle<()>> = vec![];
    let mut counter: u32 = 0;

    quick_sort_aux(array, &mut threads, &mut counter, sorting);

    for handle in threads {
        let _ = handle.join();
    }
}

fn qs_para<T>(ptr: usize, len: usize, sorting: Sorting)
where
    T: Ord + std::fmt::Debug,
{
    let ptr = ptr as *mut T;
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    qs_para_aux(slice, &sorting);
}

fn qs_para_aux<T>(slice: &mut [T], sorting: &Sorting)
where
    T: Ord + std::fmt::Debug,
{
    if slice.len() > 1 {
        let index = sort(slice, sorting);
        let (first_half, second_half) = split(slice, index);
        qs_para_aux(first_half, sorting);
        qs_para_aux(second_half, sorting);
    }
}
