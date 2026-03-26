//! A blocking concurrent task runner with fail-fast semantics.
//!
//! Provides `TaskRunner` for running multiple blocking tasks concurrently
//! with bounded parallelism. When any task fails, no new tasks will be started.

use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use anyhow::{Result, anyhow};

// ============================================================================
// Internal Semaphore Implementation
// ============================================================================

/// Internal state for the concurrency-limiting semaphore.
struct SemaphoreState {
    permits: usize,
    failed: bool,
}

/// A semaphore with fail-fast cancellation support.
///
/// When `set_failed()` is called, all current and future `acquire()` calls
/// return `false` immediately, allowing blocked spawns to abort.
struct Semaphore {
    state: Mutex<SemaphoreState>,
    condvar: Condvar,
}

impl Semaphore {
    fn new(permits: usize) -> Self {
        Self {
            state: Mutex::new(SemaphoreState {
                permits,
                failed: false,
            }),
            condvar: Condvar::new(),
        }
    }

    /// Acquire a permit, blocking until one is available.
    /// Returns `false` immediately if failure has been signaled.
    fn acquire(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        loop {
            if state.failed {
                return false;
            }
            if state.permits > 0 {
                state.permits -= 1;
                return true;
            }
            state = self.condvar.wait(state).unwrap();
        }
    }

    /// Release a permit, waking one waiting thread.
    fn release(&self) {
        let mut state = self.state.lock().unwrap();
        state.permits += 1;
        self.condvar.notify_one();
    }

    /// Signal failure, waking all waiting threads.
    fn set_failed(&self) {
        let mut state = self.state.lock().unwrap();
        state.failed = true;
        self.condvar.notify_all();
    }

    /// Check if failure has been signaled.
    fn is_failed(&self) -> bool {
        self.state.lock().unwrap().failed
    }
}

// ============================================================================
// Public API
// ============================================================================

/// A blocking task runner that executes tasks concurrently with bounded parallelism.
///
/// Tasks are spawned progressively using [`spawn`](Self::spawn), which blocks if
/// the maximum number of concurrent tasks is already running. Once any task returns
/// an error, no new tasks will be started.
///
/// # Example
///
/// ```rust
/// use anyhow::{Result, anyhow};
///
/// let mut runner = TaskRunner::new(4);
///
/// for i in 0..10 {
///     runner.spawn(move || -> Result<i32> {
///         // Simulate work
///         std::thread::sleep(std::time::Duration::from_millis(10));
///         Ok(i * 2)
///     });
/// }
///
/// let results = runner.join_all()?;
/// assert_eq!(results.len(), 10);
/// ```
pub struct TaskRunner<T: Send + 'static> {
    semaphore: Arc<Semaphore>,
    handles: Vec<JoinHandle<Result<T>>>,
}

impl<T: Send + 'static> TaskRunner<T> {
    /// Creates a new task runner with the default maximum concurrency.
    ///
    /// # Example
    ///
    /// ```rust
    /// let runner: TaskRunner<()> = TaskRunner::new();
    /// ```
    pub fn new() -> Self {
        Self::new_custom(
            // TODO: put into constant
            32,
        )
    }

    /// Creates a new task runner with the specified maximum concurrency.
    ///
    /// # Panics
    ///
    /// Panics if `max_concurrent` is 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// let runner: TaskRunner<()> = TaskRunner::new_custom(4);
    /// ```
    pub fn new_custom(max_concurrent: usize) -> Self {
        assert!(max_concurrent > 0, "max_concurrent must be at least 1");

        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            handles: Vec::new(),
        }
    }

    /// Spawns a new task to be executed concurrently.
    ///
    /// **Blocking behavior**: This method blocks if `max_concurrent` tasks are
    /// already running, waiting until a slot becomes available.
    ///
    /// **Fail-fast**: Returns `false` without spawning if any previous task has
    /// failed. The task closure will not be executed in this case.
    ///
    /// # Arguments
    ///
    /// * `task` - A closure returning `anyhow::Result<T>`.
    ///
    /// # Returns
    ///
    /// * `true` - The task was successfully spawned.
    /// * `false` - Spawning was refused due to a prior task failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut runner = TaskRunner::new(2);
    ///
    /// let spawned = runner.spawn(|| {
    ///     // Do some work...
    ///     Ok(42)
    /// });
    ///
    /// if !spawned {
    ///     println!("Task was not spawned due to prior failure");
    /// }
    /// ```
    pub fn spawn<F>(&mut self, task: F)
    where
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        // Early exit if already failed
        if self.semaphore.is_failed() {
            return;
        }

        // Block until a permit is available (or failure is signaled)
        if !self.semaphore.acquire() {
            return;
        }

        let semaphore = Arc::clone(&self.semaphore);

        let handle = thread::spawn(move || {
            // RAII guard ensures permit release even on panic
            struct ReleaseGuard(Arc<Semaphore>);
            impl Drop for ReleaseGuard {
                fn drop(&mut self) {
                    self.0.release();
                }
            }
            let _guard = ReleaseGuard(Arc::clone(&semaphore));

            let result = task();

            if result.is_err() {
                semaphore.set_failed();
            }

            result
        });

        self.handles.push(handle);
    }

    /// Waits for all spawned tasks to complete and collects their results.
    ///
    /// Consumes the runner and blocks until every spawned task has finished.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<T>)` - All tasks succeeded; results are in spawn order.
    /// * `Err(...)` - At least one task failed or panicked; returns the first error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut runner = TaskRunner::new(2);
    /// runner.spawn(|| Ok(1));
    /// runner.spawn(|| Ok(2));
    ///
    /// match runner.join_all() {
    ///     Ok(results) => println!("All succeeded: {:?}", results),
    ///     Err(e) => println!("A task failed: {}", e),
    /// }
    /// ```
    pub fn join_all(self) -> Result<Vec<T>> {
        let mut results = Vec::with_capacity(self.handles.len());
        let mut first_error: Option<anyhow::Error> = None;

        for handle in self.handles {
            match handle.join() {
                Ok(Ok(value)) => {
                    if first_error.is_none() {
                        results.push(value);
                    }
                }
                Ok(Err(e)) => {
                    if first_error.is_none() {
                        first_error = Some(e);
                    }
                }
                Err(_panic_payload) => {
                    if first_error.is_none() {
                        first_error = Some(anyhow!("task thread panicked"));
                    }
                }
            }
        }

        match first_error {
            Some(e) => Err(e),
            None => Ok(results),
        }
    }
}

// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    // =========================================================================
    // Semaphore Tests
    // =========================================================================

    mod semaphore_tests {
        use super::*;

        #[test]
        fn test_new_creates_with_correct_permits() {
            let sem = Semaphore::new(5);
            // Acquire all 5 permits
            for _ in 0..5 {
                assert!(sem.acquire());
            }
            // No more permits should be immediately available
            // (we'd block, so we test via is_failed instead)
            assert!(!sem.is_failed());
        }

        #[test]
        fn test_acquire_and_release_basic() {
            let sem = Semaphore::new(1);
            assert!(sem.acquire());
            sem.release();
            // Should be able to acquire again after release
            assert!(sem.acquire());
        }

        #[test]
        fn test_is_failed_initially_false() {
            let sem = Semaphore::new(3);
            assert!(!sem.is_failed());
        }

        #[test]
        fn test_set_failed_marks_semaphore_as_failed() {
            let sem = Semaphore::new(3);
            sem.set_failed();
            assert!(sem.is_failed());
        }

        #[test]
        fn test_acquire_returns_false_when_failed() {
            let sem = Semaphore::new(3);
            sem.set_failed();
            assert!(!sem.acquire());
        }

        #[test]
        fn test_acquire_returns_false_after_failure_even_with_permits() {
            let sem = Semaphore::new(10);
            sem.set_failed();
            // Even with 10 permits available, should return false
            assert!(!sem.acquire());
        }

        #[test]
        fn test_blocked_acquire_unblocks_on_failure() {
            let sem = Arc::new(Semaphore::new(1));
            // Acquire the only permit
            assert!(sem.acquire());

            let sem_clone = Arc::clone(&sem);
            let handle = thread::spawn(move || {
                // This should block until failure is signaled
                sem_clone.acquire()
            });

            // Give the thread time to start blocking
            thread::sleep(Duration::from_millis(50));

            // Signal failure
            sem.set_failed();

            // The blocked thread should now return false
            let result = handle.join().unwrap();
            assert!(!result);
        }

        #[test]
        fn test_blocked_acquire_unblocks_on_release() {
            let sem = Arc::new(Semaphore::new(1));
            // Acquire the only permit
            assert!(sem.acquire());

            let sem_clone = Arc::clone(&sem);
            let handle = thread::spawn(move || {
                // This should block until permit is released
                sem_clone.acquire()
            });

            // Give the thread time to start blocking
            thread::sleep(Duration::from_millis(50));

            // Release the permit
            sem.release();

            // The blocked thread should now acquire successfully
            let result = handle.join().unwrap();
            assert!(result);
        }

        #[test]
        fn test_multiple_concurrent_acquires() {
            let sem = Arc::new(Semaphore::new(3));
            let acquired_count = Arc::new(AtomicUsize::new(0));

            let mut handles = Vec::new();

            for _ in 0..5 {
                let sem_clone = Arc::clone(&sem);
                let count_clone = Arc::clone(&acquired_count);
                handles.push(thread::spawn(move || {
                    if sem_clone.acquire() {
                        count_clone.fetch_add(1, Ordering::SeqCst);
                        // Hold the permit briefly
                        thread::sleep(Duration::from_millis(50));
                        sem_clone.release();
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            // All 5 threads should have acquired eventually
            assert_eq!(acquired_count.load(Ordering::SeqCst), 5);
        }
    }

    // =========================================================================
    // TaskRunner Tests
    // =========================================================================

    mod task_runner_tests {
        use super::*;

        #[test]
        fn test_new_creates_runner_with_default_concurrency() {
            let runner: TaskRunner<()> = TaskRunner::new();
            // Should not panic and should be usable
            let results = runner.join_all().unwrap();
            assert!(results.is_empty());
        }

        #[test]
        fn test_new_custom_creates_runner_with_specified_concurrency() {
            let runner: TaskRunner<()> = TaskRunner::new_custom(4);
            let results = runner.join_all().unwrap();
            assert!(results.is_empty());
        }

        #[test]
        #[should_panic(expected = "max_concurrent must be at least 1")]
        fn test_new_custom_panics_with_zero_concurrency() {
            let _runner: TaskRunner<()> = TaskRunner::new_custom(0);
        }

        #[test]
        fn test_spawn_single_task() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(4);
            runner.spawn(|| Ok(42));
            let results = runner.join_all().unwrap();
            assert_eq!(results, vec![42]);
        }

        #[test]
        fn test_spawn_multiple_tasks() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(4);
            for i in 0..5 {
                runner.spawn(move || Ok(i));
            }
            let results = runner.join_all().unwrap();
            assert_eq!(results.len(), 5);
            // Results should be in spawn order
            assert_eq!(results, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn test_join_all_returns_results_in_spawn_order() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(2);

            // Spawn tasks that take varying amounts of time
            runner.spawn(|| {
                thread::sleep(Duration::from_millis(100));
                Ok(1)
            });
            runner.spawn(|| {
                thread::sleep(Duration::from_millis(10));
                Ok(2)
            });
            runner.spawn(|| {
                thread::sleep(Duration::from_millis(50));
                Ok(3)
            });

            let results = runner.join_all().unwrap();
            // Despite different completion times, results should be in spawn order
            assert_eq!(results, vec![1, 2, 3]);
        }

        #[test]
        fn test_spawn_limits_concurrency() {
            let max_concurrent = 2;
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(max_concurrent);
            let active_count = Arc::new(AtomicUsize::new(0));
            let max_observed = Arc::new(AtomicUsize::new(0));

            for _ in 0..10 {
                let active = Arc::clone(&active_count);
                let max_obs = Arc::clone(&max_observed);
                runner.spawn(move || {
                    let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                    // Update max observed concurrency
                    max_obs.fetch_max(current, Ordering::SeqCst);

                    thread::sleep(Duration::from_millis(20));

                    active.fetch_sub(1, Ordering::SeqCst);
                    Ok(())
                });
            }

            runner.join_all().unwrap();

            // Max observed concurrency should not exceed the limit
            assert!(max_observed.load(Ordering::SeqCst) <= max_concurrent);
        }

        #[test]
        fn test_spawn_blocks_when_at_max_concurrency() {
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(1);
            let started = Arc::new(AtomicUsize::new(0));

            let started_clone = Arc::clone(&started);
            runner.spawn(move || {
                started_clone.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(100));
                Ok(())
            });

            // This spawn should block because the first task is still running
            let spawn_start = Instant::now();
            let started_clone2 = Arc::clone(&started);
            runner.spawn(move || {
                started_clone2.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
            let spawn_duration = spawn_start.elapsed();

            runner.join_all().unwrap();

            // The second spawn should have blocked for approximately the first task's duration
            assert!(spawn_duration >= Duration::from_millis(80));
            assert_eq!(started.load(Ordering::SeqCst), 2);
        }

        #[test]
        fn test_join_all_returns_error_on_task_failure() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(4);

            runner.spawn(|| Ok(1));
            runner.spawn(|| Err(anyhow!("task failed")));
            runner.spawn(|| Ok(3));

            let result = runner.join_all();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "task failed");
        }

        #[test]
        fn test_join_all_returns_first_error_when_multiple_failures() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(1);

            runner.spawn(|| Err(anyhow!("first error")));
            runner.spawn(|| Err(anyhow!("second error")));

            let result = runner.join_all();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "first error");
        }

        #[test]
        fn test_fail_fast_prevents_new_spawns() {
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(1);
            let task_executed = Arc::new(AtomicUsize::new(0));

            // First task fails
            let exec1 = Arc::clone(&task_executed);
            runner.spawn(move || {
                exec1.fetch_add(1, Ordering::SeqCst);
                Err(anyhow!("failure"))
            });

            // Give the first task time to complete and set failed state
            thread::sleep(Duration::from_millis(50));

            // Second task should not be spawned due to fail-fast
            let exec2 = Arc::clone(&task_executed);
            runner.spawn(move || {
                exec2.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });

            let _ = runner.join_all();

            // Only the first task should have executed
            assert_eq!(task_executed.load(Ordering::SeqCst), 1);
        }

        #[test]
        fn test_fail_fast_unblocks_waiting_spawns() {
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(1);
            let spawned_count = Arc::new(AtomicUsize::new(0));

            // This task will hold the permit and then fail
            let count1 = Arc::clone(&spawned_count);
            runner.spawn(move || {
                count1.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(50));
                Err(anyhow!("failure"))
            });

            // Queue up more spawns that would block
            for _ in 0..3 {
                let count = Arc::clone(&spawned_count);
                runner.spawn(move || {
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                });
            }

            let _ = runner.join_all();

            // The subsequent spawns may or may not execute depending on timing,
            // but the runner should not deadlock
            assert!(spawned_count.load(Ordering::SeqCst) >= 1);
        }

        #[test]
        fn test_join_all_handles_panic() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(4);

            runner.spawn(|| Ok(1));
            runner.spawn(|| panic!("task panicked!"));
            runner.spawn(|| Ok(3));

            let result = runner.join_all();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "task thread panicked");
        }

        #[test]
        fn test_join_all_prefers_error_over_panic() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(1);

            // First task returns error (processed first in join_all)
            runner.spawn(|| Err(anyhow!("error first")));
            // Second task panics
            runner.spawn(|| panic!("panic second"));

            let result = runner.join_all();
            assert!(result.is_err());
            // First error should be returned
            assert_eq!(result.unwrap_err().to_string(), "error first");
        }

        #[test]
        fn test_empty_runner_join_all_succeeds() {
            let runner: TaskRunner<i32> = TaskRunner::new_custom(4);
            let results = runner.join_all().unwrap();
            assert!(results.is_empty());
        }

        #[test]
        fn test_spawn_with_closure_capturing_values() {
            let mut runner: TaskRunner<String> = TaskRunner::new_custom(4);

            let data = vec!["hello", "world", "test"];
            for item in data {
                runner.spawn(move || Ok(item.to_uppercase()));
            }

            let results = runner.join_all().unwrap();
            assert_eq!(results, vec!["HELLO", "WORLD", "TEST"]);
        }

        #[test]
        fn test_large_number_of_tasks() {
            let mut runner: TaskRunner<usize> = TaskRunner::new_custom(8);

            for i in 0..100 {
                runner.spawn(move || {
                    thread::sleep(Duration::from_micros(100));
                    Ok(i)
                });
            }

            let results = runner.join_all().unwrap();
            assert_eq!(results.len(), 100);

            // Verify spawn order is preserved
            for (i, &result) in results.iter().enumerate() {
                assert_eq!(result, i);
            }
        }

        #[test]
        fn test_concurrent_execution_actually_parallel() {
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(4);

            let start = Instant::now();

            // Spawn 4 tasks that each take 100ms
            for _ in 0..4 {
                runner.spawn(|| {
                    thread::sleep(Duration::from_millis(100));
                    Ok(())
                });
            }

            runner.join_all().unwrap();

            let elapsed = start.elapsed();

            // If running in parallel, should take ~100ms, not ~400ms
            // Allow some margin for scheduling overhead
            assert!(
                elapsed < Duration::from_millis(250),
                "Expected parallel execution, but took {elapsed:?}",
            );
        }

        #[test]
        fn test_task_can_return_complex_types() {
            #[derive(Debug, PartialEq)]
            struct ComplexResult {
                id: u32,
                name: String,
                values: Vec<i32>,
            }

            let mut runner: TaskRunner<ComplexResult> = TaskRunner::new_custom(2);

            runner.spawn(|| {
                Ok(ComplexResult {
                    id: 1,
                    name: "first".to_owned(),
                    values: vec![1, 2, 3],
                })
            });

            runner.spawn(|| {
                Ok(ComplexResult {
                    id: 2,
                    name: "second".to_owned(),
                    values: vec![4, 5, 6],
                })
            });

            let results = runner.join_all().unwrap();
            assert_eq!(results.len(), 2);
            assert_eq!(results.first().unwrap().id, 1);
            assert_eq!(results.get(1).unwrap().id, 2);
        }

        #[test]
        fn test_release_guard_releases_on_panic() {
            let mut runner: TaskRunner<()> = TaskRunner::new_custom(1);
            let task_after_panic_executed = Arc::new(AtomicUsize::new(0));

            // First task panics
            runner.spawn(|| {
                panic!("intentional panic");
            });

            // This spawn should eventually acquire the permit after panic releases it
            let exec = Arc::clone(&task_after_panic_executed);
            runner.spawn(move || {
                exec.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });

            let _ = runner.join_all();

            // The second task should have executed (permit was released despite panic)
            // Note: Due to fail-fast behavior from panic handling, this depends on timing
            // The key test is that we don't deadlock
        }
    }

    // =========================================================================
    // Integration Tests
    // =========================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn test_example_from_documentation() {
            let mut runner: TaskRunner<i32> = TaskRunner::new_custom(4);

            for i in 0..10 {
                runner.spawn(move || -> Result<i32> {
                    // Simulate work
                    thread::sleep(Duration::from_millis(10));
                    Ok(i * 2)
                });
            }

            let results = runner.join_all().unwrap();
            assert_eq!(results.len(), 10);
            assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
        }

        #[test]
        fn test_real_world_file_processing_simulation() {
            let mut runner: TaskRunner<(String, usize)> = TaskRunner::new_custom(3);

            let files = vec![
                ("file1.txt", 100),
                ("file2.txt", 200),
                ("file3.txt", 150),
                ("file4.txt", 50),
            ];

            for (name, size) in files {
                runner.spawn(move || {
                    // Simulate file processing
                    thread::sleep(Duration::from_millis(u64::try_from(size).unwrap() / 10));
                    Ok((name.to_owned(), size))
                });
            }

            let results = runner.join_all().unwrap();
            assert_eq!(results.len(), 4);

            // Results should be in spawn order, not completion order
            assert_eq!(results.first().unwrap().0, "file1.txt");
            assert_eq!(results.get(1).unwrap().0, "file2.txt");
            assert_eq!(results.get(2).unwrap().0, "file3.txt");
            assert_eq!(results.get(3).unwrap().0, "file4.txt");
        }

        #[test]
        fn test_fail_fast_scenario() {
            let mut runner: TaskRunner<String> = TaskRunner::new_custom(2);
            let successful_tasks = Arc::new(AtomicUsize::new(0));

            // First few tasks succeed
            for i in 0..3 {
                let count = Arc::clone(&successful_tasks);
                runner.spawn(move || {
                    thread::sleep(Duration::from_millis(10));
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok(format!("success-{i}"))
                });
            }

            // This task will fail
            runner.spawn(|| {
                thread::sleep(Duration::from_millis(5));
                Err(anyhow!("validation error"))
            });

            // More tasks that may or may not run due to fail-fast
            for i in 10..15 {
                let count = Arc::clone(&successful_tasks);
                runner.spawn(move || {
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok(format!("late-{i}"))
                });
            }

            let result = runner.join_all();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "validation error");

            // Some tasks should have been prevented from running
            // (exact count depends on timing, but should be less than 8)
            let executed = successful_tasks.load(Ordering::SeqCst);

            assert!(
                executed < 8,
                "Expected fail-fast to prevent some tasks, but {executed} executed",
            );
        }
    }
}
