@carried n=1 label=security-plan/dependency-security fact=fct-a4qh6ioi
why: The probe's tree, including any probe check or example it keeps, must pass the audit leg over the whole workspace's graph, advisories and licences both.
quote: `cargo deny --locked --workspace --all-features check advisories licenses` against the RustSec advisory database and the licence table, over the whole workspace's graph
@carried n=2 label=security-plan/dependency-security fact=fct-a4qh6ioi
why: The chunk must not add a blanket allow or an unreasoned ignore to `deny.toml`; if a new advisory turns the audit red, the only acceptable ignore is one per ID with a written reason.
quote: `[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults (deny.toml:1-29; .github/scripts/ci-leg.sh:40).
@carried n=3 label=security-plan/dependency-security fact=fct-7aitszps
why: The probe reads the Dioxus runtime with no fork-level change, so the standing pins must hold unchanged at the chunk's end: Dioxus stays "0.7.3" and no git dependency enters.
quote: the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency;
@carried n=4 label=security-plan fact=fct-hhqzsgtc
why: No patch, vendored copy or git source of a Dioxus crate may enter the root manifest to expose a link; such a probe stops at the verdict «not exposed without a fork-level change».
quote: No git dependency is left in the root manifest
@carried n=5 label=security-plan/dependency-security fact=fct-zrg6urnw
why: If the probe's test or example code brings any package into `Cargo.lock`, that package must be reviewed by hand the way earlier entries were before the chunk closes.
quote: each from crates.io with a checksum and reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, its source beyond the manifest not read
@carried n=6 label=security-plan fact=fct-wn5xussh
why: A probe check that lives in `tests/blitz-tests/` must take any dev-dependency it needs with `workspace = true`, never as an inline version.
quote: every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-42)
@carried n=7 label=security-plan fact=fct-ehhtk64b
why: The probe's measuring runs and any check it keeps must build and pass under `--locked`, the way every ci.yml cargo leg does.
quote: and so does every ci.yml cargo leg — the leg script's build, test, clippy (both lines), doc, msrv, counter, wasm, audit, a11y, coverage and, since 2026-10-14-quality-gates, stand-repeat commands and the matrix command
@carried n=8 label=security-plan fact=fct-6cupfjou
why: The chunk's pushed HEAD must read green on the Dependency audit job.
quote: ci.yml's `audit` job ("Dependency audit") runs `bash .github/scripts/ci-leg.sh audit` on every pull request and push
@binding n=1
**Dependency pins ↔ the chunk scope's boundary «No fork-level change»**: the scope requires Dioxus to stay at 0.7.3 with no patch, vendored copy or pin move. On this category's side, the standing pins (security-plan/dependency-security fct-7aitszps) and the absence of any git dependency (security-plan fct-hhqzsgtc) must still hold at the chunk's end, unchanged. If a link can only be exposed by breaking them, the probe returns the verdict «not exposed without a fork-level change».
@binding n=2
**Audit leg ↔ the body's CI integration and the tests plan's CI gate**: the whole-workspace `advisories licenses` check (security-plan/dependency-security fct-a4qh6ioi) runs as ci.yml's `audit` job (security-plan fct-6cupfjou). That job is a required check for the chunk. Any ignore it needs must be per ID with a written reason (security-plan/dependency-security fct-a4qh6ioi).
@binding n=3
**New probe dependency ↔ the chunk scope's surfaces `tests/blitz-tests/` and `examples/seven_guis/`**: probe code kept there must take its dependencies through the workspace (security-plan fct-wn5xussh) and build `--locked` (security-plan fct-ehhtk64b). Any package it adds to `Cargo.lock` must be reviewed by hand (security-plan/dependency-security fct-zrg6urnw).
