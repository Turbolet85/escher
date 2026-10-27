# tests · standing conventions

- S1 · A stand check reads the tables and helpers it shares from `tests/common/mod.rs` and `tests/session_common/mod.rs`, declared after its `use` block. Those tables are stated expectation, never a second mapping, and a new reader moves that module's recorded reader count · §3 Test Harness Contract (blitz-tests stand checks · session checks)
- S2 · A unit-test module moved out of line sits in a file of its own, declared `#[cfg(test)]` with `#[path]` at its parent, its test paths unchanged · §4 Unit Test Strategy, Conventions (unit-test-strategy.md)
  (.andromeda/test-plan-detail/unit-test-strategy.md:8) unit
- S3 · Assertions over a call carry a row index only, so a failing log prints no argument value · §4 Unit Test Strategy, escher-driver (unit-test-strategy.md)
- S4 · blitz-tests keeps one test-target file per behaviour, pinned by `BlitzTestsTargetsTest`, with the shared modules no target · §2 Test Strategy, Directory pattern (test-strategy.md)
- S5 · Session-held stand checks run in both layout modes, with no sleep and no wall-clock read in a check · §5 Integration Test Strategy (integration-test-strategy.md)
- S6 · A green per-package run is not the workspace leg's green, so a check is read under both builds · §9 CI Integration, Engine features by runner (ci-integration.md)
