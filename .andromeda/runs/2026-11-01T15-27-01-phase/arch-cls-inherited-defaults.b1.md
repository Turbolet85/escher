# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · yes · the root Cargo.toml is a conflict surface, so its virtual-root and in-repo `=` pin lines (Cargo.toml:44-60, 252-263) bind the take's manifest resolution; the publish flags of blitz-tests and blitz-test-harness bind where upstream's test files join blitz-tests
inherited-defaults#2.1 · yes · the scope's proof gates run the `fast`, `doc`, `a11y` and `audit` legs of ci-leg.sh with fork CI green, so the leg runner, the docs job and the cargo-deny audit bind; the fmt, clippy and rustdoc lines stay standing S4
inherited-defaults#3.1 · no · standing S6 · a new log line in the engine crates is gated per call site with a `cfg(not(feature = "tracing"))` fallback
