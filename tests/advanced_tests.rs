//! Advanced test suite - Bonus tests
//!
//! These tests are optional but demonstrate advanced understanding:
//! - Data flow between nodes (Add node)
//! - Cycle detection
//! - Concurrent execution verification
//! - Complex workflow patterns

use serde_json::json;
use std::time::{Duration, Instant};
use workflow_executor::{SimpleExecutor, WorkflowExecutor};

#[tokio::test]
async fn test_add_node_with_data_flow() {
    // Tests that the Add node can receive inputs from previous nodes
    // and sum them correctly
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
                "node_type": {"type": "Echo", "message": "10"},
                "inputs": []
            },
            "echo2": {
                "id": "echo2",
                "node_type": {"type": "Echo", "message": "20"},
                "inputs": []
            },
            "add": {
                "id": "add",
                "node_type": {"type": "Add"},
                "inputs": ["echo1.output", "echo2.output"]
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": ["add.output"]
            }
        },
        "edges": [
            {"from": "start", "to": "echo1", "condition": null},
            {"from": "start", "to": "echo2", "condition": null},
            {"from": "echo1", "to": "add", "condition": null},
            {"from": "echo2", "to": "add", "condition": null},
            {"from": "add", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    // Verify the Add node correctly summed the inputs
    assert_eq!(
        result.node_outputs.get("add"),
        Some(&json!(30.0)),
        "Add node should sum numeric inputs (10 + 20 = 30)"
    );
}

#[tokio::test]
async fn test_complex_data_flow() {
    // More complex data flow: multiple Add nodes
    //     echo(5) --\
    //                 -> add1 -> add3 -> end
    //     echo(3) --/        /
    //                       /
    //     echo(7) ---------/
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "echo1": {
                "id": "echo1",
                "node_type": {"type": "Echo", "message": "5"},
                "inputs": []
            },
            "echo2": {
                "id": "echo2",
                "node_type": {"type": "Echo", "message": "3"},
                "inputs": []
            },
            "echo3": {
                "id": "echo3",
                "node_type": {"type": "Echo", "message": "7"},
                "inputs": []
            },
            "add1": {
                "id": "add1",
                "node_type": {"type": "Add"},
                "inputs": ["echo1.output", "echo2.output"]
            },
            "add2": {
                "id": "add2",
                "node_type": {"type": "Add"},
                "inputs": ["add1.output", "echo3.output"]
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": ["add2.output"]
            }
        },
        "edges": [
            {"from": "echo1", "to": "add1", "condition": null},
            {"from": "echo2", "to": "add1", "condition": null},
            {"from": "add1", "to": "add2", "condition": null},
            {"from": "echo3", "to": "add2", "condition": null},
            {"from": "add2", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");

    assert_eq!(result.node_outputs.get("add1"), Some(&json!(8.0)));
    assert_eq!(
        result.node_outputs.get("add2"),
        Some(&json!(15.0)),
        "Should compute (5 + 3) + 7 = 15"
    );
}

#[tokio::test]
async fn test_cycle_detection() {
    // Workflow with a cycle: a -> b -> c -> a
    // This should be detected and rejected
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "a": {
                "id": "a",
                "node_type": {"type": "Echo", "message": "A"},
                "inputs": []
            },
            "b": {
                "id": "b",
                "node_type": {"type": "Echo", "message": "B"},
                "inputs": []
            },
            "c": {
                "id": "c",
                "node_type": {"type": "Echo", "message": "C"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "a", "to": "b", "condition": null},
            {"from": "b", "to": "c", "condition": null},
            {"from": "c", "to": "a", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor.execute(workflow).await;

    assert!(
        result.is_err(),
        "Cyclic workflows should be rejected with an error"
    );

    // Optionally check the error message contains "cycle"
    if let Err(e) = result {
        let error_msg = e.to_string().to_lowercase();
        assert!(
            error_msg.contains("cycle") || error_msg.contains("cyclic"),
            "Error should mention cycle detection, got: {}",
            e
        );
    }
}

#[tokio::test]
async fn test_self_cycle_detection() {
    // Edge case: node points to itself
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "loop": {
                "id": "loop",
                "node_type": {"type": "Echo", "message": "Loop"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "loop", "to": "loop", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor.execute(workflow).await;

    assert!(result.is_err(), "Self-referencing nodes should be rejected");
}

#[tokio::test]
async fn test_missing_node_reference() {
    // Workflow references a node that doesn't exist
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "start", "to": "nonexistent", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor.execute(workflow).await;

    assert!(
        result.is_err(),
        "References to missing nodes should be rejected"
    );
}

#[tokio::test]
// This test requires actual concurrent execution
async fn test_concurrent_execution_timing() {
    // This test verifies that parallel branches actually execute concurrently
    // by measuring execution time
    //
    // To make this test meaningful, your Echo nodes should support a delay
    // or you can add a Sleep node type
    //
    // Expected: Two 100ms sleeps in parallel should take ~100ms, not 200ms

    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "slow1": {
                "id": "slow1",
                "node_type": {"type": "Echo", "message": "Slow 1"},
                "inputs": []
            },
            "slow2": {
                "id": "slow2",
                "node_type": {"type": "Echo", "message": "Slow 2"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": ["slow1.output", "slow2.output"]
            }
        },
        "edges": [
            {"from": "start", "to": "slow1", "condition": null},
            {"from": "start", "to": "slow2", "condition": null},
            {"from": "slow1", "to": "end", "condition": null},
            {"from": "slow2", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;

    let start = Instant::now();
    let _result = executor
        .execute(workflow)
        .await
        .expect("Workflow execution failed");
    let duration = start.elapsed();

    // If parallel: ~100ms, if sequential: ~200ms
    // Allow some overhead, so check it's less than 150ms
    assert!(
        duration < Duration::from_millis(150),
        "Parallel execution should complete in ~100ms, took {:?}",
        duration
    );
}

#[tokio::test]
async fn test_diamond_pattern() {
    // Diamond dependency pattern:
    //        /-> b -\
    //    a ->        -> d
    //        \-> c -/
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "a": {
                "id": "a",
                "node_type": {"type": "Echo", "message": "A"},
                "inputs": []
            },
            "b": {
                "id": "b",
                "node_type": {"type": "Echo", "message": "B"},
                "inputs": []
            },
            "c": {
                "id": "c",
                "node_type": {"type": "Echo", "message": "C"},
                "inputs": []
            },
            "d": {
                "id": "d",
                "node_type": {"type": "Echo", "message": "D"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "a", "to": "b", "condition": null},
            {"from": "a", "to": "c", "condition": null},
            {"from": "b", "to": "d", "condition": null},
            {"from": "c", "to": "d", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Diamond pattern should execute successfully");

    // Verify execution order respects dependencies
    let pos_a = result
        .execution_order
        .iter()
        .position(|n| n == "a")
        .unwrap();
    let pos_b = result
        .execution_order
        .iter()
        .position(|n| n == "b")
        .unwrap();
    let pos_c = result
        .execution_order
        .iter()
        .position(|n| n == "c")
        .unwrap();
    let pos_d = result
        .execution_order
        .iter()
        .position(|n| n == "d")
        .unwrap();

    assert!(pos_a < pos_b, "a must execute before b");
    assert!(pos_a < pos_c, "a must execute before c");
    assert!(pos_b < pos_d, "b must execute before d");
    assert!(pos_c < pos_d, "c must execute before d");

    // All nodes should have outputs
    assert!(result.node_outputs.contains_key("a"));
    assert!(result.node_outputs.contains_key("b"));
    assert!(result.node_outputs.contains_key("c"));
    assert!(result.node_outputs.contains_key("d"));
}

#[tokio::test]
async fn test_nested_conditionals() {
    // Tests nested if/else patterns
    let workflow = serde_json::from_str(
        r#"{
        "nodes": {
            "start": {
                "id": "start",
                "node_type": {"type": "Start"},
                "inputs": []
            },
            "if1": {
                "id": "if1",
                "node_type": {"type": "IfElse", "condition": "true"},
                "inputs": []
            },
            "if2": {
                "id": "if2",
                "node_type": {"type": "IfElse", "condition": "false"},
                "inputs": []
            },
            "path_a": {
                "id": "path_a",
                "node_type": {"type": "Echo", "message": "Path A"},
                "inputs": []
            },
            "path_b": {
                "id": "path_b",
                "node_type": {"type": "Echo", "message": "Path B"},
                "inputs": []
            },
            "path_c": {
                "id": "path_c",
                "node_type": {"type": "Echo", "message": "Path C"},
                "inputs": []
            },
            "end": {
                "id": "end",
                "node_type": {"type": "End"},
                "inputs": []
            }
        },
        "edges": [
            {"from": "start", "to": "if1", "condition": null},
            {"from": "if1", "to": "if2", "condition": "true"},
            {"from": "if1", "to": "path_c", "condition": "false"},
            {"from": "if2", "to": "path_a", "condition": "true"},
            {"from": "if2", "to": "path_b", "condition": "false"},
            {"from": "path_a", "to": "end", "condition": null},
            {"from": "path_b", "to": "end", "condition": null},
            {"from": "path_c", "to": "end", "condition": null}
        ]
    }"#,
    )
    .expect("Failed to parse workflow JSON");

    let executor = SimpleExecutor;
    let result = executor
        .execute(workflow)
        .await
        .expect("Nested conditionals should execute");

    // if1=true -> if2, if2=false -> path_b
    assert!(!result.execution_order.contains(&"path_a".to_string()));
    assert!(result.execution_order.contains(&"path_b".to_string()));
    assert!(!result.execution_order.contains(&"path_c".to_string()));
}
