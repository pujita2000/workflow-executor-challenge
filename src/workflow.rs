use crate::graph::{Edge, Node, NodeId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Represents a complete workflow with nodes and edges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Map of node IDs to node definitions
    pub nodes: HashMap<NodeId, Node>,
    /// Edges connecting nodes (defines execution order)
    pub edges: Vec<Edge>,
}

/// Result of executing a workflow
#[derive(Debug)]
pub struct ExecutionResult {
    /// Map of node IDs to their output values
    pub node_outputs: HashMap<String, serde_json::Value>,
    /// Order in which nodes were executed (useful for debugging)
    pub execution_order: Vec<String>,
}

impl Workflow {
    /// Validate that this workflow is well-formed
    /// Should check for:
    /// - Cycles in the graph
    /// - Missing node references in edges
    /// - Invalid input references
    // TODO (PS): check for disconnected components
    // DOCS: used adjacency list because we'd only have to go through edges once + will need to use it later (I think)
    pub fn validate(&self) -> Result<()> {
        let mut adj_list: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        for e in &self.edges {
            // Check for invalid nodes
            if !self.nodes.contains_key(&e.from) || !self.nodes.contains_key(&e.to) {
                bail!("Node in edge does not exist");
            }
            adj_list
                .entry(e.from.clone())
                .or_default()
                .push(e.to.clone());
        }

        // Traditionally visited nodes and those in the recursion stack
        let mut safe: HashSet<NodeId> = HashSet::new();
        let mut hot: HashSet<NodeId> = HashSet::new();

        for n in self.nodes.values() {
            // Check for invalid inputs
            check_inputs(&n.inputs, &self.nodes)?;
            if !safe.contains(&n.id) {
                Self::check_cycle(&mut safe, &mut hot, &adj_list, &n.id)?
            }
        }
        Ok(())
    }

    /// Check cycles from the given node
    fn check_cycle(
        safe: &mut HashSet<NodeId>,
        hot: &mut HashSet<NodeId>,
        adj_list: &HashMap<NodeId, Vec<NodeId>>,
        n: &NodeId,
    ) -> Result<()> {
        safe.insert(n.clone());
        hot.insert(n.clone());

        for nprime in &adj_list[n] {
            if !safe.contains(nprime) {
                Self::check_cycle(safe, hot, adj_list, nprime)?;
            } else if hot.contains(nprime) {
                bail!("Cycle detected");
            }
        }
        Ok(())
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

/// Check if a node has valid inputs
fn check_inputs(inputs: &[String], nodes: &HashMap<NodeId, Node>) -> Result<()> {
    for i in inputs.iter() {
        let (id, _) = parse_input_reference(i)?;
        if !nodes.contains_key(&id) {
            bail!("Invalid node id in input");
        }
    }
    Ok(())
}

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
