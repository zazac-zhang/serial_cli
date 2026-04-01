//! Batch runner
//!
//! This module provides batch processing mode for running multiple scripts.

use crate::error::{Result, SerialError};
use crate::lua::executor::ScriptEngine;
use crate::task::executor::TaskExecutor;
use crate::task::{Task, TaskPriority, TaskType};
use std::path::Path;
use std::sync::Arc;
use tokio::time::Duration;

/// Batch runner configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Task timeout in seconds
    pub timeout_secs: u64,
    /// Continue on error
    pub continue_on_error: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 5,
            timeout_secs: 60,
            continue_on_error: false,
        }
    }
}

/// Batch runner
pub struct BatchRunner {
    config: BatchConfig,
    executor: Arc<TaskExecutor>,
}

impl BatchRunner {
    /// Create a new batch runner
    pub fn new(config: BatchConfig) -> Result<Self> {
        let executor = Arc::new(TaskExecutor::new(config.max_concurrent));

        Ok(Self { config, executor })
    }

    /// Run a single script
    pub async fn run_script(&self, script_path: &Path) -> Result<()> {
        let engine = ScriptEngine::new()?;
        engine.execute_file(script_path)?;
        Ok(())
    }

    /// Run multiple scripts in sequence
    pub async fn run_scripts(&self, script_paths: Vec<&Path>) -> Result<BatchResult> {
        let executor = self.executor.clone();
        executor.start().await?;

        let mut results = Vec::new();

        for script_path in script_paths {
            let script_content = std::fs::read_to_string(script_path).map_err(SerialError::Io)?;

            let task = Task::new(TaskType::Script {
                name: script_path.display().to_string(),
                content: script_content,
            });

            self.executor.submit(task, TaskPriority::Normal).await?;

            // Wait for task to complete (with timeout)
            let start = std::time::Instant::now();
            loop {
                tokio::time::sleep(Duration::from_millis(100)).await;

                let completed = self.executor.get_completed().await;
                if let Some(last) = completed.last() {
                    results.push(ScriptResult {
                        script: script_path.display().to_string(),
                        success: matches!(last.result, crate::task::TaskResult::Success),
                        duration: last.duration,
                    });

                    if !self.config.continue_on_error
                        && matches!(last.result, crate::task::TaskResult::Error(_))
                    {
                        return Err(SerialError::Script(crate::error::ScriptError::ApiError(
                            "Script execution failed".to_string(),
                        )));
                    }
                    break;
                }

                if start.elapsed() > Duration::from_secs(self.config.timeout_secs) {
                    return Err(SerialError::Io(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Script execution timeout",
                    )));
                }
            }
        }

        executor.stop().await?;

        Ok(BatchResult { results })
    }

    /// Run scripts in parallel
    pub async fn run_scripts_parallel(&self, script_paths: Vec<&Path>) -> Result<BatchResult> {
        let executor = self.executor.clone();
        executor.start().await?;

        // Submit all tasks
        for script_path in script_paths {
            let script_content = std::fs::read_to_string(script_path).map_err(SerialError::Io)?;

            let task = Task::new(TaskType::Script {
                name: script_path.display().to_string(),
                content: script_content,
            });

            self.executor.submit(task, TaskPriority::Normal).await?;
        }

        // Wait for all to complete
        let start = std::time::Instant::now();
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;

            let completed = self.executor.get_completed().await;
            let pending = self.executor.pending_count().await;
            let running = self.executor.running_count().await;

            if pending == 0 && running == 0 && completed.len() >= self.config.max_concurrent {
                break;
            }

            if start.elapsed() > Duration::from_secs(self.config.timeout_secs * 2) {
                return Err(SerialError::Io(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Batch execution timeout",
                )));
            }
        }

        let completed = self.executor.get_completed().await;
        let results: Vec<ScriptResult> = completed
            .into_iter()
            .map(|c| {
                let script_name = c.task_id.clone(); // In real implementation, store script name
                ScriptResult {
                    script: script_name,
                    success: matches!(c.result, crate::task::TaskResult::Success),
                    duration: c.duration,
                }
            })
            .collect();

        executor.stop().await?;

        Ok(BatchResult { results })
    }
}

/// Batch execution result
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub results: Vec<ScriptResult>,
}

/// Script execution result
#[derive(Debug, Clone)]
pub struct ScriptResult {
    pub script: String,
    pub success: bool,
    pub duration: Duration,
}

impl Default for BatchRunner {
    fn default() -> Self {
        Self::new(BatchConfig::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_runner_creation() {
        let runner = BatchRunner::new(BatchConfig::default()).unwrap();
        assert_eq!(runner.config.max_concurrent, 5);
    }

    #[tokio::test]
    async fn test_run_single_script() {
        let runner = BatchRunner::new(BatchConfig::default()).unwrap();

        // Create a test script
        let script_path = Path::new("/tmp/test_batch.lua");
        std::fs::write(script_path, "print('test')").unwrap();

        let result = runner.run_script(script_path).await;
        assert!(result.is_ok());

        // Cleanup
        let _ = std::fs::remove_file(script_path);
    }
}
