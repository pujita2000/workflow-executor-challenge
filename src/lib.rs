//! Workflow Executor Library
//!
//! This library provides the core types and traits for executing workflows
//! represented as directed acyclic graphs (DAGs) of nodes.
//!
//! Your task is to implement the `SimpleExecutor` struct to execute workflows
//! correctly, handling parallel execution, branching, and data flow between nodes.

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents a complete workflow with nodes and edges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Map of node IDs to node definitions
    pub nodes: HashMap<String, Node>,
    /// Edges connecting nodes (defines execution order)
    pub edges: Vec<Edge>,
}

/// A single node in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node
    pub id: String,
    /// The type and configuration of this node
    pub node_type: NodeType,
    /// Input references from other nodes (e.g., ["node1.output", "node2.output"])
    pub inputs: Vec<String>,
}

/// Different types of nodes that can be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    /// Starting point of the workflow (no-op)
    Start,
    /// End point of the workflow (collects all inputs)
    End,
    /// Outputs a static message
    Echo { message: String },
    /// Adds all numeric inputs together (BONUS)
    Add,
    /// Conditional branching based on a condition
    /// The condition is a simple string that can be "true" or "false"
    IfElse { condition: String },
}

/// An edge connecting two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Source node ID
    pub from: String,
    /// Destination node ID
    pub to: String,
    /// Optional condition for conditional edges (e.g., "true" or "false" for IfElse nodes)
    pub condition: Option<String>,
}

/// Result of executing a workflow
#[derive(Debug)]
pub struct ExecutionResult {
    /// Map of node IDs to their output values
    pub node_outputs: HashMap<String, serde_json::Value>,
    /// Order in which nodes were executed (useful for debugging)
    pub execution_order: Vec<String>,
}

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

/// Your implementation of the workflow executor
///
/// # Your Task
/// Implement the `WorkflowExecutor` trait for this struct to:
/// 1. Parse and validate the workflow structure
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

// Helper implementations - you may want to implement these!

impl Workflow {
    /// Validate that this workflow is well-formed
    /// Should check for:
    /// - Cycles in the graph
    /// - Missing node references in edges
    /// - Invalid input references
    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation
        // Hint: Use topological sort or DFS to detect cycles

        todo!("Implement workflow validation")
    }

    /// Get all nodes that this node depends on (incoming edges)
    pub fn get_dependencies(&self, node_id: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.to == node_id)
            .map(|e| e.from.clone())
            .collect()
    }

    /// Get all nodes that depend on the given node (outgoing edges)
    pub fn get_dependents(&self, node_id: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id)
            .map(|e| e.to.clone())
            .collect()
    }
}

impl Node {
    /// Execute this node and return its output value
    ///
    /// Context contains resolved input values from previous nodes
    /// (only needed for bonus Add node implementation)
    pub async fn execute_with_context(
        &self,
        context: &HashMap<String, Value>,
    ) -> Result<Value> {
        match &self.node_type {
            NodeType::Start => {
                // TODO: Implement Start node
                // Hint: Just return null or empty value
                todo!("Implement Start node execution")
            }

            NodeType::End => {
                // TODO: Implement End node
                // Hint: Return null or collect inputs
                todo!("Implement End node execution")
            }

            NodeType::Echo { message } => {
                // TODO: Implement Echo node
                // Hint: Return the message as a JSON string
                todo!("Implement Echo node execution")
            }

            NodeType::IfElse { condition } => {
                // TODO: Implement IfElse node
                // Hint: Parse condition string and return true/false as JSON boolean
                todo!("Implement IfElse node execution")
            }

            NodeType::Add => {
                // Hint:
                // 1. Get input values from context
                // 2. Convert each to a number
                // 3. Sum them up
                // 4. Return as JSON number
                todo!("BONUS: Implement Add node execution")
            }
        }
    }
}

// Additional helper functions you might find useful:

/// Parse an input reference like "node1.output" into (node_id, field)
/// BONUS: Only needed if implementing Add node
#[allow(dead_code)]
fn parse_input_reference(reference: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = reference.split('.').collect();
    if parts.len() != 2 {
        bail!("Invalid input reference: {}", reference);
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Check if an edge should be taken based on node output and edge condition
#[allow(dead_code)]
fn should_take_edge(node_output: &Value, edge_condition: &Option<String>) -> bool {
    match edge_condition {
        None => true, // Unconditional edge
        Some(expected) => {
            // TODO: Compare node output with expected condition
            // Hint: Handle true/false boolean values
            false
        }
    }
}
