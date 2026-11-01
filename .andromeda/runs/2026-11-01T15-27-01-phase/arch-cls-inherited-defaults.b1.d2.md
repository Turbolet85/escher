# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · yes · the publish flags of the root and crate manifests sit in Cargo.toml, reached whole by the take's manifest and lock merge, and the take's pins and any new package must keep the publish and in-repo `=` pin rules
inherited-defaults#2.1 · yes · the scope's proof gates bind the stated gates: `ci-leg.sh fast`, `doc` (rustdoc and the docs job), `a11y`, `audit` (cargo-deny) and fork CI green with every job settled; the fmt and clippy lines are standing S4 and not counted here
inherited-defaults#3.1 · no · standing S6 · tracing gated per call site in the engine crates is the convention for new log lines; the sink check reads the new call sites' targets only, not their gating, and the telemetry crates' ungated features are outside the reached files
