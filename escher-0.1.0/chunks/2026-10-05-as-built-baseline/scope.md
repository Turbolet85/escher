# Scope — As-built baseline

**Marker:** 2026-10-05-as-built-baseline · **Version:** escher-0.1.0 · **Epoch:** Epoch 1 — Foundation
**Working entry (verbatim):** As-built baseline — workspace build and blitz-tests green on this host, one run's wall-clock recorded

## What this chunk builds
A measured starting point for the fork: proof that the adopted Blitz workspace, as it stands at take-up, builds and
passes its integration tests on THIS host, with the run's cost on record. It is the first rung of the version's
"infrastructure ahead of the queue" order (intent.md §Principles: "the as-built baseline green on this host and the
fork's CI reached → …"); every later chunk's gate reads against it.

- The whole workspace builds on this host: `cargo build --workspace --locked`, dev profile. CI runs `cargo build
  --workspace` (ci.yml:36, :46) after an `opt-level = 2 → 0` rewrite that touches only `[profile.p2]` (Cargo.toml:210,
  the sole hit), so the dev-profile form equals CI's; `--locked` is added per the security rule (CI passes none).
  (verified — research §Patterns detected)
- The `blitz-tests` integration suite (`tests/blitz-tests/`, 59 files, one per behaviour) runs green on this host.
- One run's wall-clock is recorded per step — build and blitz-tests each, with a stated cold/warm basis — in the
  chunk's evidence folder. [premise-corrected: `target/` held 1.4G but only `target/debug/build/` script outputs from
  setup's code-graph indexing, no compiled crate — a first build is cold for compilation, not from-empty; the basis
  (clean first or not) is a P4 decision]
- The host's facts that make the run reproducible are recorded beside the figure: Omarchy 4.0.4 (Arch-based), kernel
  7.2.5, rustc/cargo 1.99.0 host stable (no rust-toolchain file; MSRV 1.91.0), the Arch stand-ins for CI's
  `libfontconfig1-dev` — `fontconfig 2.18.3`, `openssl 3.6.4`, `pkgconf 3.0.7`, `python 3.14.7` — and the fonts
  fontconfig resolves. (verified — research §Host facts measured)

## Boundaries
- NOT CI: reaching the fork's CI is the next entry ("Fork CI reached"). This chunk runs on the local host only.
- NOT new tooling: no nextest, coverage, audit or harness install — those belong to later Foundation entries
  ("CI gate legs", "Stand test contract", which owns `scripts/agent-run.*`).
- No product change intended. A red found here is first RECORDED as the baseline's reading; whether it is fixed in
  this chunk (a host-local cause) or pinned forward (an owner named) is P4's decision. (a P4 decision, not a
  premise — research found nothing that predicts a red: no network in the suite, fonts present, no windowing dep)
- The wider workspace test leg (`cargo test --workspace`, the CI test leg) and the lint gates (`cargo fmt --all
  --check`, `cargo clippy --workspace -- -D warnings`, rustdoc `-D warnings` — CLAUDE.md's done-gates) are not named
  by the entry; they are measured here as RECORDED readings (report-only, never counted green) — resolved at P4 on
  the operator's word "Record them" (overseer: "the baseline states where every done-gate stands before any code
  moves"); the build figure is from-empty (`cargo clean` first, same round). (val-1: intent-incomplete, amended)
- Font-dependent assertions are real only if fonts load: the suite's one runtime font skip is
  `text_selection_anonymous_block.rs:107/:130` (`skipping: no usable font`), visible only under `--nocapture`; the 3
  `#[ignore]` tests are all `paint_tree_bench` and stay out of the timed run. (research §Patterns detected)
- Incremental and non-incremental layout tests (`for incremental in [false, true]`) run as the suite has them; the
  chunk adds no test.

## Surfaces / contracts touched
- No source crate. Artifacts: the chunk folder (`chunks/2026-10-05-as-built-baseline/`) — evidence of the run.
- Reads: `Cargo.toml` / `Cargo.lock` (pins, coupled dependency families — never bumped here), `tests/blitz-tests/`.

## CI verdict at take-up (Setup 5a)
- `eaca28ba` (HEAD; no master flip committed yet, so HEAD alone): verdict `none recorded` · checks 0/0 · runs 0 ·
  active workflows 0 — the fork has no active workflow yet (the "Fork CI reached" entry's subject). No wall-clock: no
  run. Not a red; nothing to disposition.

## Freight folded
- `route.py pins`: no freight block on any markerless entry — nothing to fold.
