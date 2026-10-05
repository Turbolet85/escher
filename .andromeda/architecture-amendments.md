# architecture — amendments

One entry per amendment to `architecture.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-as-built-baseline — the rustdoc doc gate reaches no library crate
**Section:** §Stack and Technologies (Code quality row) · §Conventions (Formatting and lints) · §Infrastructure Patterns (Build system · CI/CD) · §Inherited Defaults (Code quality)
**Change:** was "Formatting, lint and doc gates" / "rustdoc warnings are errors" / "fmt, clippy and rustdoc gates" — a CI rustdoc `-D warnings` gate over the workspace; now `RUSTDOCFLAGS: "-D warnings"` is set workflow-wide but CI's docs job runs bare `cargo doc`, which documents only the lib-less root package `blitz-examples`, so no library crate's rustdoc is gated; `cargo doc --workspace --no-deps` under `-D warnings` fails — 3 crates (blitz-dom, blitz-vibey-script, example transparent), 9 errors — as measured at the chunk's `evidence/baseline.md`.
**Why:** the as-built baseline measured the docs job documenting nothing and the workspace form red; making the gate real and green is owned by the "CI gate legs" route entry (named at the P5 review on the founder's word, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/

## 2026-10-05-as-built-baseline — measured dev-host build environment
**Section:** §Stack and Technologies (Build environment row)
**Change:** the row now states that the flake's rust-bin "1.90.0" pin sits below the workspace `rust-version` "1.91.0" its own comment says to keep in sync with, and carries the dev-host reading as measured at the chunk's `evidence/baseline.md` (Omarchy 4.0.4, Arch-based, 2026-10-05): the flake is not used; host stable rustc/cargo 1.99.0; no `rust-toolchain*` file, no `.cargo/config*`; Arch packages fontconfig 2.18.3, openssl 3.6.4, pkgconf 3.0.7, python 3.14.7 stand in for CI's `libfontconfig1-dev` and build-time python3.
**Why:** the baseline is the first verified dev-host build; host-tool versions are readings on that host, not pins — no lockfile resolves them.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/
