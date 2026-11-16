# Outcome — 2026-11-16-errors-with-the-action

**Outcome:** green (2 fix-loop iterations) · smoke: `host_log` / `mcp_log` ran as P2 entry 7, green (the plan's smoke stand-in) · controls: 4 of 4 hold
**Run:** 2026-11-16T05-36-28-implement
**Next:** the OPERATOR PASS — entries 11 (`gate.py hygiene`), 12 (the push) and 13 (`ci.py conclusion`), then `/andromeda-wrap-session`.

## Files changed
- New: `packages/escher-telemetry/src/capture.rs` · `packages/dioxus-native-dom/src/boundary.rs` · `packages/escher-driver/src/caused.rs` · `tests/blitz-tests/tests/stand_act_errors.rs`
- Modified: `packages/escher-telemetry/src/lib.rs` · `packages/dioxus-native-dom/src/{lib,reached}.rs` · `packages/escher-driver/Cargo.toml` · `packages/escher-driver/src/{execute,session,schema,schema_tests,json,explore,fixture_tests,recording_tests,replay,lib}.rs` · `tests/blitz-tests/tests/stand_explore.rs` · companions `tests/blitz-tests/tests/session_common/mod.rs`, `tests/blitz-tests/tests/stand_act_spans.rs` · mechanical `Cargo.lock`
- Listed and left unchanged (table-driven or already `..`-matched): `packages/escher-driver/src/fixture.rs`, `describe.rs`, `mcp/listing.rs`, `tests/blitz-tests/tests/fixtures/mod.rs` (the fixture page has one reader and stays in its check's file), `tests/blitz-tests/tests/stand_mcp_tools.rs` (its listing pins read the table and stayed green)

## Deviations, with justifications
1. **Framework targets count at warn+ only** (operator's word, inputs#I5, relay-1.md: "Framework at warn+ (Recommended)"). The plan counted every level from every outside target, which put dioxus-core / dioxus-signals / generational-box trace records on every ordinary click (`diff/node.rs:87`, `signal.rs:531`), against its own Q2 proof and the unmoved Counter checks. `FRAMEWORK_TARGET_PREFIXES = ["dioxus", "generational_box", "warnings"]`; `warnings` (DioxusLabs' crate, repo dioxuslabs/warnings) joined when `stand_act_errors` read 5 `warnings::warnings` TRACE records on the count click — inside the operator's answer.
2. **The `Err` case runs on a session of its own** (operator's word, inputs#I6, relay-2.md: "Own session, narrow (Recommended)"). dioxus-core 0.7.10's root default ErrorBoundary replaces the whole app with `div > pre > {the error's Display}`, so the count-after proof follows the panic and log cases only, and the Err row's sentinel check reads its `caused` member alone; the error's text crosses as screen content the app's root boundary renders (an existing screen crossing). For the wrap's security amendment.
3. **The scoped capture dispatcher is one per process, with a never-used twin, not one per thread.** The fast leg's workspace run read `capture::tests::a_capture_counts_the_records_its_closure_caused` red (left `[0, 0, 1, 0, 1]`) though the target alone was green: while tracing-core holds a single registered dispatcher it takes a new callsite's interest from the calling thread's default alone (`tracing-core-0.1.36/src/callsite.rs:544-549`), so a callsite first hit under no subscriber cached `never`. Two capture dispatchers alive for the process keep the registry out of that mode. 40 consecutive runs of the target and the whole block's re-run read green.
4. **A render panic, `log`-crate records in process**: as the plan states (stated limits in the field's help).

Scope record (`gate.py scope`: `scope: clean — changed 22 · listed 19 · recorded 3 (companion 2 · mechanical 1 · in-intent 0 · widening 0)`):
- companion · `tests/blitz-tests/tests/session_common/mod.rs` · serves `packages/escher-driver/src/execute.rs` · self — its outcome reader names the acting variants' fields and took `..`
- companion · `tests/blitz-tests/tests/stand_act_spans.rs` · serves `packages/escher-driver/src/execute.rs` · self — the same
- mechanical · `Cargo.lock` · serves `packages/escher-driver/Cargo.toml` · self — escher-driver's new edge to escher-telemetry

## Gates (second firing, the whole block)
- `cargo test -p escher-telemetry --locked --lib` — green
- `cargo test -p dioxus-native-dom --locked --lib -- reached:: boundary::` — green
- `cargo test -p escher-driver --locked --lib` — green
- `cargo test -p blitz-tests --locked --test stand_act_errors` — green
- `cargo test -p blitz-tests --locked --test stand_explore … --test stand_cross_os_readings` — green
- `cargo test -p blitz-tests --locked --test telemetry_scrub --test telemetry_drop --test telemetry_panic_hook` — green
- `cargo test -p seven_guis --locked --test host_log --test mcp_log` — green
- `bash .github/scripts/ci-leg.sh fast` — green (first firing red, deviation 3)
- `bash .github/scripts/ci-leg.sh stand-repeat` — green
- `bash .github/scripts/ci-leg.sh coverage` — green
- `gate.py hygiene`, the push, `ci.py conclusion` — not run — leg operator (the operator pass)

Controls (`{work_dir}/evidence/control-1.txt`): `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.

Matrix: v030-10 planned → implemented, ref `tests/blitz-tests/tests/stand_act_errors.rs::…` (four tests).

Surfaced: none outstanding — both plan conflicts were resolved by the operator's word (deviations 1, 2).

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / test binaries of the gate block and the controls | this run (the gate tool) | terminated — `gate.py census --comm escher-session --comm seven_guis --comm cargo --args stand_act_errors`: `census: none running` |

## Decisions & corrections
- The operator ruled the capture counts Dioxus framework targets only at warn and error (inputs#I5).
- The operator ruled the `Err` case runs on its own session, its sentinel check narrowed to `caused` (inputs#I6).
- Sweep hazard: a capture under no global subscriber is subject to tracing-core's one-dispatcher interest mode — a green lone target hid it; the workspace run showed it.
- Guard refusals: the Bash guard refused a `cd` into the cargo registry and a `cat` heredoc to a scratch file; each re-issued once.
