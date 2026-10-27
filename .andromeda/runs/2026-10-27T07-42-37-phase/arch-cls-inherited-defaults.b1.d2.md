# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · yes · Cargo.toml is a merge surface, so its workspace version, in-repo `=` pins (44-60) and virtual root (252-263) can move under a merge, and the manifests are re-read for any members change or new package
inherited-defaults#2.1 · yes · the fast, doc and audit legs the scope re-runs (ci-leg.sh re-read) are the commands it states: fmt, clippy -D warnings, rustdoc -D warnings, cargo doc --locked, cargo-deny
inherited-defaults#3.1 · no · standing S5 · engine-crate tracing gated per call site with a cfg(not(feature = "tracing")) fallback
