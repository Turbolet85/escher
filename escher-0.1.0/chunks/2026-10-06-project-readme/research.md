# Codebase Research — 2026-10-06-project-readme

## Scope
- **Depth:** minimal (one Markdown file; no Rust symbol in the modify set) · **Reads:** 6 (README.md, intent.md, Cargo.toml:240-262, packages/stylo_taffy/Cargo.toml:1-8, justfile:64-65, apps/readme/src/main.rs:206-219 via grep context) · **Globs/Greps:** 5
- **Harness rules consulted:** none — no live leg in this chunk (the gates are the standard `ci-leg.sh fast` and `doc` legs)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository (intent.md, the masters, the working route, git)

## Files inspected
- `README.md` (full, 115 lines) — upstream Blitz's front page: a `<picture>` Blitz logo from `blitz-website.fly.dev`, five upstream badges (dioxuslabs CI, crates.io `blitz`, docs.rs, licence, deps.rs), the Dioxus Discord, Blitz beta status, `blitz.is` links, upstream examples, Goals, Architecture crate list, the git-Dioxus-Native recipe, and a License section (dual Apache-2.0 / MIT; `stylo_taffy` additionally MPL-2.0; contribution licensing). The whole file is replaced.
- `escher-0.1.0/intent.md` (full) — the source for "what" (opening paragraph), "why" (§Principles "The API explains itself"; §Findings 1–9), lineage (opening: fork of DioxusLabs/blitz adopted at `0f60502e`; §Principles "Upstream stays in reach": an "Upstream sync" chunk at each epoch boundary), and "beyond 0.1.0" (§Out of this version).
- `Cargo.toml` (36-38, 245-254) — workspace `license = "MIT OR Apache-2.0"`; `homepage`/`repository` still `https://github.com/dioxuslabs/blitz` (out of this chunk — "No other file changes"); the root `[package]` `blitz-examples` is `publish = false` with no `readme =` field.
- `packages/stylo_taffy/Cargo.toml` (3) — `license = "MIT OR Apache-2.0 OR MPL-2.0"`: the MPL-2.0 triple licence the current README states is real (arch's extract found it unrecorded in arch §Conventions; the manifest settles it).
- `LICENSE-MIT`, `LICENSE-APACHE` (heads) — no named copyright holder in either (`grep -n -i copyright LICENSE-APACHE` hits only the licence's own definitions; LICENSE-MIT opens at "Permission is hereby granted"). Upstream's credit is therefore an attribution line, not a copyright line to preserve.
- `justfile` (64-65) — `just seven_guis` runs `cargo run --release --package seven_guis --bin seven_guis_native`.
- `apps/readme/src/main.rs` (206-219) — `rdme` resolves `README.md` at RUNTIME from a directory argument (current dir or a parent); no compile-time read.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- not queried — the modify set is one Markdown file and no Rust symbol is named, changed or called; the change surface lies outside the indexed rust plane (graph not applicable); the consumer question was answered by the name sweep below.

## Patterns detected
- **No compile-time README consumer** (sweep): `grep -rn -i readme` over `*.rs *.toml *.py *.sh *.yml *.yaml justfile`, excluding `target/` and `wpt/wpt/`: 18 hits · 0 changed · 18 no-change (17 are `apps/readme` identifiers or comments — `ReadmeApplication`, `ReadmeEvent`, `readme_application` module — plus `Cargo.toml:20` `"apps/readme"` workspace member; the one path read is runtime, main.rs:212). No `include_str!` of a README, no Cargo `readme =` field, and 0 hits in `.github/scripts/*.py` (`grep -c -i readme .github/scripts/*.py` → 0 in all five). So neither the `doc` leg nor the `ci-scripts` leg reads the root README: the rewrite cannot move their verdicts or counts.
- **Upstream README churn** — `git log --oneline 23354585..upstream/main -- README.md` → 0 commits (upstream/main as fetched, `2335458` at 2026-10-06T01:33Z, is the merge base itself); `git log --oneline --since=2025-10-06 upstream/main -- README.md | wc -l` → 6. So upstream edits its README roughly every two months; a wholesale rewrite meets a conflict at some, not every, sync.
- **The fork's public face** — `gh repo view Turbolet85/escher --json visibility,defaultBranchRef,description` → `PUBLIC`, default branch `build/escher-0.1.0`, description "An agent-first UI framework built on Blitz". The README on the default branch IS the repository front page; a fork CI badge (`ci.yml`, `name: CI` at `.github/workflows/ci.yml:1`) would render that branch's status.

## Conventions to follow
- **Facts with a source**: every capability the README calls built maps to a `complete` master record and a `verified` matrix entry (`matrix.py coverage --dir escher-0.1.0` → verified 2/15: v010-01, v010-02); everything else is worded as planned (arch/tests/security/obs/a11y extracts concur).
- **Licence statement**: workspace `MIT OR Apache-2.0` (Cargo.toml:36), `stylo_taffy` `MIT OR Apache-2.0 OR MPL-2.0` (packages/stylo_taffy/Cargo.toml:3); the LICENSE files stay untouched.
- **Epoch one-liners**: the six `### Epoch K — {name}` headers of `escher-0.1.0/working-route.md` (`route.py epoch --root .`: lines 10, 25, 36, 47, 62, 73).

## New files to create
- none

## Files to modify
- `README.md` — rewritten whole for escher

## Open questions
- Does the README carry anything beyond the entry's five parts — a CI badge for the fork, a short "Try it" (`just seven_guis`; `bash scripts/agent-run.sh boot` then `run stand`)? → blocks: plan-decision
