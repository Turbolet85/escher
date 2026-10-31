# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the driver CLI reads the snapshot text with its option bounds (snapshot.rs edits them), the `help` line equal to describe() that carries the edited covered schema text (schema.rs), and the hosted `lint` line (lint.rs overlaps exclusion)
e2e-test-strategy#1.2 · yes · the MCP `lint` tool call answering its findings reaches the lint.rs overlaps exclusion, and `help` through MCP carries the edited schema text; stand_mcp_tools reads that lint through the harness in both layout modes
e2e-test-strategy#2.1 · no · Cells' MountedData calls from a task spawned in a key handler are a Cells path that the chunk's select keys and snapshot bounds do not call, and events.rs:162 is not a reached file
e2e-test-strategy#3.1 · no · the chunk adds no assertion to the example fixtures, which the task's Not needed drops as holding no duty
e2e-test-strategy#3.2 · yes · the chunk's windowed-path claim for dioxus-native is read at the engine and through the harness because no test launches a window, so the no-window absence is what that claim rests on
