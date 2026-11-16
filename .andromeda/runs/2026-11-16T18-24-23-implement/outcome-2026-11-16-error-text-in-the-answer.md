# Outcome — 2026-11-16-error-text-in-the-answer

**Outcome:** green · **Run:** 2026-11-16T18-24-23-implement · **Next:** the OPERATOR PASS (entries 13 hygiene, 14 push, 15 CI read), then `/andromeda-wrap-session`.

## Files changed (24: 20 listed, 4 recorded; `gate.py scope`: clean)
- New: `tests/blitz-tests/tests/stand_act_error_text.rs`, `tests/blitz-tests/tests/capture_panic_hook.rs`
- Modified (listed): `packages/escher-telemetry/src/{capture,lib}.rs`, `packages/blitz-dom/src/{events/pointer,form,mutator,layout/mod,lib}.rs`, `packages/dioxus-native/src/assets.rs`, `packages/dioxus-native-dom/src/{boundary,dioxus_document,lib}.rs`, `packages/escher-driver/src/{caused,execute,json,schema,schema_tests,lib}.rs`, `tests/blitz-tests/tests/stand_act_errors.rs`
- Listed and unchanged: `packages/escher-driver/src/explore.rs` (its `Panicked` reading off `kind == Panic` holds as written)
- Recorded (scope-record rev 2): `packages/escher-telemetry/src/format.rs` · in-intent · serves step 1; `packages/escher-driver/src/{fixture_tests,recording_tests,replay}.rs` · companion · serves `execute.rs`

## Deviations, with justifications
1. `password` added to `CONTENT_FIELDS` (`format.rs`, out of list, in-intent). The plan's security criterion and step 7 require a `password` field to read `[redacted]` by name, and the plan's scrub is "a field whose name is in `CONTENT_FIELDS`"; that set held no `password` (research's sweep read format.rs as no change). The sink now redacts a `password` field too: a tightening of what the sink prints, never a widening.
2. `escher_telemetry::subscriber` made public. Step 7 runs steps under it; it was private and took an `EnvFilter` blitz-tests cannot name. It now takes a `tracing::level_filters::LevelFilter`; the private one kept as `filtered_subscriber`. Re-export of the capture's new types: `FIELDS_KEPT`, `KeptField`, `Panic`, `RECORDS_KEPT`, `Record`, `Source`, `TEXT_CUT`, `Text`.
3. `caused` written shape — two members the plan names only as "the rest counted": an error item writes `chain_past` (always, beside `chain`), as a log item writes `fields_past` beside `fields`. A chain link and a field are each written as an object (`{"message"[,"cut"]}`, `{"name","value"[,"cut"]}`). A panic or error in `caused_past` carries no `source` (none applies).
4. A panic the run path caught that the capture hook did not see (a process whose hook was replaced after the first capture) still answers a `panic` item, its message the fixed `<payload unread>`, so `explore`'s `Panicked` ending never goes dark.
5. The bad-link click reaches no handler of the app's, so its engine item carries no `id`/`event`; the founder check asserts that, not a tie.
6. The boundary registry holds every boundary ever built under the factory for the document's life (dioxus-core's `ErrorContext` fields are private, so a dropped boundary cannot be told); an app that remounts many boundaries grows it. Stated here, not mitigated.
7. A first `boundary.rs` unit test for a document built after its vdom's first build was written and dropped before any gate (it re-ran `rebuild` on a built vdom); the fallback branch is covered by no test.
8. The flood check's cut first derived from `TEXT_CUT` itself, so the cut control read DOES NOT HOLD; the check now pins the measured 1024 as a literal, and the re-fired control holds.

## Gates
- `cargo test -p escher-telemetry --locked --lib` — green (25 passed)
- `cargo test -p dioxus-native-dom --locked --lib -- boundary::` — green (5 passed; 118 filtered out)
- `cargo test -p escher-driver --locked --lib` — green (122 passed)
- `cargo test -p blitz-tests --locked --test stand_act_error_text --test capture_panic_hook` — green
- `cargo test -p blitz-tests --locked --features dioxus-native-dom/tracing --test stand_act_error_text` — green
- `cargo test -p blitz-tests --locked --test stand_act_errors` — green (5 passed)
- companions `stand_explore … stand_cross_os_readings` — green (2+2 ignored · 7 · 7 · 8 · 4 · 3)
- `telemetry_scrub`, `telemetry_drop`, `telemetry_panic_hook` — green (1+1 ignored · 1 · 1)
- seven_guis `host_log`, `mcp_log`, `mcp_server`, `cli_commands` — green (18 · 6 · 3 · 8)
- `bash .github/scripts/ci-leg.sh fast` — red ×2 (rustfmt diff; clippy `type_complexity` on `settled_step`), then green twice (10.3, 10.4 on the final tree)
- `bash .github/scripts/ci-leg.sh stand-repeat` — green, `stand-repeat: 3 of 3 rounds green` (948.8 s)
- `bash .github/scripts/ci-leg.sh coverage` — green, `coverage floor: 3 of 3 crates at or above their floors` (escher-driver 6262/6397 = 97.89 %, dioxus-native-dom 4444/4912 = 90.47 %, blitz-test-harness 1006/1047 = 96.08 %)
- operator entries 13 (hygiene), 14 (push), 15 (CI read) — not run, the operator pass's
- Tree note: entries 2, 6–9, 11, 12 last ran before the final edits (cargo fmt, the `StepCaused` alias, the cut literal in the founder check); entries 3, 4, 5 and `fast` re-ran on the final tree, green.
- Windows cross-check: `cargo check --tests --locked --target x86_64-pc-windows-gnu -p blitz-tests --test stand_act_error_text --test capture_panic_hook --test stand_act_errors` — exit 0, no warning in the new files.

## Controls (gate.py control; evidence `control-1.txt`, `control-2.txt`)
- scrub (content-name test always-false) · entry 4 · holds
- cut (TEXT_CUT 1024 → 2048) · entry 4 · DOES NOT HOLD, then after the check's fix · holds
- chain (the hook's call to the replaced hook removed) · entry 4 · holds
- factory (the provision in `DioxusDocument::new` removed) · entry 4 · holds
- href (the interpolation restored in `pointer.rs`) · entry 5 · holds

## Smoke
No binary's start changed; `host_log` and `mcp_log` (entry 9) stand in, green.

## Ledger
No capability owned (`coverage where chunk = …`: 0 rows); no matrix write.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `error_text_host_full` / `error_text_host_incremental` (test-binary host children) | this run (via the founder check) | terminated — stopped and killed by the check |
| `cargo` / test binaries of the gates and controls | this run | terminated |
`gate.py census --comm escher-session --comm stand_act_error --comm cargo --args error_text_host`: `census: none running`.

## Decisions & corrections
- Plan premise disproved: `password` not in `CONTENT_FIELDS` — measured by reading format.rs:49-61; resolved by adding it (deviation 1).
- Plan premise disproved: `escher_telemetry::subscriber` was private — resolved (deviation 2).
- Sweep hazard: a check that computes its expectation from the constant under control (`TEXT_CUT`) moves with the mutation — the control read DOES NOT HOLD (control-1.txt).
- Guard refusal: the Bash guard refused a `cd` into the registry and a pipe of `gate.py run --dry-run` into `head`; each re-issued once in the guard's form.
- Hygiene refused the four P3 probe logs research wrote under `evidence/` (absolute home paths of the cargo registry; `gate.py respell` refused each: "P1 hits of another form (home)"). The operator chose «Move to scratchpad (Recommended)», noting «In probe-figures.md name each figure research cites and say the raw logs were captures not kept.» (AskUserQuestion, this session; inputs#I3). The four logs moved to `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/920606e4-79ab-473c-ad99-048e595afffd/scratchpad/probe-captures/`; `evidence/probe-figures.md` names each figure research cites; hygiene then read `clean`. Research's path citations of those logs now point at files not kept.
