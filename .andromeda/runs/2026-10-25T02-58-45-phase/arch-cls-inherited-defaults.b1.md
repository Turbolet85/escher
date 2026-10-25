# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · no · the publish flags and the workspace version are manifest facts the chunk moves nothing in, and the statement names no crate's publication (Not needed: manifests, release versioning)
inherited-defaults#2.1 · no · the fmt, clippy, rustdoc, audit and coverage gates check Rust code the chunk does not touch (Not needed: formatting and lints, documentation comments), no gate reads README.md, and the a11y reach's CI witnesses come from the CI contracts, not this default
inherited-defaults#3.1 · no · Cargo feature and tracing gating are code conventions and telemetry the chunk does not touch (Not needed: feature gating, default features, the telemetry bootstrap), and the statement names neither a feature gate nor a span
