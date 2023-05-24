use std::thread::JoinHandle;

#[derive(Debug)]
pub struct ThreadPool<T> {
    pub workers: Vec<Worker<T>>,
    pub thread_count: u32,
}

impl<T> ThreadPool<T> {
    pub fn new() -> Self {
        Self {
            workers: vec![],
            thread_count: sys_info::cpu_num().map_or(4, |count| {
                if count > 4 {
                    return count - 2;
                }
                count
            }),
        }
    }
}

#[derive(Debug)]
pub struct Worker<T> {
    handle: JoinHandle<T>,
}

impl<T> Worker<T> {
    pub fn new(handle: JoinHandle<T>) -> Self {
        Self { handle }
    }
}
