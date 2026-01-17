# Profound Workflow Executor Take-Home Challenge

Welcome! This take-home challenge is designed to assess your ability to build a concurrent workflow execution engine in Rust.

## Time Expectation

**Estimated time: 4-8 hours**

We understand everyone works at different paces. Focus on producing quality code rather than rushing. If you need more time, that's perfectly fine - just let us know.

## Overview

You'll implement a workflow executor that can run directed acyclic graphs (DAGs) of tasks. Think of it like a simplified version of workflow engines like Apache Airflow, Temporal, or GitHub Actions.

### Key Features to Implement

1. **DAG Execution**: Execute nodes in topological order respecting dependencies
2. **Parallel Execution**: Run independent nodes concurrently using async Rust
3. **Conditional Branching**: Support if/else logic to skip branches
4. **Data Flow**: Pass outputs from one node to another as inputs
5. **Validation**: Detect invalid workflows (cycles, missing nodes, etc.)

## Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Basic familiarity with async Rust (tokio)

### Project Structure

```
workflow-executor/
├── Cargo.toml          # Dependencies (already configured)
├── src/
│   └── lib.rs          # Core types and trait definition (partially implemented)
├── tests/
│   ├── basic_tests.rs  # Required tests - ALL must pass
│   └── advanced_tests.rs # Bonus tests - optional but recommended
└── examples/           # Sample workflow JSON files
```

### Running Tests

```bash
# Run all basic tests (required)
cargo test --test basic_tests

# Run all advanced tests (bonus)
cargo test --test advanced_tests

# Run specific test
cargo test test_linear_workflow

# Run with output
cargo test -- --nocapture
```

## Your Task

### Core Implementation (Required)

Implement the `WorkflowExecutor` trait for `SimpleExecutor` in `src/lib.rs`.

Your implementation must:

1. **Parse and validate** the workflow structure
   - Detect cycles (reject cyclic graphs)
   - Validate node references in edges
   - Ensure the graph is well-formed

2. **Execute nodes in the correct order**
   - Respect dependencies between nodes
   - Use topological sorting or similar approach
   - Handle the Start and End node types

3. **Support parallel execution**
   - Independent nodes should run concurrently
   - Use `tokio::spawn`, `tokio::join!`, or similar
   - Maximize parallelism while respecting dependencies

4. **Implement node types**
   - `Start`: No-op node, marks workflow beginning
   - `End`: Collects inputs, marks workflow completion
   - `Echo`: Outputs a static message
   - `IfElse`: Evaluates condition, follows matching edge

5. **Handle conditional branching**
   - `IfElse` nodes have a `condition` field ("true" or "false")
   - Only follow edges where `edge.condition` matches the node's output
   - Skip branches that don't match

6. **Return execution results**
   - Populate `node_outputs` map with each node's output
   - Track `execution_order` for debugging
   - Use proper error handling (return `anyhow::Result`)

### Node Behavior Specification

#### Start Node
```rust
NodeType::Start
```
- No inputs required
- Output: `json!(null)` or empty value
- Simply marks the beginning of execution

#### End Node
```rust
NodeType::End
```
- Can have multiple inputs
- Output: `json!(null)` or collects inputs
- Marks completion of workflow

#### Echo Node
```rust
NodeType::Echo { message: String }
```
- No inputs required (inputs can be ignored)
- Output: `json!(message)` - returns the message as a JSON string

#### IfElse Node
```rust
NodeType::IfElse { condition: String }
```
- No inputs required
- Evaluates the condition ("true" or "false")
- Only edges with matching `condition` field should be followed
- Output: `json!(true)` or `json!(false)` based on condition

#### Add Node (Bonus)
```rust
NodeType::Add
```
- Requires inputs from node.inputs field (e.g., ["node1.output", "node2.output"])
- Parses each input reference, retrieves values from previous node outputs
- Converts input values to numbers and sums them
- Output: `json!(sum)` - returns the sum as a number

### Input Resolution (Bonus)

For the `Add` node (and potentially others), you need to resolve input references:

- Input format: `"node_id.output"` (e.g., `"echo1.output"`)
- Parse the node ID from the reference
- Look up the output value from `node_outputs` map
- Convert string values to numbers if needed ("10" -> 10.0)

## Test Requirements

### Basic Tests (Required - Must Pass)

All tests in `tests/basic_tests.rs` must pass:

- ✅ `test_linear_workflow` - Sequential execution
- ✅ `test_parallel_execution` - Concurrent independent nodes
- ✅ `test_if_else_takes_true_branch` - Conditional branching (true)
- ✅ `test_if_else_takes_false_branch` - Conditional branching (false)
- ✅ `test_empty_workflow` - Edge case handling
- ✅ `test_single_node` - Single node execution

### Advanced Tests (Optional - Bonus Points)

Tests in `tests/advanced_tests.rs` demonstrate deeper understanding:

- 🌟 `test_add_node_with_data_flow` - Data passing between nodes
- 🌟 `test_complex_data_flow` - Multi-level data dependencies
- 🌟 `test_cycle_detection` - Graph validation
- 🌟 `test_self_cycle_detection` - Self-reference detection
- 🌟 `test_missing_node_reference` - Error handling
- 🌟 `test_diamond_pattern` - Complex dependency patterns
- 🌟 `test_nested_conditionals` - Nested branching logic
- 🌟 `test_concurrent_execution_timing` - Actual concurrency verification

## Example Workflow

See `examples/` directory for sample workflows. Here's a simple one:

```json
{
  "nodes": {
    "start": {
      "id": "start",
      "node_type": {"type": "Start"},
      "inputs": []
    },
    "greet": {
      "id": "greet",
      "node_type": {"type": "Echo", "message": "Hello, World!"},
      "inputs": []
    },
    "end": {
      "id": "end",
      "node_type": {"type": "End"},
      "inputs": ["greet.output"]
    }
  },
  "edges": [
    {"from": "start", "to": "greet", "condition": null},
    {"from": "greet", "to": "end", "condition": null}
  ]
}
```

## Implementation Tips

### Algorithm Suggestions

1. **Topological Sort**: Use Kahn's algorithm or DFS-based approach
2. **Readiness Tracking**: Maintain a queue of nodes ready to execute
3. **State Management**: Track completed nodes and their outputs
4. **Parallelism**: Use `tokio::spawn` for independent nodes, `join!` for dependencies

### Suggested Structure

```rust
struct ExecutionContext {
    completed: HashSet<String>,
    outputs: HashMap<String, serde_json::Value>,
    // ... other state
}

impl SimpleExecutor {
    async fn execute_node(&self, node: &Node, context: &HashMap<String, Value>) -> Result<Value> {
        // Match on node_type and execute accordingly
    }

    fn find_ready_nodes(&self, workflow: &Workflow, completed: &HashSet<String>) -> Vec<String> {
        // Find nodes whose dependencies are all satisfied
    }

    fn detect_cycles(&self, workflow: &Workflow) -> Result<()> {
        // Validate the graph is acyclic
    }
}
```

## Submission Guidelines

### What to Submit

1. **Your implementation** in `src/lib.rs`
2. **Any additional files** you created (helper modules, etc.)
3. **Test output** showing all basic tests passing:
   ```bash
   cargo test --test basic_tests > test_output.txt
   ```
4. **Brief writeup** (optional but recommended):
   - Your approach and design decisions
   - Challenges you faced
   - Trade-offs you made
   - Ideas for future improvements
   - Estimated time spent

### How to Submit

1. Create a private GitHub repository
2. Push your code
3. Invite `chazzhou` and `kirk-xuhj` as a collaborator
4. Email `charles@tryprofound.com` and `kirk@tryprofound.com` with the repository link

**OR**

1. Create a `.zip` or `.tar.gz` of your project
2. Email to `charles@tryprofound.com` and `kirk@tryprofound.com`

### Deadline

Please submit within **3 days** of receiving this challenge. If you need an extension, just let us know - we're flexible!

## Questions?

If you have questions about:

- **Requirements**: Email us! We want you to succeed.
- **Clarifications**: Reasonable assumptions are fine, just document them.
- **Technical issues**: Make sure Rust and cargo are installed correctly.
- **Time extension**: Just ask, we understand life happens.

## Good Luck!

We're excited to see your solution. Remember:

- **Quality > Speed**: Take the time you need
- **Communication**: Document your thought process; If you used AI, please document your workflow
- **Completeness**: Basic tests must pass, bonus tests are optional
- **Creativity**: We love seeing unique approaches

If you have any questions, don't hesitate to reach out. Happy coding!
