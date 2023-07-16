mod bubble_sort;
mod quick_sort;
mod test_utils;
mod utils;

use std::time::Instant;

use quick_sort::mono::quick_sort as qs_mono;
use quick_sort::para::quick_sort as qs_para;
use sys_info;

use crate::quick_sort::para::qs_tp_controller;
use crate::utils::{read_input, Sorting};

fn main() {
    let _cpu_count = sys_info::cpu_num().unwrap_or(4);
    let mut unsorted_vec = read_input::<i32>().unwrap();

    let now = Instant::now();

    // let x = qs_mono(&mut unsorted_vec, &Sorting::Ascending);
    qs_tp_controller(&mut unsorted_vec);

    // println!("elapsed {:?}", now.elapsed());

    // println!("{:?}", unsorted_vec);

}
