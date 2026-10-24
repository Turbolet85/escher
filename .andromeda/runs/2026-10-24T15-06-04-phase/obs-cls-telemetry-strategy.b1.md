# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · yes · the command and tool_call spans and escher-session's sink, which host_log and mcp_log read at RUST_LOG=trace for the lint, part and recording verbs
telemetry-strategy#2.1 · yes · the workspace build turns blitz-dom/tracing on so engine lines reach the sink and the -p builds leave it off, the two builds the measurement runs under
telemetry-strategy#3.1 · yes · tests/blitz-tests/Cargo.toml and the reached .github/scripts/*.py lie inside its search, so no telemetry backend may enter them
telemetry-strategy#3.2 · yes · the reached host_log.rs, mcp_log.rs and seven_guis tests/common/mod.rs sit in the 32-file examples slice, which must keep no tracing or log use
telemetry-strategy#3.3 · yes · the reached stand_lint.rs, text_extent.rs and shared test modules sit in the 61-file slice, whose silence the sweep's checks must keep
telemetry-strategy#3.4 · no · the chunk reaches no wpt/runner file, and the WPT runner's logging is out of the sweep (no 0.2.0 witness runs it)
