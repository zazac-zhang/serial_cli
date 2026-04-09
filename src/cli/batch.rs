//! Batch runner
//!
//! This module provides batch processing mode for running multiple scripts.

use crate::error::{Result, SerialError};
use crate::lua::executor::ScriptEngine;
use crate::task::executor::TaskExecutor;
use crate::task::{Task, TaskPriority, TaskType};
use crate::utils::ProgressReporter;
use std::path::Path;
use std::sync::Arc;
use tokio::time::Duration;

/// Batch script line type
#[derive(Debug, Clone)]
pub enum BatchLine {
    /// Comment line
    Comment(String),
    /// Script to execute
    Script(std::path::PathBuf),
    /// Conditional execution
    Conditional {
        condition: String,
        scripts: Vec<std::path::PathBuf>,
    },
    /// Loop execution
    Loop {
        count: usize,
        scripts: Vec<std::path::PathBuf>,
    },
    /// Sleep/delay
    Sleep(Duration),
}

/// Batch runner configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Task timeout in seconds
    pub timeout_secs: u64,
    /// Continue on error
    pub continue_on_error: bool,
    /// Enable progress reporting
    pub show_progress: bool,
    /// Verbose output
    pub verbose: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 5,
            timeout_secs: 60,
            continue_on_error: false,
            show_progress: true,
            verbose: false,
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

impl BatchRunner {
    /// Parse a batch file into executable lines
    pub fn parse_batch_file(&self, path: &Path) -> Result<Vec<BatchLine>> {
        let content = std::fs::read_to_string(path).map_err(SerialError::Io)?;
        let mut lines = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Comment line
            if trimmed.starts_with('#') {
                lines.push(BatchLine::Comment(trimmed.to_string()));
                continue;
            }

            // Check for loop directive
            if trimmed.starts_with("loop ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let count: usize = parts[1].parse().unwrap_or(1);
                    lines.push(BatchLine::Loop {
                        count,
                        scripts: vec![],
                    });
                }
                continue;
            }

            // Check for sleep directive
            if trimmed.starts_with("sleep ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ms: u64 = parts[1].parse().unwrap_or(1000);
                    lines.push(BatchLine::Sleep(Duration::from_millis(ms)));
                }
                continue;
            }

            // Script path
            lines.push(BatchLine::Script(Path::new(trimmed).to_path_buf()));
        }

        Ok(lines)
    }

    /// Run parsed batch lines with enhanced features
    pub async fn run_batch_lines(&self, batch_lines: Vec<BatchLine>) -> Result<BatchResult> {
        let mut results = Vec::new();
        let total_lines = batch_lines.len();
        let mut progress = if self.config.show_progress {
            Some(ProgressReporter::new("Batch execution".to_string(), total_lines))
        } else {
            None
        };

        let mut i = 0;
        while i < batch_lines.len() {
            match &batch_lines[i] {
                BatchLine::Comment(msg) => {
                    if self.config.verbose {
                        println!("  # {}", msg);
                    }
                }
                BatchLine::Script(script_path) => {
                    if self.config.verbose {
                        println!("Running: {}", script_path.display());
                    }

                    match self.run_script(script_path).await {
                        Ok(_) => {
                            results.push(ScriptResult {
                                script: script_path.display().to_string(),
                                success: true,
                                duration: Duration::ZERO,
                            });
                        }
                        Err(e) => {
                            results.push(ScriptResult {
                                script: script_path.display().to_string(),
                                success: false,
                                duration: Duration::ZERO,
                            });

                            if !self.config.continue_on_error {
                                eprintln!("Error executing {}: {}", script_path.display(), e);
                                break;
                            }
                        }
                    }

                    if let Some(ref mut p) = progress {
                        p.update(1);
                    }
                }
                BatchLine::Sleep(duration) => {
                    if self.config.verbose {
                        println!("Sleeping for {:?}", duration);
                    }
                    tokio::time::sleep(*duration).await;
                }
                BatchLine::Loop { count, scripts: _ } => {
                    // Handle loop - execute next lines in a loop
                    let loop_count = *count;
                    if self.config.verbose {
                        println!("Starting loop ({} iterations)", loop_count);
                    }

                    for iteration in 0..loop_count {
                        if self.config.verbose {
                            println!("  Loop iteration {}/{}", iteration + 1, loop_count);
                        }

                        // Execute scripts in the loop (simplified - would need more complex parsing)
                        if let Some(next_line) = batch_lines.get(i + 1) {
                            if let BatchLine::Script(script) = next_line {
                                if let Err(e) = self.run_script(script).await {
                                    eprintln!("Loop iteration failed: {}", e);
                                    if !self.config.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                BatchLine::Conditional { condition, scripts } => {
                    // Handle conditional execution (simplified - always execute for now)
                    if self.config.verbose {
                        println!("Conditional: {} (always true in current implementation)", condition);
                    }

                    for script in scripts {
                        let _ = self.run_script(script).await;
                    }
                }
            }

            i += 1;
        }

        if let Some(ref mut p) = progress {
            p.update(total_lines); // Ensure complete
        }

        Ok(BatchResult { results })
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
