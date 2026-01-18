use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::{parse_input_reference, workflow::Workflow};

pub type NodeId = String;
pub type AdjList = HashMap<NodeId, Vec<NodeId>>;

// TODO (PS): add a phantom marker to tie lifetime to workflow?
/// Key components of Workflow DAG, tracks state changes and job ouputs
#[derive(Debug)]
pub struct GraphState {
    pub completed: HashSet<NodeId>,
    /// Context for each node after job complete
    pub outputs: HashMap<NodeId, Value>,
    pub execution_order: Vec<NodeId>,
    /// Number of dependencies a node has
    pub in_degree: HashMap<NodeId, usize>,
}

impl GraphState {
    pub fn new(workflow: &Workflow) -> Self {
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
    pub async fn execute_with_context(&self, context: &HashMap<NodeId, Value>) -> Result<Value> {
        match &self.node_type {
            NodeType::Start => Ok(json!(null)),
            NodeType::End => {
                if self.inputs.is_empty() {
                    return Ok(json!(null));
                }

                let mut coll = Vec::new();
                for i in self.inputs.iter() {
                    let (id, _) = parse_input_reference(i)?;
                    let v = context
                        .get(&id)
                        .ok_or_else(|| anyhow!("Input not in context map"))?;
                    coll.push(v);
                }
                Ok(json!(coll))
            }
            NodeType::Echo { message } => Ok(json!(message)),
            NodeType::IfElse { condition } => match condition.as_str() {
                "true" => Ok(json!(true)),
                "false" => Ok(json!(false)),
                _ => Ok(json!(false)),
            },
            NodeType::Add => {
                let mut sum = 0.0;

                for i in self.inputs.iter() {
                    let (id, _) = parse_input_reference(i)?;
                    let v = context
                        .get(&id)
                        .ok_or_else(|| anyhow!("Input not in context map"))?;
                    let num = value_to_f64(v)?;
                    sum += num;
                }

                Ok(json!(sum))
            }
        }
    }
}

/// Convert a json Number or String to f64
fn value_to_f64(v: &Value) -> Result<f64> {
    match v {
        Value::Number(n) => n.as_f64().ok_or_else(|| anyhow!("Invalid number: {}", n)),
        Value::String(s) => s
            .parse::<f64>()
            .map_err(|_| anyhow!("Invalid number: {}", s)),
        _ => bail!("Expected number or numeric string"),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_to_f64() {
        // Perhaps this is overkill, but let's just be safe

        // Valid integer
        assert_eq!(value_to_f64(&json!(10)).unwrap(), 10.0);
        assert_eq!(value_to_f64(&json!(-5)).unwrap(), -5.0);
        assert_eq!(value_to_f64(&json!(0)).unwrap(), 0.0);

        // Valid float
        assert_eq!(value_to_f64(&json!(10.5)).unwrap(), 10.5);
        assert_eq!(value_to_f64(&json!(-3.1)).unwrap(), -3.1); // clippy stops us from using approximate values of pi

        // Valid numeric string
        assert_eq!(value_to_f64(&json!("10")).unwrap(), 10.0);
        assert_eq!(value_to_f64(&json!("10.5")).unwrap(), 10.5);
        assert_eq!(value_to_f64(&json!("-3.1")).unwrap(), -3.1);

        // Invalid string (not a number)
        assert!(value_to_f64(&json!("hello")).is_err());
        assert!(value_to_f64(&json!("")).is_err());

        // Other types should fail
        assert!(value_to_f64(&json!(null)).is_err());
        assert!(value_to_f64(&json!(true)).is_err());
        assert!(value_to_f64(&json!(false)).is_err());
        assert!(value_to_f64(&json!([])).is_err());
        assert!(value_to_f64(&json!({})).is_err());
    }
}
