# security · standing conventions

- S1 · A new log site in engine code sits behind the crate's `tracing` feature with a no-op fallback, prints only safe fields and adds no user-content field · .andromeda/security-plan-detail/logging-monitoring.md
- S2 · A new dependency is reviewed by hand and run through the audit's advisory and licence checks. An advisory is fixed by a semver-compatible update or ignored by ID with a written reason, never a blanket allow · .andromeda/security-plan-detail/dependency-security.md
- S3 · No secret in code, fixtures, logs or committed evidence · .andromeda/security-plan-detail/secret-management.md
  (.andromeda/security-plan-detail/secret-management.md:21) secrets
  (.andromeda/security-plan-detail/secret-management.md:23) secrets
  (.andromeda/security-plan-detail/secret-management.md:27) secrets
  (.andromeda/security-plan-detail/secret-management.md:31) secrets
  (.andromeda/security-plan-detail/secret-management.md:32) secrets
  (.andromeda/security-plan-detail/secret-management.md:33) secrets
- S4 · A new env read, a test-only one included, is registered in the environment-values list before it lands · .andromeda/security-plan-detail/secret-management.md
- S5 · A new input path keeps parse failures as typed `Result`s, never panics · .andromeda/security-plan-detail/input-validation.md
- S6 · A test name or assertion message holds no element id, accessible name or control value · .andromeda/security-plan-detail/input-validation.md
