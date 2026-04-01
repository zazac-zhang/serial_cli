//! Task executor
//!
//! This module provides asynchronous task execution.

use crate::error::Result;
use crate::task::{Task, TaskResult, TaskType};
use crate::task::queue::TaskQueue;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

/// Task executor
pub struct TaskExecutor {
    /// Task queue
    queue: Arc<TaskQueue>,
    /// Running tasks
    running_tasks: Arc<RwLock<Vec<TaskInfo>>>,
    /// Completed tasks
    completed_tasks: Arc<Mutex<Vec<TaskCompletion>>>,
    /// Executor is running
    running: Arc<Mutex<bool>>,
}

/// Information about a running task
#[derive(Debug, Clone)]
struct TaskInfo {
    id: String,
    started_at: std::time::Instant,
}

/// Task completion information
#[derive(Debug, Clone)]
pub struct TaskCompletion {
    pub task_id: String,
    pub result: TaskResult,
    pub duration: Duration,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            queue: Arc::new(TaskQueue::new(max_concurrent)),
            running_tasks: Arc::new(RwLock::new(Vec::new())),
            completed_tasks: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start the executor
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        tracing::info!("Task executor started");

        // Spawn execution loop
        let queue = self.queue.clone();
        let running_tasks = self.running_tasks.clone();
        let completed_tasks = self.completed_tasks.clone();
        let running_flag = self.running.clone();

        tokio::spawn(async move {
            while *running_flag.lock().await {
                // Try to get next task
                if let Some(task) = queue.pop().await {
                    // Acquire permit for concurrency control
                    if let Ok(_permit) = queue.acquire_permit().await {
                        let task_id = task.id();
                        let task_type = task.task_type().clone();

                        // Add to running tasks
                        {
                            let mut tasks = running_tasks.write().await;
                            tasks.push(TaskInfo {
                                id: task_id.clone(),
                                started_at: std::time::Instant::now(),
                            });
                        }

                        // Spawn task execution
                        let completed_tasks_clone = completed_tasks.clone();
                        let running_tasks_clone = running_tasks.clone();

                        tokio::spawn(async move {
                            let start = std::time::Instant::now();
                            let result = Self::execute_task_internal(task_type).await;
                            let duration = start.elapsed();

                            // Record completion
                            {
                                let mut completions = completed_tasks_clone.lock().await;
                                completions.push(TaskCompletion {
                                    task_id: task_id.clone(),
                                    result,
                                    duration,
                                });
                            }

                            // Remove from running tasks
                            {
                                let mut tasks = running_tasks_clone.write().await;
                                tasks.retain(|t| t.id != task_id);
                            }
                        });
                    }
                } else {
                    // No tasks, sleep a bit
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        });

        Ok(())
    }

    /// Stop the executor
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        tracing::info!("Task executor stopped");
        Ok(())
    }

    /// Submit a task for execution
    pub async fn submit(&self, task: Task, priority: crate::task::TaskPriority) -> Result<()> {
        self.queue.push(task, priority).await
    }

    /// Get the number of pending tasks
    pub async fn pending_count(&self) -> usize {
        self.queue.len().await
    }

    /// Get the number of running tasks
    pub async fn running_count(&self) -> usize {
        let tasks = self.running_tasks.read().await;
        tasks.len()
    }

    /// Get completed tasks
    pub async fn get_completed(&self) -> Vec<TaskCompletion> {
        let completions = self.completed_tasks.lock().await;
        completions.clone()
    }

    /// Clear completed tasks
    pub async fn clear_completed(&self) {
        let mut completions = self.completed_tasks.lock().await;
        completions.clear();
    }

    /// Internal task execution
    async fn execute_task_internal(task_type: TaskType) -> TaskResult {
        match task_type {
            TaskType::Script { name, content } => {
                tracing::info!("Executing script: {}", name);

                // Execute Lua script
                use crate::lua::executor::ScriptEngine;

                match ScriptEngine::new() {
                    Ok(engine) => match engine.execute_string(&content) {
                        Ok(_) => TaskResult::Success,
                        Err(e) => TaskResult::Error(format!("{:?}", e)),
                    },
                    Err(e) => TaskResult::Error(format!("Failed to create engine: {:?}", e)),
                }
            }
            TaskType::SerialOp { port_name, operation: _ } => {
                tracing::info!("Executing serial operation on {}", port_name);

                // For now, just return success
                // In real implementation, this would perform the actual serial operation
                TaskResult::Success
            }
            TaskType::Custom { name, data } => {
                tracing::info!("Executing custom task: {}", name);
                TaskResult::SuccessWithText(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::TaskPriority;

    #[tokio::test]
    async fn test_executor_creation() {
        let executor = TaskExecutor::new(10);
        assert_eq!(executor.pending_count().await, 0);
        assert_eq!(executor.running_count().await, 0);
    }

    #[tokio::test]
    async fn test_submit_task() {
        let executor = TaskExecutor::new(10);

        let task = Task::new(TaskType::Custom {
            name: "test".to_string(),
            data: "test data".to_string(),
        });

        executor.submit(task, TaskPriority::Normal).await.unwrap();
        assert_eq!(executor.pending_count().await, 1);
    }

    #[tokio::test]
    async fn test_start_stop() {
        let executor = TaskExecutor::new(10);
        executor.start().await.unwrap();
        executor.stop().await.unwrap();
    }
}
