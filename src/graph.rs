use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type NodeId = String;

/// A single node in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node
    pub id: NodeId,
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

impl Node {
    /// Execute this node and return its output value
    ///
    /// Context contains resolved input values from previous nodes
    /// (only needed for bonus Add node implementation) // TODO (PS) will tackle this towards the end
    pub async fn execute_with_context(&self, context: &HashMap<String, Value>) -> Result<Value> {
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

/// An edge connecting two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Source node ID
    pub from: NodeId,
    /// Destination node ID
    pub to: NodeId,
    /// Optional condition for conditional edges (e.g., "true" or "false" for IfElse nodes)
    pub condition: Option<String>,
}
