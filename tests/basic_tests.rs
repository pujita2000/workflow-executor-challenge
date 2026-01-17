//! Basic test suite - All tests here must pass for submission
//!
//! These tests cover the core functionality of the workflow executor:
//! - Linear execution (sequential nodes)
//! - Parallel execution (independent nodes)
//! - Conditional branching (if/else)

use serde_json::json;
use workflow_executor::{SimpleExecutor, WorkflowExecutor};

#[tokio::test]
async fn test_linear_workflow() {
    // Simple linear workflow: start -> echo -> end
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "echo1": {
                "id": "echo1",
                "node_type": {"type": "Echo", "message": "Hello World"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": ["echo1.output"]
            }
        },
        "edges": [
            {"from": "start", "to": "echo1", "condition": null},
            {"from": "echo1", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    // Verify execution order
    assert_eq!(
        result.execution_order,
        vec!["start", "echo1", "end"],
        "Nodes should execute in order: start -> echo1 -> end"
    );

    // Verify output
    assert_eq!(
        result.node_outputs.get("echo1"),
        Some(&json!("Hello World")),
        "Echo node should output its message"
    );
}

#[tokio::test]
async fn test_parallel_execution() {
    // Workflow with parallel branches:
    //        /-> echo1 -\
    // start              -> end
    //        \-> echo2 -/
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "echo1": {
                "id": "echo1",
                "node_type": {"type": "Echo", "message": "Branch A"},
                "inputs": []
            },
            "echo2": {
                "id": "echo2",
                "node_type": {"type": "Echo", "message": "Branch B"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": ["echo1.output", "echo2.output"]
            }
        },
        "edges": [
            {"from": "start", "to": "echo1", "condition": null},
            {"from": "start", "to": "echo2", "condition": null},
            {"from": "echo1", "to": "end", "condition": null},
            {"from": "echo2", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    // Verify both branches executed
    assert!(
        result.execution_order.contains(&"echo1".to_string()),
        "echo1 should be in execution order"
    );
    assert!(
        result.execution_order.contains(&"echo2".to_string()),
        "echo2 should be in execution order"
    );

    // Verify both executed before end
    let echo1_pos = result
        .execution_order
        .iter()
        .position(|n| n == "echo1")
        .unwrap();
    let echo2_pos = result
        .execution_order
        .iter()
        .position(|n| n == "echo2")
        .unwrap();
    let end_pos = result
        .execution_order
        .iter()
        .position(|n| n == "end")
        .unwrap();

    assert!(echo1_pos < end_pos, "echo1 should execute before end node");
    assert!(echo2_pos < end_pos, "echo2 should execute before end node");

    // Verify outputs
    assert_eq!(result.node_outputs.get("echo1"), Some(&json!("Branch A")));
    assert_eq!(result.node_outputs.get("echo2"), Some(&json!("Branch B")));
}

#[tokio::test]
async fn test_if_else_takes_true_branch() {
    // Workflow with conditional branching
    //               /-> true_branch -\
    // start -> if                     -> end
    //               \-> false_branch -/
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "if_node": {
                "id": "if_node",
                "node_type": {"type": "IfElse", "condition": "true"},
                "inputs": []
            },
            "true_branch": {
                "id": "true_branch",
                "node_type": {"type": "Echo", "message": "Took true path"},
                "inputs": []
            },
            "false_branch": {
                "id": "false_branch",
                "node_type": {"type": "Echo", "message": "Took false path"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "start", "to": "if_node", "condition": null},
            {"from": "if_node", "to": "true_branch", "condition": "true"},
            {"from": "if_node", "to": "false_branch", "condition": "false"},
            {"from": "true_branch", "to": "end", "condition": null},
            {"from": "false_branch", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    // Only true branch should execute
    assert!(
        result.execution_order.contains(&"true_branch".to_string()),
        "true_branch should execute when condition is true"
    );
    assert!(
        !result.execution_order.contains(&"false_branch".to_string()),
        "false_branch should NOT execute when condition is true"
    );

    assert_eq!(
        result.node_outputs.get("true_branch"),
        Some(&json!("Took true path"))
    );
    assert_eq!(result.node_outputs.get("false_branch"), None);
}

#[tokio::test]
async fn test_if_else_takes_false_branch() {
    // Same as above but condition is "false"
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "if_node": {
                "id": "if_node",
                "node_type": {"type": "IfElse", "condition": "false"},
                "inputs": []
            },
            "true_branch": {
                "id": "true_branch",
                "node_type": {"type": "Echo", "message": "Took true path"},
                "inputs": []
            },
            "false_branch": {
                "id": "false_branch",
                "node_type": {"type": "Echo", "message": "Took false path"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "start", "to": "if_node", "condition": null},
            {"from": "if_node", "to": "true_branch", "condition": "true"},
            {"from": "if_node", "to": "false_branch", "condition": "false"},
            {"from": "true_branch", "to": "end", "condition": null},
            {"from": "false_branch", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    // Only false branch should execute
    assert!(
        !result.execution_order.contains(&"true_branch".to_string()),
        "true_branch should NOT execute when condition is false"
    );
    assert!(
        result.execution_order.contains(&"false_branch".to_string()),
        "false_branch should execute when condition is false"
    );

    assert_eq!(result.node_outputs.get("true_branch"), None);
    assert_eq!(
        result.node_outputs.get("false_branch"),
        Some(&json!("Took false path"))
    );
}

#[tokio::test]
async fn test_empty_workflow() {
    // Edge case: workflow with no nodes
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {},
        "edges": []
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Empty workflow should execute successfully");

    assert!(
        result.execution_order.is_empty(),
        "Empty workflow should have no executions"
    );
    assert!(
        result.node_outputs.is_empty(),
        "Empty workflow should have no outputs"
    );
}

#[tokio::test]
async fn test_single_node() {
    // Edge case: workflow with only one node
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "only": {
                "id": "only",
                "node_type": {"type": "Echo", "message": "Solo"},
                "inputs": []
            }
        },
        "edges": []
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Single node workflow should execute");

    assert_eq!(result.execution_order, vec!["only"]);
    assert_eq!(result.node_outputs.get("only"), Some(&json!("Solo")));
}
