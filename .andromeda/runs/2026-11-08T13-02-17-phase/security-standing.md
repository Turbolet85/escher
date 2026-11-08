# security · standing conventions

- S1 · each new escher-driver test or code path reads no env var but test-only build-time or TMPDIR sites registered as test-only, and names no secret · Secret Management
  (security-plan/secret-management fct-tbouybgj) secrets
  (security-plan/secret-management fct-r7u7wjon) secrets
  (security-plan/secret-management fct-q25a2pwc) secrets
  (security-plan/secret-management fct-5k3imw3w) secrets
  (security-plan/secret-management fct-tgxeps3a) secrets
  (security-plan/secret-management fct-s6div4md) secrets
  (security-plan/secret-management fct-hoy5axpi) secrets
  (security-plan/secret-management fct-ujc6szjz) secrets
  (security-plan/secret-management fct-cdb7ihdt) secrets
  (security-plan/secret-management fct-jphiqah2) secrets
  (security-plan/secret-management fct-trkab6m3) secrets
  (security-plan/secret-management fct-oge3aocv) secrets
- S2 · each new log line or span field holds fixed words, bools and counts only, never a call's content, screen text or a path, inside the escher_telemetry allowlist sink · logging-monitoring
- S3 · each new fixture or test leaves no package entering Cargo.lock and builds under --locked · dependency-security
  (security-plan/dependency-security fct-q5ecmhgy) deps-supply-chain
- S4 · each new kept check or recorded fixture stays owner-only (0600 in a 0700 directory) and is never committed · data-protection
- S5 · each new test's panics on stale ids are guarded and code under test never panics on call-supplied input · error-handling
