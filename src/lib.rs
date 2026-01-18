//! Workflow Executor Library
//!
//! This library provides the core types and traits for executing workflows
//! represented as directed acyclic graphs (DAGs) of nodes.
//!
//! Your task is to implement the `SimpleExecutor` struct to execute workflows
//! correctly, handling parallel execution, branching, and data flow between nodes.

// TODO (PS) right now everything happens in tests, lets make a cli command to execute this work
mod graph;
mod workflow;

// Re-export public types
pub use graph::*;
pub use workflow::*;

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

// Import for use within this module
use crate::workflow::{ExecutionResult, Workflow};

/// Trait for executing workflows
#[async_trait]
pub trait WorkflowExecutor {
    /// Execute a workflow and return the results
    ///
    /// # Requirements
    /// - Nodes should execute in topological order (respecting dependencies)
    /// - Independent nodes should execute in parallel
    /// - Conditional edges should only execute the matching branch
    /// - Node inputs should be resolved from previous node outputs
    /// - Return an error for invalid workflows (cycles, missing nodes, etc.)
    async fn execute(&self, workflow: Workflow) -> Result<ExecutionResult>; // TODO (PS): potentially add your own error type here for easier debugging
}

/// Your implementation of the workflow executor
///
/// # Your Task
/// Implement the `WorkflowExecutor` trait for this struct to:
/// 1. Parse and validate the workflow structure // TODO (PS) cycles and broken paths
/// 2. Execute nodes in the correct order
/// 3. Handle parallel execution where possible
/// 4. Resolve node inputs from previous outputs (BONUS)
/// 5. Handle conditional branching
/// 6. Collect and return execution results
pub struct SimpleExecutor;

#[async_trait]
impl WorkflowExecutor for SimpleExecutor {
    async fn execute(&self, workflow: Workflow) -> Result<ExecutionResult> {
        // TODO: Implement workflow execution
        //
        // Suggested approach:
        // 1. Validate the workflow (check for cycles, missing nodes)
        // 2. Build a dependency graph
        // 3. Find nodes that are ready to execute (no dependencies)
        // 4. Execute ready nodes in parallel using tokio::spawn or tokio::join!
        // 5. When nodes complete, update state and find new ready nodes
        // 6. Handle conditional edges (IfElse nodes)
        // 7. Continue until all nodes are executed
        // 8. Return the results

        // Hint: You might want to track:
        // - Which nodes have been completed
        // - Output values from each node
        // - Which nodes are ready to execute next

        // Handle empty workflows
        if workflow.nodes.is_empty() {
            return Ok(ExecutionResult {
                node_outputs: HashMap::new(),
                execution_order: Vec::new(),
            });
        }

        // Your implementation here...

        todo!("Implement workflow execution")
    }
}
