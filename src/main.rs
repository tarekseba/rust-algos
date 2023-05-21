mod bubble_sort;
use bubble_sort::bubble_sort;

fn main() {
    let mut unsorted_vec = vec![5, 2, 3, 10, 1, -20, 20, -2, 4, 2];
    let unsorted_vec = bubble_sort(&mut unsorted_vec);
    assert_eq!(vec![-20, -2, 1, 2, 2, 3, 4, 5, 10, 20], unsorted_vec)
    let mut unsorted_vec = vec![-100, 5, 2, 3, 10, 1, -20, 20, -2, 4, 2];
    let unsorted_vec = bubble_sort(&mut unsorted_vec, std::cmp::Ordering::Less);
    println!("{:?}", unsorted_vec);
}
