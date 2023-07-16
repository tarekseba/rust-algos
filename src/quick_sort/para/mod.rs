use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::utils::{thread_pools::ThreadPool, Sorting};

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

// ----------------------------------------------------------------------------------

fn create_thread<T>(
    state_tx: Sender<Option<(usize, usize)>>,
    data_rx: Arc<Mutex<Receiver<(usize, usize)>>>,
    active_threads: Arc<RwLock<usize>>,
    thread_count: usize,
    index: usize,
) -> JoinHandle<()>
where
    T: Ord + std::fmt::Debug,
{
    let tx = state_tx.clone();
    thread::spawn(move || {
        qs_tp::<i32>(tx, data_rx, active_threads, thread_count, index);
        ()
    })
}

pub fn qs_tp_controller<T>(slice: &mut [T])
where
    T: Ord + std::fmt::Debug,
{
    let mut tp: ThreadPool<()> = ThreadPool::new();
    let (state_tx, state_rx) = mpsc::channel();
    let (data_tx, data_rx) = mpsc::channel();
    let data_rx_arc = Arc::new(Mutex::new(data_rx));
    let active_threads = Arc::new(RwLock::new(0));
    for t in 0..tp.thread_count {
        tp.workers.push(create_thread::<T>(
            state_tx.clone(),
            Arc::clone(&data_rx_arc),
            Arc::clone(&active_threads),
            tp.thread_count,
            t,
        ));
    }

    let ptr = slice.as_mut_ptr() as usize;
    let _ = data_tx.send((ptr, slice.len()));

    let now = Instant::now();
    loop {
        let received = state_rx.recv().unwrap();
        match received {
            Some((ptr, len)) => {
                // println!("main thread: received slice");
                let mut act_threads = active_threads.write().unwrap();
                *act_threads += 1;
                drop(act_threads);
                match data_tx.send((ptr, len)) {
                    Ok(_) => (),
                    Err(err) => println!("Error sending slice: {:?}", err),
                }
            }
            None => {
                let mut act_threads = active_threads.write().unwrap();
                println!("active threads : {:?}", *act_threads);
                if *act_threads > 1 {
                    *act_threads -= 1;
                } else {
                    *act_threads = 0;
                    println!("Leaving, cya...");
                    break;
                }
            }
        }
    }
    println!("{:?}\n elapsed: {:?}", slice, now.elapsed());
}

fn qs_tp<T>(
    state_tx: Sender<Option<(usize, usize)>>,
    data_rx: Arc<Mutex<Receiver<(usize, usize)>>>,
    active_threads: Arc<RwLock<usize>>,
    thread_count: usize,
    index: usize,
) where
    T: Ord + std::fmt::Debug,
{
    loop {
        let rx_lock = data_rx.lock().unwrap();
        if let Ok((ptr, len)) = rx_lock.recv() {
            // println!("Worker {index} : thread received data | length : {len}");
            drop(rx_lock);
            if len > 1 {
                let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut T, len) };
                let index = sort(slice, &Sorting::Ascending);
                let (first_half, second_half): (&mut [T], &mut [T]) = split(slice, index);
                let slice_len = first_half.len();
                let act_threads = active_threads.read().unwrap();
                if first_half.len() > 20 && thread_count > *act_threads {
                    drop(act_threads);
                    let slice_ptr = first_half.as_mut_ptr() as usize;
                    match state_tx.send(Some((slice_ptr, slice_len))) {
                        Ok(_) => (),
                        Err(err) => println!("error sending slice: {:?}", err),
                    };
                } else {
                    drop(act_threads);
                    qs_tp_aux(first_half, &state_tx, &active_threads, &thread_count);
                }
                qs_tp_aux(second_half, &state_tx, &active_threads, &thread_count);
            }
            state_tx.send(None);
        } else {
            break;
        }
    }
    ()
}

fn qs_tp_aux<T>(
    slice: &mut [T],
    state_tx: &Sender<Option<(usize, usize)>>,
    active_threads: &RwLock<usize>,
    thread_count: &usize,
) where
    T: Ord + std::fmt::Debug,
{
    if (slice.len() > 1) {
        let index = sort(slice, &Sorting::Ascending);
        let (first_half, second_half): (&mut [T], &mut [T]) = split(slice, index);
        let (first_half, second_half) = if first_half.len() > second_half.len() {
            (first_half, second_half)
        } else {
            (second_half, first_half)
        };
        let slice_len = first_half.len();
        let act_threads = active_threads.read().unwrap();
        if first_half.len() > 20 && *thread_count > *act_threads {
            drop(act_threads);
            let slice_ptr = first_half.as_mut_ptr() as usize;
            match state_tx.send(Some((slice_ptr, slice_len))) {
                Ok(_) => (),
                Err(err) => println!("error sending slice: {:?}", err),
            }
        } else {
            drop(act_threads);
            qs_tp_aux(first_half, &state_tx, active_threads, thread_count);
        }
        qs_tp_aux(second_half, &state_tx, active_threads, thread_count);
    }
}
