//! Workflow Executor Library
//!
//! This library provides the core trait for executing workflows

mod graph;
mod workflow;

pub use graph::*;
pub use workflow::*;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;

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
    async fn execute(&self, workflow: Workflow) -> Result<ExecutionResult>;
}

/// Simple implementation of Workflow Executor
///
/// Parse and validates workflow structure
/// Executes nodes in correct order, in parallel when possible
/// Collects and returns results from multiple branching and node types
pub struct SimpleExecutor;

#[async_trait]
impl WorkflowExecutor for SimpleExecutor {
    async fn execute(&self, workflow: Workflow) -> Result<ExecutionResult> {
        // Handle empty workflows
        if workflow.nodes.is_empty() {
            return Ok(ExecutionResult {
                node_outputs: HashMap::new(),
                execution_order: Vec::new(),
            });
        }

        let mut state = workflow.validate()?;

        loop {
            let ready = state.get_ready_nodes();

            // No nodes left to execute
            if ready.is_empty() {
                break;
            }

            let mut handles = Vec::new();
            for node_id in ready.iter() {
                let node = workflow
                    .nodes
                    .get(node_id)
                    .ok_or_else(|| anyhow!("Trying to execute a node that does not exist"))?
                    .clone();

                // Get context from state (stores execution progress)
                let context = state.outputs.clone();

                let handle = tokio::spawn(async move { node.execute_with_context(&context).await });
                handles.push((node_id, handle));
            }

            for (node_id, handle) in handles {
                let output = handle.await??;
                // Adjust dependencies so that nodes can enter the ready queue
                let deps = workflow.get_dependents(node_id, &output);
                state.update(node_id.clone(), output, &deps)?;
            }
        }
        Ok(ExecutionResult {
            node_outputs: state.outputs.to_owned(),
            execution_order: state.execution_order.to_owned(),
        })
    }
}
