# arch · picked accounting · group 2 of 7

## Picked
conventions#1.1 · carried · the scope names the `fast` and `doc` legs (fmt, clippy `-D warnings`, rustdoc `-D warnings`) as the gates the refactor re-asserts green, and the task re-reads `ci-leg.sh` for them
conventions#6.1 · not in scope · the chunk adds no tracing call site, span, span field or cargo feature; escher-driver's hand-built `command` and `tool_call` spans live in `execute.rs` and `mcp.rs`, which the chunk does not reach
conventions#8.1 · carried · the entity records `mcp_tests.rs` as `mcp.rs`'s `#[path]` child keeping its test paths, `stand_requirement_sweep` among `session_common`'s readers and one blitz-tests target per file pinned by `test_blitz_tests_targets.py` — the files the chunk edits and the pin it re-reads — and states the `#[track_caller]` assertion helpers the scope's named-helper factoring introduces
