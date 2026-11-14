# security · standing conventions

- S1 · each new test owes the per-crate line coverage floors of the crates it lands in · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
- S2 · each new log line or span field holds no content and passes the escher target scrub and the sink's target allowlist · logging-monitoring
  decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
  decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
  decision/other-log-targets-dropped · Other log targets dropped whole — In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
  decision/outside-target-dropped · Outside targets dropped whole — An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names. · why: The named cost: a third-party WARN or ERROR no longer prints either.
  decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
- S3 · each new dependency is workspace-inherited, locked, licence-gated and audited by cargo-deny · dependency-security
  decision/local-audit-before-push · Run the audit leg locally before a push — The local fast leg runs no audit, so the audit leg is run locally before a push. · why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
- S4 · each new refusal is a cause of the closed refusal set with its fixed meaning and remedy · error-handling
  contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
  contract/refusal-causes/disabled · Driver refusal causes · disabled — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
  contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
  contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
