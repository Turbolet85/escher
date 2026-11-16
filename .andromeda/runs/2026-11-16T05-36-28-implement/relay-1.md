# Relay 1 — the operator's answer on the log capture's levels (2026-11-16, this /implement run, P1)

**Asked (AskUserQuestion, verbatim):** Plan conflict in «Errors with the action» (the log capture): the plan counts log records at every level from every target outside the engine and escher_ sets. But dioxus-core, dioxus-signals and generational-box emit trace! records on every ordinary re-render and signal write in debug builds (e.g. dioxus-core diff/node.rs:87, dioxus-signals signal.rs:531). So a plain Counter click would get a `caused` trace item. That breaks the plan's own Q2 proof ("the count click has no caused key") and moves the committed Counter checks and the cross-OS digests. How should the capture count?

**Options offered:** «Framework at warn+ (Recommended)» — Records from the Dioxus framework's crates (prefix `dioxus`, `generational_box`) count only at warn and error. Every other non-engine, non-escher target (the app's own crate) counts at every level. · «Every level, as planned» · «Stop: surface, re-plan».

**Answer (verbatim):** "Framework at warn+ (Recommended)"
