mod bubble_sort;
mod quick_sort;
mod test_utils;
mod utils;

use quick_sort::quick_sort;
use sys_info;

use crate::utils::Sorting;

fn main() {
    let _cpu_count = sys_info::cpu_num().unwrap_or(4);
    let mut unsorted_vec = vec![-100, 5, 1, 3, 10, 2, -20, 20, -2, 4, 2];

    let x = quick_sort(&mut unsorted_vec, &Sorting::Ascending);
    println!("{:?} {:?}", x, unsorted_vec);
    let j: usize = 1;
    if let Some(x) = j.checked_sub(1) {
        println!("{}", x)
    }

    let mut my_string_vec = vec![
        "tarek", "is", "not", "going", "anywhere", "what", "the", "hell", "are", "you", "saying",
    ];
    let ptr = my_string_vec.as_mut_ptr();
    quick_sort::vec_from_raw_ptr::<&str>(ptr as usize, 5)
}
