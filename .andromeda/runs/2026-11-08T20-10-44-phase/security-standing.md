# security · standing conventions

- S1 · each new test that records or keeps a check writes it 0600 in a 0700 directory under the test target directory, never commits it, and reads it back only through Check::from_bytes · data-protection
  (security-plan/data-protection fct-6n6morf4) data-rest
- S2 · each new test reaches its directories only through compile-time CARGO_TARGET_TMPDIR or CARGO_MANIFEST_DIR, never a new runtime env read · secret-management
  (security-plan/secret-management fct-cdb7ihdt) secrets
- S3 · each new log line or span field under an escher_ target holds fixed words, bools and counts only and passes the escher_telemetry allowlist · logging-monitoring
  (security-plan/input-validation fct-r5yt6pat) input
  (security-plan/logging-monitoring fct-4hqe6pzr) logging
- S4 · each new or changed CI cargo command passes --locked, and every ci.yml uses: stays pinned to a commit SHA · dependency-security
  (security-plan/dependency-security fct-pq2wmfyf) deps-supply-chain
  (security-plan/dependency-security fct-q5ecmhgy) deps-supply-chain
