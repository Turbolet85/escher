# Classified — architecture · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · no · every crate manifest is byte-identical to the base and no crate was added or retired, so each publish flag and the workspace-version pins stand as read
inherited-defaults#2.1 · no · the CI scripts are unchanged and the new target rides the workspace leg with no script change, so each named gate stands as read
inherited-defaults#3.1 · no · the uncaught-JS-error tracing sites moved to runtime/events.rs:21 and stay feature-gated, no new log site or feature was added and escher-telemetry and escher-driver have empty diffs, so the gating stands as read
