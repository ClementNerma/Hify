use std::{
    panic::UnwindSafe,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};

use crate::logging::progress_bar;

pub struct TaskSet<T: Send + 'static> {
    queue: Vec<Box<dyn FnOnce() -> T + Send + UnwindSafe>>,
}

impl<T: Send + 'static> TaskSet<T> {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn add(&mut self, task: impl FnOnce() -> T + Send + UnwindSafe + 'static) {
        self.queue.push(Box::new(task));
    }

    pub fn run(self, opts: TaskSetOptions) -> Vec<Result<T>> {
        if self.queue.is_empty() {
            return vec![];
        }

        let TaskSetOptions {
            max_threads,
            use_progress_bar,
        } = opts;

        let tasks_count = self.queue.len();

        let remaining_tasks = Arc::new(Mutex::new(self.queue));
        let results = Arc::new(Mutex::<Vec<Result<T>>>::new(vec![]));

        let pb = if use_progress_bar {
            Some(progress_bar(tasks_count))
        } else {
            None
        };

        let mut runners = vec![];

        for _ in 0..max_threads
            .unwrap_or_else(|| (num_cpus::get() - 1).max(1))
            .min(tasks_count)
        {
            let remaining_tasks = Arc::clone(&remaining_tasks);
            let results = Arc::clone(&results);
            let pb = pb.clone();

            // TODO: handle panics (+ remove .unwrap() calls)
            runners.push(std::thread::spawn(move || {
                loop {
                    let Some(next_task) = remaining_tasks.lock().unwrap().pop() else {
                        return;
                    };

                    let result = std::panic::catch_unwind(next_task);

                    results.lock().unwrap().push(result.map_err(|_| {
                        // TODO: show error
                        anyhow!("Task panicked")
                    }));

                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
                }
            }));
        }

        for runner in runners {
            runner.join().unwrap();
        }

        if let Some(pb) = pb {
            pb.finish();
        }

        let results = Arc::into_inner(results).unwrap().into_inner().unwrap();
        assert_eq!(results.len(), tasks_count);

        results
    }
}

#[derive(Default)]
pub struct TaskSetOptions {
    pub max_threads: Option<usize>,
    pub use_progress_bar: bool,
}

impl TaskSetOptions {
    pub fn with_progress_bar() -> Self {
        Self {
            max_threads: None,
            use_progress_bar: true,
        }
    }
}
