use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // something
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static,
    {
        // We still use the () after FnOnce because this FnOnce represents
        // a closure that takes no parameters and returns the unit type ()
    }
}