use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, map::Values, Value};
use std::{
    any,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::workflow::Workflow;

pub type NodeId = String;
pub type AdjList = HashMap<NodeId, Vec<NodeId>>;

// TODO (PS): add a phantom marker to tie lifetime to workflow?
/// Key components of Workflow DAG, tracks state changes and job ouputs
#[derive(Debug)]
pub struct GraphState {
    pub completed: HashSet<NodeId>,
    // Context for each node after executing
    pub outputs: HashMap<NodeId, Value>,
    pub execution_order: Vec<NodeId>,
    // Dependency count of node
    pub in_degree: HashMap<NodeId, usize>,
    // Map of nodes pointing to their dependents
    pub adj_list: AdjList,
}

impl GraphState {
    pub fn new(workflow: &Workflow, adj_list: AdjList) -> Self {
        let in_degree: HashMap<NodeId, usize> = workflow
            .nodes
            .keys()
            .map(|id| (id.clone(), workflow.get_dependencies(id).len()))
            .collect();
        Self {
            completed: HashSet::new(),
            outputs: HashMap::new(),
            execution_order: Vec::new(),
            in_degree,
            adj_list,
        }
    }

    /// Get all nodes that have no dependencies so that they can be executed
    pub fn get_ready_nodes(&self) -> Vec<NodeId> {
        self.in_degree
            .iter()
            .filter(|(n, &u)| u == 0 && !self.completed.contains(*n))
            .map(|(n, _)| n.clone())
            .collect()
    }

    /// Update state after a job is completed
    pub fn update(&mut self, node_id: NodeId, output: Value, deps: &Vec<NodeId>) -> Result<()> {
        self.outputs.insert(node_id.clone(), output.clone());
        self.completed.insert(node_id.clone());
        self.execution_order.push(node_id.clone());

        // Adjust dependencies so that nodes can enter the ready queue
        for id in deps {
            *self
                .in_degree
                .get_mut(id)
                .ok_or_else(|| anyhow!("Updating on a node that does not exist"))? -= 1;
        }
        Ok(())
    }
}
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
    /// (only needed for bonus Add node implementation)
    pub async fn execute_with_context(&self, context: &HashMap<String, Value>) -> Result<Value> {
        match &self.node_type {
            NodeType::Start => Ok(json!(null)),

            NodeType::End => Ok(json!(null)), // TODO (PS) collect inputs later

            NodeType::Echo { message } => Ok(json!(message)),

            NodeType::IfElse { condition } => match condition.as_str() {
                "true" => Ok(json!(true)),
                "false" => Ok(json!(false)),
                _ => Ok(json!(false)),
            },
            NodeType::Add => {
                // Hint:
                // 1. Get input values from context
                // 2. Convert each to a number
                // 3. Sum them up
                // 4. Return as JSON number
                todo!("BONUS: Implement Add node execution") // TODO (PS) will tackle this towards the end
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
