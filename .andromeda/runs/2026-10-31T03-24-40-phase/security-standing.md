# security · standing conventions

- S1 · A new test or fixture reads no environment variable outside the plan's registered list, and a new test-only read is registered there · ## Secret Management
  (.andromeda/security-plan-detail/secret-management.md:25) secrets
  (.andromeda/security-plan-detail/secret-management.md:26) secrets
  (.andromeda/security-plan-detail/secret-management.md:28) secrets
  (.andromeda/security-plan-detail/secret-management.md:29) secrets
- S2 · A new test or fixture holds no secret, credential or host path · .andromeda/security-plan-detail/secret-management.md
  (.andromeda/security-plan-detail/secret-management.md:14) secrets
  (.andromeda/security-plan-detail/secret-management.md:21) secrets
  (.andromeda/security-plan-detail/secret-management.md:22) secrets
  (.andromeda/security-plan-detail/secret-management.md:23) secrets
  (.andromeda/security-plan-detail/secret-management.md:27) secrets
  (.andromeda/security-plan-detail/secret-management.md:30) secrets
  (.andromeda/security-plan-detail/secret-management.md:31) secrets
  (.andromeda/security-plan-detail/secret-management.md:32) secrets
- S3 · No package enters `Cargo.lock` and no dependency is bumped; every build stays `--locked` and the audit leg stays green over the graph · ## Dependency Security
  (.andromeda/security-plan-detail/dependency-security.md:8) deps
  (.andromeda/security-plan-detail/dependency-security.md:17) deps
  (.andromeda/security-plan-detail/dependency-security.md:18) deps
  (.andromeda/security-plan-detail/dependency-security.md:19) deps
  (.andromeda/security-plan-detail/dependency-security.md:20) deps
  (.andromeda/security-plan-detail/dependency-security.md:21) deps
  (.andromeda/security-plan-detail/dependency-security.md:22) deps
  (.andromeda/security-plan-detail/dependency-security.md:23) deps
  (.andromeda/security-plan-detail/dependency-security.md:29) dep-serde-json
  (.andromeda/security-plan-detail/dependency-security.md:35) dep-png
  (.andromeda/security-plan-detail/dependency-security.md:49) deps-supply-chain
- S4 · A new engine log line goes through `tracing` behind its feature and holds no attribute text, value or other page content; a new driver line holds fixed words, bools and counts only · ## Logging & Monitoring
- S5 · New driver or engine code has no `unwrap`, `expect` or `panic!` on anything a call or a document supplies · ## Error Handling
  (.andromeda/security-plan-detail/input-validation.md:81) input
- S6 · A new fixture is no kept check of a session beyond the two committed stand-Counter checks, which alone leave the owner-only medium · ## Data Protection
