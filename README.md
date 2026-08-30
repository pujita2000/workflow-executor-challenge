# Workflow Executor

A concurrent DAG-based workflow execution engine built in Rust.

## Overview
Implements a workflow executor that runs directed acyclic graphs (DAGs) of tasks with support for:

Parallel execution of independent nodes using async Rust / Tokio
Conditional branching (IfElse nodes)
Data flow between nodes
Cycle detection and graph validation

## Running Tests

`cargo test --test basic_tests`

All 6 basic tests pass. Advanced tests (Add node, diamond patterns, concurrent timing verification) also implemented.

## Tech
Rust, Tokio, async/await, serde_json
