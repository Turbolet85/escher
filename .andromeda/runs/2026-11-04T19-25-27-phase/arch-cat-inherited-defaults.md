@carried n=1 label=architecture fact=fct-656exhoj
why: The probe must read the runtime as pinned: Dioxus stays at 0.7.3 with no pin move, patch or vendored copy, and a link that needs a different Dioxus is reported as «not exposed without a fork-level change».
quote: Dioxus "0.7.3" (Cargo.toml:85-102)
@carried n=2 label=architecture/inherited-defaults fact=fct-77ljaxug
why: Any probe code kept in `tests/blitz-tests/`, `examples/seven_guis/` or `packages/dioxus-native-dom/` must pass the inherited quality gates (fmt, clippy, the blitz-dom clippy line, rustdoc, cargo-deny, the coverage floors) and keep the `stand-repeat` leg's zero-failure budget.
quote: `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and, since 2026-10-14-quality-gates, upstream's blitz-dom clippy line under `--no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets` (all run through `.github/scripts/ci-leg.sh`), rustdoc `-D warnings` set workflow-wide and enforced over every workspace library crate — the docs job runs `cargo doc --workspace --no-deps --locked`; a cargo-deny audit of the advisories and licences of the whole workspace and a cargo-llvm-cov coverage report gated by per-crate floors, with the `stand-repeat` leg's zero-failure budget beside them, all legs of the same script
@carried n=3 label=architecture/inherited-defaults fact=fct-fgkqqy76
why: An escher-side change in `dioxus-native-dom`, if the probe needs one, must keep tracing gated per call site in the engine crates and add no span or log field, so that nothing the probe reads gets logged.
quote: Optional capabilities: Cargo features, tracing gated per call site in the engine crates — escher-telemetry and escher-driver (its `command` span and, since 2026-10-10-mcp-surface, its `tool_call` span) are ungated
@binding n=1
**Dioxus pin ↔ the body's Upstream sync line**: the inherited Dioxus "0.7.3" (architecture fct-656exhoj) is the version the probe measures against. Dioxus 0.8.0-alpha.3 waits for escher's next version, so no link verdict may rest on 0.8 behaviour, and moving the pin counts as a fork-level change (architecture fct-7lmrueug).
@binding n=2
**Code quality ↔ the body's Conventions (Tests)**: a probe check kept under `tests/blitz-tests/` must pass the inherited gates (architecture/inherited-defaults fct-77ljaxug). It must also follow the test conventions: one behaviour per file, a `//!` doc, both layout modes, and asserting first that the fixture produces the condition (architecture fct-wqndq65v).
@binding n=3
**Optional capabilities ↔ the body's Logging and timing and Dioxus DOM bridge contract**: tracing stays gated per call site in the engine crates (architecture/inherited-defaults fct-fgkqqy76). The bridge reads the screen read-only and logs no id, name or value, so a probe read of a component, handler or signal adds no span or field (architecture fct-srb7slak).
