use std::thread::JoinHandle;

#[derive(Debug)]
pub struct ThreadPool<T> {
    pub workers: Vec<JoinHandle<T>>,
    pub thread_count: usize,
    pub active_threads: usize,
}

impl<T> ThreadPool<T> {
    pub fn new() -> Self {
        Self {
            workers: vec![],
            // thread_count: usize::min(
            //     8,
            //     sys_info::cpu_num().map_or(4, |count| {
            //         if count > 4 {
            //             return (count - 2) as usize;
            //         }
            //         count as usize
            //     }),
            // ),
            thread_count: 4,
            active_threads: 0,
        }
    }
}
