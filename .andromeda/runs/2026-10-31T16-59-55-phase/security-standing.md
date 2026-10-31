# security · standing conventions

- S1 · A new log line or span in an engine or `escher_` target holds fixed words, bools and counts, never option text or an attribute's text, and passes `escher_telemetry`'s target allowlist · Logging & Monitoring, .andromeda/security-plan-detail/logging-monitoring.md
  (.andromeda/security-plan-detail/logging-monitoring.md:8) logging
- S2 · A new test or test file that reads an env var or the temp directory is recorded in the environment values list, as test-only and not secret · Secret Management, .andromeda/security-plan-detail/secret-management.md
- S3 · A new check a test keeps stays owner-only and uncommitted, the two committed stand-Counter checks being the one exception · Data Protection, .andromeda/security-plan-detail/data-protection.md
  (.andromeda/security-plan-detail/logging-monitoring.md:21) logging
- S4 · Any dependency a test or fixture would take inherits `workspace = true` and enters no package to `Cargo.lock` without the audit leg's `--locked` read · Dependency Security, .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:8) deps
  (.andromeda/security-plan-detail/dependency-security.md:18) deps
  (.andromeda/security-plan-detail/dependency-security.md:19) deps
  (.andromeda/security-plan-detail/dependency-security.md:20) deps
