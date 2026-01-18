use crate::{
    graph::{Edge, Node, NodeId},
    AdjList, GraphState,
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Result of executing a workflow
#[derive(Debug)]
pub struct ExecutionResult {
    /// Map of node IDs to their output values
    pub node_outputs: HashMap<String, serde_json::Value>,
    /// Order in which nodes were executed (useful for debugging)
    pub execution_order: Vec<String>,
}
/// Represents a complete workflow with nodes and edges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Map of node IDs to node definitions
    pub nodes: HashMap<NodeId, Node>,
    /// Edges connecting nodes (defines execution order)
    pub edges: Vec<Edge>,
}

impl Workflow {
    /// Validate that this workflow is well-formed
    /// Should check for:
    /// - Cycles in the graph
    /// - Missing node references in edges
    /// - Invalid input references
    // DOCS: used adjacency list because we'd only have to go through edges once + will need to use it later (I think)
    pub fn validate(&self) -> Result<GraphState> {
        // TODO: We should also check for disconnected graphs

        let mut adj_list: AdjList = HashMap::new();
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

        // Traditionally: visited nodes and those in the recursion stack
        let mut safe: HashSet<NodeId> = HashSet::new();
        let mut hot: HashSet<NodeId> = HashSet::new();

        for n in self.nodes.values() {
            // Check for invalid inputs
            Self::check_inputs(&n.inputs, &self.nodes)?;
            // Check for cycles
            if !safe.contains(&n.id) {
                Self::check_cycle(&mut safe, &mut hot, &adj_list, &n.id)?
            }
        }
        Ok(GraphState::new(self))
    }

    /// Check for cycles from the given node
    fn check_cycle(
        safe: &mut HashSet<NodeId>,
        hot: &mut HashSet<NodeId>,
        adj_list: &HashMap<NodeId, Vec<NodeId>>,
        n: &NodeId,
    ) -> Result<()> {
        hot.insert(n.clone());

        for nprime in adj_list.get(n).into_iter().flatten() {
            if hot.contains(nprime) {
                bail!("Cycle detected");
            };
            if !safe.contains(nprime) {
                Self::check_cycle(safe, hot, adj_list, nprime)?;
            };
        }

        hot.remove(n);
        safe.insert(n.clone());
        Ok(())
    }

    /// Check if a node has valid inputs
    /// This means the input is dot separated string with two parts: (id, field)
    /// The id must be a valid node_id
    fn check_inputs(inputs: &[String], nodes: &HashMap<NodeId, Node>) -> Result<()> {
        for i in inputs.iter() {
            let (id, _) = parse_input_reference(i)?;
            if !nodes.contains_key(&id) {
                bail!("Invalid node id in input");
            }
        }
        Ok(())
    }

    /// Get all nodes that this node depends on (incoming edges)
    /// Assumes the graph is valid
    pub fn get_dependencies(&self, node_id: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.to == node_id)
            .map(|e| e.from.clone())
            .collect()
    }

    /// Get all nodes that depend on the given node (outgoing edges), filtered by edge conditions
    /// Assumes the graph is valid
    pub fn get_dependents(&self, node_id: &str, output: &Value) -> Vec<String> {
        self.edges
            .iter()
            .filter(|e| e.from == node_id && should_take_edge(output, &e.condition))
            .map(|e| e.to.clone())
            .collect()
    }
}

/// Parse an input reference like "node1.output" into (node_id, field)
pub fn parse_input_reference(reference: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = reference.split('.').collect();
    if parts.len() != 2 {
        bail!("Invalid input reference: {}", reference);
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Check if an edge should be taken based on node output and edge condition
fn should_take_edge(node_output: &Value, edge_condition: &Option<String>) -> bool {
    match edge_condition {
        None => true, // Unconditional edge
        Some(expected) => match node_output {
            Value::Bool(b) => {
                let expected_bool = expected == "true";
                *b == expected_bool
            }
            // node_output is restricted to a bool in graph::execute_with_context
            _ => false,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::NodeType;
    use serde_json::json;

    fn node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            node_type: NodeType::Start,
            inputs: vec![],
        }
    }

    fn edge(from: &str, to: &str) -> Edge {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
            condition: None,
        }
    }

    #[test]
    fn test_should_take_edge() {
        // Unconditional edge - always taken
        assert!(should_take_edge(&json!(true), &None));
        assert!(should_take_edge(&json!(false), &None));
        assert!(should_take_edge(&json!("anything"), &None));

        // Matching conditions
        assert!(should_take_edge(&json!(true), &Some("true".to_string())));
        assert!(should_take_edge(&json!(false), &Some("false".to_string())));

        // Non-matching conditions
        assert!(!should_take_edge(&json!(true), &Some("false".to_string())));
        assert!(!should_take_edge(&json!(false), &Some("true".to_string())));

        // Non-bool node_output - should not take edge
        assert!(!should_take_edge(&json!("true"), &Some("true".to_string())));
        assert!(!should_take_edge(&json!(123), &Some("true".to_string())));
        assert!(!should_take_edge(&json!(null), &Some("false".to_string())));
    }

    #[test]
    fn test_check_cycle() {
        // Acyclic Graph
        let mut adj_list = HashMap::new();
        adj_list.insert("a".to_string(), vec!["b".to_string()]);
        adj_list.insert("b".to_string(), vec!["c".to_string()]);
        adj_list.insert("c".to_string(), vec![]);

        let mut safe = HashSet::new();
        let mut hot = HashSet::new();
        let res = Workflow::check_cycle(&mut safe, &mut hot, &adj_list, &"a".to_string());
        assert!(res.is_ok(), "Acyclic graph should be valid");

        // Self loop
        let mut adj_list_self = HashMap::new();
        adj_list_self.insert("a".to_string(), vec!["a".to_string()]);

        let mut safe = HashSet::new();
        let mut hot = HashSet::new();
        let res = Workflow::check_cycle(&mut safe, &mut hot, &adj_list_self, &"a".to_string());
        assert!(res.is_err(), "Self loop should be detected");

        // Simple cycle
        let mut adj_list_cycle = HashMap::new();
        adj_list_cycle.insert("a".to_string(), vec!["b".to_string()]);
        adj_list_cycle.insert("b".to_string(), vec!["c".to_string()]);
        adj_list_cycle.insert("c".to_string(), vec!["a".to_string()]);

        let mut safe = HashSet::new();
        let mut hot = HashSet::new();
        let res = Workflow::check_cycle(&mut safe, &mut hot, &adj_list_cycle, &"a".to_string());
        assert!(res.is_err(), "Cycle should be detected");

        // Diamond DAG
        let mut adj_list_diamond = HashMap::new();
        adj_list_diamond.insert("a".to_string(), vec!["b".to_string(), "c".to_string()]);
        adj_list_diamond.insert("b".to_string(), vec!["d".to_string()]);
        adj_list_diamond.insert("c".to_string(), vec!["d".to_string()]);
        adj_list_diamond.insert("d".to_string(), vec![]);

        let mut safe = HashSet::new();
        let mut hot = HashSet::new();
        let res = Workflow::check_cycle(&mut safe, &mut hot, &adj_list_diamond, &"a".to_string());
        assert!(res.is_ok(), "Diamond DAG should be valid");

        // Empty graph
        let adj_list_empty = HashMap::new();
        let mut safe = HashSet::new();
        let mut hot = HashSet::new();
        let res = Workflow::check_cycle(&mut safe, &mut hot, &adj_list_empty, &"a".to_string());
        assert!(res.is_ok(), "Empty graph should be valid");
    }

    #[test]
    fn test_validate() {
        // Missing node reference in edges
        let mut nodes = HashMap::new();
        nodes.insert("a".to_string(), node("a"));

        let edges = vec![edge("a", "missing")];

        let workflow = Workflow {
            nodes: nodes.clone(),
            edges,
        };

        let res = workflow.validate();
        assert_eq!(
            res.unwrap_err().to_string(),
            "Node in edge does not exist".to_string()
        );

        // Invalid input reference
        let mut nodes_with_inputs = HashMap::new();
        nodes_with_inputs.insert(
            "a".to_string(),
            Node {
                id: "a".to_string(),
                node_type: NodeType::Add,
                inputs: vec!["nonexistent.output".to_string()],
            },
        );

        let workflow_inputs = Workflow {
            nodes: nodes_with_inputs,
            edges: vec![],
        };
        let res = workflow_inputs.validate();
        assert_eq!(
            res.unwrap_err().to_string(),
            "Invalid node id in input".to_string()
        );

        // Invalid input reference again
        let mut nodes_with_inputs = HashMap::new();
        nodes_with_inputs.insert(
            "a".to_string(),
            Node {
                id: "a".to_string(),
                node_type: NodeType::Add,
                inputs: vec!["bad".to_string()],
            },
        );

        let workflow_inputs = Workflow {
            nodes: nodes_with_inputs,
            edges: vec![],
        };
        let res = workflow_inputs.validate();
        assert_eq!(
            res.unwrap_err().to_string(),
            "Invalid input reference: bad".to_string()
        );

        // Sanity check
        let mut nodes = HashMap::new();
        nodes.insert("a".to_string(), node("a"));
        nodes.insert("b".to_string(), node("b"));
        nodes.insert("c".to_string(), node("c"));

        let edges = vec![edge("a", "b"), edge("b", "c")];
        let workflow = Workflow {
            nodes: nodes.clone(),
            edges,
        };
        assert!(workflow.validate().is_ok(), "Acyclic graph should be valid");
    }
}
