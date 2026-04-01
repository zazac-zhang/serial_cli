//! Task scheduler module
//!
//! This module provides task scheduling and execution.

pub mod executor;
pub mod monitor;
pub mod queue;

use uuid::Uuid;

/// Task ID
pub type TaskId = String;

/// Task type
#[derive(Debug, Clone)]
pub enum TaskType {
    /// Script execution task
    Script { name: String, content: String },
    /// Serial port operation
    SerialOp {
        port_name: String,
        operation: SerialOperation,
    },
    /// Custom task
    Custom { name: String, data: String },
}

/// Serial operation type
#[derive(Debug, Clone)]
pub enum SerialOperation {
    Send { data: Vec<u8> },
    Recv { bytes: usize },
    Open { config: SerialConfig },
    Close,
}

/// Serial configuration for tasks
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub baudrate: u32,
    pub databits: u8,
    pub stopbits: u8,
    pub parity: String,
}

/// Task
#[derive(Debug, Clone)]
pub struct Task {
    id: TaskId,
    task_type: TaskType,
    created_at: std::time::Instant,
}

impl Task {
    /// Create a new task
    pub fn new(task_type: TaskType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_type,
            created_at: std::time::Instant::now(),
        }
    }

    /// Get task ID
    pub fn id(&self) -> TaskId {
        self.id.clone()
    }

    /// Get task type
    pub fn task_type(&self) -> &TaskType {
        &self.task_type
    }

    /// Get elapsed time since creation
    pub fn elapsed(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Task result
#[derive(Debug, Clone)]
pub enum TaskResult {
    Success,
    SuccessWithData(Vec<u8>),
    SuccessWithText(String),
    Error(String),
}

pub use queue::{TaskPriority, TaskQueue};
