# obs · standing conventions

- S1 · every new check or test is silent on success and a failing log check names a kind and a count, never content · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S2 · every new check that asserts an empty stderr unsets RUST_LOG for the process it runs · 3-observability-harness-contract
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · every new engine log line is compiled only with the tracing cargo feature, and escher's own crates take tracing ungated · 2-telemetry-strategy · telemetry-strategy
  decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
  decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
- S4 · every new log line on an engine target prints safe fields only and every escher target redacts the content-named set · 8-pii-scrubbing-compliance · pii-scrubbing-compliance
