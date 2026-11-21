# obs · standing conventions

- S1 · every new test or fixture is silent: it prints nothing on success, and a check that asserts empty stderr unsets RUST_LOG · 3-observability-harness-contract
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S2 · every new engine log site writes a fixed message, prints safe fields only and carries no page content · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
  contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
  contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
  decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
  decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
- S3 · tracing in engine and upstream crates compiles only under the tracing cargo feature · 2-telemetry-strategy · telemetry-strategy
