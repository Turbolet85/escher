# security · standing conventions

- S1 · each new test reads no env var but the build-time CARGO_MANIFEST_DIR, CARGO_TARGET_TMPDIR or CARGO_BIN_EXE ones already registered; a new test-only read is registered as test-only before it lands · secret-management
  (security-plan/secret-management fct-r7u7wjon) secrets
  (security-plan/secret-management fct-5k3imw3w) secrets
  (security-plan/secret-management fct-tgxeps3a) secrets
  (security-plan/secret-management fct-s6div4md) secrets
  (security-plan/secret-management fct-ujc6szjz) secrets
  (security-plan/secret-management fct-cdb7ihdt) secrets
  (security-plan/secret-management fct-jphiqah2) secrets
- S2 · each new test that makes a state or recording directory creates it 0700 under the test target directory, with its files 0600 and its path passed as a parameter, and removes only what it created, non-recursively · input-validation
  (security-plan/input-validation fct-doehymzv) input
  (security-plan/input-validation fct-ifbk2u2m) input
- S3 · each new log line or span carries fixed words, bools and counts only, never read content, and passes escher-telemetry's target allowlist with its content-named fields redacted · logging-monitoring
- S4 · each new test's or example's dependency is inherited with workspace = true, no package enters Cargo.lock, and every cargo leg still passes --locked under the cargo-deny audit · dependency-security
  (security-plan/dependency-security fct-a4qh6ioi) deps
  (security-plan/dependency-security fct-n5tq3u6e) deps-supply-chain
  (security-plan/dependency-security fct-q5ecmhgy) deps-supply-chain
- S5 · each committed piece of evidence holds no host path (masked), no typed text, no secret, token or credential-file name, and no kept check beyond the two committed stand-Counter checks · data-protection
  (security-plan/data-protection fct-6n6morf4) data-rest
  (security-plan/logging-monitoring fct-tqdgjkao) logging
  (security-plan/secret-management fct-ov4w6i7j) secrets
  (security-plan/secret-management fct-yffuq3co) secrets
- S6 · each test diagnostic or debug dump on stdout or stderr holds nothing a caller typed or a document carries beyond the masked values · Logging & Monitoring
