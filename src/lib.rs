//! Workflow Executor Library
//!
//! This library provides the core trait for executing workflows

mod graph;
mod workflow;

pub use graph::*;
pub use workflow::*;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use tokio::task::JoinSet;

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
        let mut jobs: JoinSet<Result<(NodeId, Value)>> = JoinSet::new();

        // Add initially ready nodes to JoinSet
        for node_id in state.get_ready_nodes() {
            let node = workflow.get_node(&node_id)?;
            let context = state.outputs.clone();
            jobs.spawn(async move {
                let output = node.execute_with_context(&context).await?;
                Ok((node.id, output))
            });
        }

        // Process completed jobs as they happen, spawn newly-ready nodes immediately
        while let Some(result) = jobs.join_next().await {
            let (node_id, output) = result??;
            let deps = workflow.get_dependents(&node_id, &output);
            state.update(node_id, output, &deps)?;

            for node_id in state.get_ready_nodes() {
                let node = workflow.get_node(&node_id)?;
                let context = state.outputs.clone();
                jobs.spawn(async move {
                    let output = node.execute_with_context(&context).await?;
                    Ok((node.id, output))
                });
            }
        }

        Ok(ExecutionResult {
            node_outputs: state.outputs.to_owned(),
            execution_order: state.execution_order.to_owned(),
        })
    }
}
