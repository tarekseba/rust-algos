mod bubble_sort;
mod quick_sort;
mod test_utils;
mod utils;

use std::time::Instant;

use quick_sort::para::quick_sort as qs_para;
use quick_sort::mono::quick_sort as qs_mono;
use sys_info;

use crate::utils::Sorting;

fn main() {
    let _cpu_count = sys_info::cpu_num().unwrap_or(4);
    let mut unsorted_vec = vec![-100, 5, 1, 3, 10, 2, -20, 20, -2, 4, 2];
    let now = Instant::now();

    let x = qs_para(&mut unsorted_vec, &Sorting::Ascending);
    println!("elapsed {:?}", now.elapsed());
    println!("{:?} {:?}", x, unsorted_vec);
}
