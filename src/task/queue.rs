//! Task queue
//!
//! This module provides a priority-based task queue for scheduling operations.

use crate::error::{Result, SerialError};
use crate::task::Task;
use std::collections::BinaryHeap;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Task queue
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
    /// Create a new task queue
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
        }
    }

    /// Add a task to the queue
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

    /// Pop the next task from the queue
    pub async fn pop(&self) -> Option<Task> {
        let mut queue = self.queue.lock().await;
        queue.pop().map(|entry| entry.task)
    }

    /// Get the number of pending tasks
    pub async fn len(&self) -> usize {
        let queue = self.queue.lock().await;
        queue.len()
    }

    /// Check if the queue is empty
    pub async fn is_empty(&self) -> bool {
        let queue = self.queue.lock().await;
        queue.is_empty()
    }

    /// Acquire a permit for concurrent execution
    pub async fn acquire_permit(&self) -> Result<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore
            .acquire()
            .await
            .map_err(|e| SerialError::Io(std::io::Error::other(e)))
    }

    /// Get the maximum concurrent tasks
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
}
