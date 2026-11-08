# security · standing conventions

- S1 · Each new test's environment read (CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR, TMPDIR) stays test-only and is recorded in the plan's list of environment values read. The library and the escher-session binary read no env var beyond RUST_LOG. · secret-management
  (security-plan/logging-monitoring fct-4cylhur3) logging
  (security-plan/secret-management fct-r7u7wjon) secrets
  (security-plan/secret-management fct-tgxeps3a) secrets
  (security-plan/secret-management fct-s6div4md) secrets
  (security-plan/secret-management fct-ujc6szjz) secrets
  (security-plan/secret-management fct-cdb7ihdt) secrets
  (security-plan/secret-management fct-jphiqah2) secrets
- S2 · Each new log line or span field on an escher target holds fixed words, bools and counts only, never content, and passes the escher_telemetry allowlist. The crate installs no subscriber. · logging-monitoring
  (security-plan/input-validation fct-t4jwjgu4) input
- S3 · Each new refusal or session error is written from a fixed table with no path and no caller-supplied text. · error-handling
- S4 · Each new verb, argument kind or MCP tool goes through validate, with typed arguments and bounded requests and results, on every carrier: the socket, the command line and MCP. · input-validation
- S5 · Each new test fixture or committed check holds no typed text, host path, name or secret. A check of any session other than the committed stand-Counter sessions stays owner-only and is never committed. · data-protection
