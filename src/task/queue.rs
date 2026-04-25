//! Task queue
//!
//! This module provides a priority-based task queue for scheduling operations.
//! Uses a `BinaryHeap` for O(log n) insertion and a `Semaphore` for concurrency limiting.

use crate::error::{Result, SerialError};
use crate::task::Task;
use std::collections::BinaryHeap;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Priority level for queued tasks.
///
/// Higher priority tasks are dequeued before lower priority ones.
/// Within the same priority level, tasks are served in FIFO order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    /// Lowest priority — background or non-urgent work.
    Low = 0,
    /// Default priority for most tasks.
    Normal = 1,
    /// Elevated priority — time-sensitive work.
    High = 2,
    /// Highest priority — critical or blocking operations.
    Critical = 3,
}

/// A priority-based task queue with concurrency limiting.
///
/// Tasks are ordered by [`TaskPriority`] (highest first), with FIFO ordering
/// within the same priority level. A semaphore limits the number of tasks
/// that can execute concurrently.
pub struct TaskQueue {
    /// Priority queue of pending tasks
    queue: Arc<Mutex<BinaryHeap<TaskEntry>>>,
    /// Semaphore for limiting concurrent tasks
    semaphore: Arc<Semaphore>,
    /// Maximum concurrent tasks
    max_concurrent: usize,
}

/// Task entry in the queue (wraps Task with priority)
#[derive(Debug, Clone)]
struct TaskEntry {
    task: Task,
    priority: TaskPriority,
    order: u64, // For FIFO ordering within same priority
}

impl PartialEq for TaskEntry {
    fn eq(&self, other: &Self) -> bool {
        self.task.id() == other.task.id()
    }
}

impl Eq for TaskEntry {}

impl PartialOrd for TaskEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaskEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare by priority first (reversed for max-heap)
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => {
                // Then by order (FIFO)
                self.order.cmp(&other.order)
            }
            other => other,
        }
    }
}

impl TaskQueue {
    /// Create a new task queue with the given maximum concurrency limit.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
        }
    }

    /// Push a task onto the queue with the given priority.
    ///
    /// Tasks with higher priority are dequeued first. Within the same
    /// priority, insertion order is preserved (FIFO).
    pub async fn push(&self, task: Task, priority: TaskPriority) -> Result<()> {
        let counter = self.order_counter();

        let entry = TaskEntry {
            task,
            priority,
            order: counter,
        };

        let mut queue = self.queue.lock().await;
        queue.push(entry);

        tracing::debug!("Task added to queue with priority {:?}", priority);
        Ok(())
    }

    /// Remove and return the highest-priority task from the queue.
    ///
    /// Returns `None` if the queue is empty.
    pub async fn pop(&self) -> Option<Task> {
        let mut queue = self.queue.lock().await;
        queue.pop().map(|entry| entry.task)
    }

    /// Get the number of pending tasks in the queue.
    pub async fn len(&self) -> usize {
        let queue = self.queue.lock().await;
        queue.len()
    }

    /// Returns `true` if the queue has no pending tasks.
    pub async fn is_empty(&self) -> bool {
        let queue = self.queue.lock().await;
        queue.is_empty()
    }

    /// Wait for and acquire a concurrency permit. The permit is released
    /// when the returned `SemaphorePermit` is dropped.
    pub async fn acquire_permit(&self) -> Result<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore
            .acquire()
            .await
            .map_err(|e| SerialError::Io(std::io::Error::other(e)))
    }

    /// Get the configured maximum number of concurrent tasks.
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }

    /// Simple order counter
    fn order_counter(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::TaskType;

    #[tokio::test]
    async fn test_queue_creation() {
        let queue = TaskQueue::new(10);
        assert_eq!(queue.max_concurrent(), 10);
        let is_empty = queue.is_empty().await;
        assert!(is_empty);
    }

    #[tokio::test]
    async fn test_push_pop() {
        let queue = TaskQueue::new(10);

        let task = Task::new(TaskType::Script {
            name: "test".to_string(),
            content: "print('test')".to_string(),
        });

        queue
            .push(task.clone(), TaskPriority::Normal)
            .await
            .unwrap();
        assert_eq!(queue.len().await, 1);

        let retrieved = queue.pop().await.unwrap();
        assert_eq!(retrieved.id(), task.id());
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let queue = TaskQueue::new(10);

        let task1 = Task::new(TaskType::Script {
            name: "low".to_string(),
            content: "".to_string(),
        });
        let task2 = Task::new(TaskType::Script {
            name: "high".to_string(),
            content: "".to_string(),
        });

        // Add low priority first
        queue.push(task1.clone(), TaskPriority::Low).await.unwrap();
        // Add high priority second
        queue.push(task2.clone(), TaskPriority::High).await.unwrap();

        // High priority should come out first
        let first = queue.pop().await.unwrap();
        assert_eq!(first.id(), task2.id());

        let second = queue.pop().await.unwrap();
        assert_eq!(second.id(), task1.id());
    }

    #[tokio::test]
    async fn test_pop_empty_queue() {
        let queue: TaskQueue = TaskQueue::new(10);
        let result = queue.pop().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_push_multiple_pop_all() {
        let queue = TaskQueue::new(10);

        for i in 0..5 {
            let task = Task::new(TaskType::Script {
                name: format!("task_{}", i),
                content: "".to_string(),
            });
            queue.push(task, TaskPriority::Normal).await.unwrap();
        }

        assert_eq!(queue.len().await, 5);
        assert!(!queue.is_empty().await);

        // Pop all tasks
        for _ in 0..5 {
            assert!(queue.pop().await.is_some());
        }

        // Queue should now be empty
        assert!(queue.is_empty().await);
        assert!(queue.pop().await.is_none());
    }
}
