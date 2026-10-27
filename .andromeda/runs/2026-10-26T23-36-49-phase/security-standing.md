# security · standing conventions

- S1 · A test name, an assertion message or a `Refusal` / `Fault` holds no element id, accessible name, control value or argument name a call supplied · input-validation.md
- S2 · escher-driver code has no `unwrap`, `expect` or panic path on anything a call supplies, and no print, `unwrap`, `expect` or `panic!` outside the tests on the MCP path · input-validation.md
- S3 · escher's own crates log only through `tracing` spans or events with fixed words, bools and counts. A driver record never carries a call's content, the screen text, an image or findings, and the crate installs no subscriber · logging-monitoring.md
  (.andromeda/security-plan-detail/input-validation.md:202) input
  (.andromeda/security-plan-detail/logging-monitoring.md:21) logging
- S4 · A listener or env read that only a test makes is registered as test-only (arch §Occupied Resources and the security plan's environment values) before it lands · secret-management.md
- S5 · Any new crate passes the cargo-deny audit (advisories and licences over the whole workspace's graph), each ignore only by ID with a written reason, and builds stay `--locked` · dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:8) deps
  (.andromeda/security-plan-detail/dependency-security.md:17) deps
  (.andromeda/security-plan-detail/dependency-security.md:18) deps
  (.andromeda/security-plan-detail/dependency-security.md:19) deps
  (.andromeda/security-plan-detail/dependency-security.md:20) deps
  (.andromeda/security-plan-detail/dependency-security.md:21) deps
  (.andromeda/security-plan-detail/dependency-security.md:22) deps
  (.andromeda/security-plan-detail/dependency-security.md:34) dep-png
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
