# Fan-out results — 2026-10-06-headless-stand

Report: `escher-0.1.0/chunks/2026-10-06-headless-stand/report.md`. Seven Explore doc-agents, one batch, prompt verbatim
(amendment-flow §Fan-out); detector counts arch 2 · security 3 · design 1 · layout 1 · tests 3 · obs 3 · a11y 2 = 15 =
the `doc:` names over drift-base. Keyed-contract renders: test-plan · obs-plan · a11y-plan (1 key each); architecture
`NOT MIGRATED`; the other three carry no keyed-contract section. No return carried HTML entities (`&lt;` `&gt;` `&amp;`
probe: 0 per return); stripping removed only trailing `#` commentary lines (substance noted per verdict). No raw twin
warranted (no `proposals: []` return failed the parse; the two empty returns' stripped commentary is summarised here).

**Report correction during the pass.** The a11y-plan detector (and the arch detector, D-arch-decisions #2) read the
report's first draft claim "blitz-dom keys disabled-ness on attribute PRESENCE (`element.rs` focusability
`has_attr(disabled)` …)" against `element.rs:629` and found focusability parses the value as a bool. The orchestrator
re-read `element.rs:445-452`, `:628-640`, `pointer.rs:325-335`, `:452-460` and `focusability_updates.rs:55-65`:
confirmed — DISABLED element state / Stylo `:disabled` and the pointer click target are presence-keyed; focusability is
parsed-bool keyed. The report (Spec claims disproved 2 and 3, the Coverage row) and `evidence/disabled-false-probe.txt`
were corrected in place before Validate; the corrected report now carries `element.rs:446-451`, `:629`,
`pointer.rs:330`, `:457`, so proposals citing them no longer carry the re-derivation tell.

## architecture — 10 proposals
Commentary stripped: none.
1. D-arch-resources · §Standard Contracts → Test harness — `HarnessOptions { width, height }` with Default → 8 fields incl. `font_ctx` / `incremental` (harness.rs:12-26, `into_config` :44-59). **apply** — check 1 playbook "Accurate this-chunk addition"; check 5 covers expected amendment 1.
2. D-arch-resources · §Standard Contracts → new "Headless stand (seven_guis)" bullet — `seven_guis::stand` surface + `app::Task` / `task_in_shell` / `DEJAVU_SANS` / `TimerTicks`. **apply** — check 1: "Registry over-reach" fails its precondition (§Standard Contracts enumerates per-symbol API — the Test harness bullet lists every fn), so NO MATCH there; "Accurate this-chunk addition" governs. Applied compactly.
3. D-arch-resources · §Standard Contracts → Dioxus DOM bridge — "falsy `checked` clears" → `checked` or `disabled`; other boolean attributes still literal "false". **apply** — check 1 accurate addition; check 3: rests on the scope record's widening line, carrying the delegate overseer's word (justified branch, PROVISIONAL).
4. D-arch-resources · §Occupied Resources → Names — `seven_guis` `[workspace.dependencies]` path entry + edges blitz-tests→seven_guis (dev), seven_guis→blitz-test-harness / blitz-traits (native). **apply** — accurate addition; expected amendment 2.
5. D-arch-resources (dependent-of) · §Stack and Technologies → Testing row — blitz-tests dev-deps gain seven_guis. **apply** (group with 4).
6. D-arch-resources (dependent-of) · §Existing Scopes → seven_guis row — `stand` module, native deps, woff. **apply** (group with 4).
7. D-arch-resources (dependent-of) · §Existing Scopes → blitz-tests row — stand checks + falsy-disabled regression. **apply** (group with 4).
8. D-arch-resources (dependent-of) · §Infrastructure Patterns → Deployment model — seven_guis' DejaVu is the shared crate-root `DEJAVU_SANS`. **apply** (group with 4).
9. D-arch-decisions · §Conventions → Manifests — `seven_guis` workspace entry is a default-features exception (dioxus-native `compile_error!` with no renderer). **apply** — accurate addition (report Dependencies bullet).
10. D-arch-decisions · §Established Decisions → DOM semantics — "`disabled` read as a parsed boolean" → focusability parsed-bool; element state / `:disabled` / click target presence-keyed; Dioxus now clears falsy `disabled`/`checked`. **apply** — accurate reconcile of wording to the measured mechanism (report Spec claims disproved 3, corrected); check 6 disposes claim 3.

## security-plan — 0 proposals (`proposals: []`)
Commentary stripped: per-detector no-violation reasoning (no external-input surface; no auth/secret; only path-crate edges, `wuff` already locked, cargo-deny green) and one out-of-scope pointer — `security-plan.md:101` (§Input Validation, "Markup attributes | `disabled` | Parsed as a boolean value") vs the report's presence-keyed claim. Taken up by the orchestrator in the cascade's cross-master sweep (step 2), not as a proposal.

## design-system — 0 proposals (`proposals: []`)
Commentary stripped: all five Coverage rows read `tokens n/a`; the §Typography Loading amendment is font loading, outside D-design-tokens. → expected amendment 6 raised by the orchestrator (check 5, below).

## layout-templates — 1 proposal
1. D-layout-surface · §Surface: desktop-native → Primary screens (line 10) — add the headless stand mount (lean task in TaskShell under `main#main`, pinned 800 × 600, no Home) + re-point app.rs citations (`app` :69, `task_in_shell` :82, `TaskShell` :153-172, `HOME_CSS` :174, `SHELL_CSS` :273-328). **apply** — accurate addition; expected amendment 7.

## test-plan — 10 proposals
Commentary stripped: D-tests-obs-harness no drift (in-process Harness only; obs §3 describes neither); the focus-assertion gap routed to a11y-plan.
1. D-tests-framework · §2 → Font-dependent tests — skip-with-eprintln covers the pre-existing tests; stand checks assert unconditionally under the bundled font. **apply** — expected amendment 4.
2. D-tests-framework (dependent-of) · §8 → Real dependencies kept — the stand keeps a real pinned font (bundled DejaVu, system fonts off, no skip). **apply** (group with 1).
3. D-tests-framework (dependent-of) · §9 → Pipeline facts (Fonts) — stand checks independent of `system-fonts`. **apply** (group with 1).
4. D-tests-framework (dependent-of) · §7 → Seed strategies (Font payload) — `seven_guis::DEJAVU_SANS` beside the bullet font. **apply** (group with 1).
5. D-tests-framework · §7 → Builders and options (HarnessOptions) — 6 → 8 fields; `stand::options` the pinned builder. **apply** — expected amendment 3.
6. D-tests-framework · §3 → blitz-test-harness Construction — stand boot surface; harness.rs cite refresh. **apply** — expected amendment 3; the proposal's `harness.rs:70-101` cite is NOT carried by the report → the line range is re-derived by the orchestrator from the file, not pasted.
7. D-tests-framework · §8 → Hand-written fakes (Time) — `TimerTicks` as the stand's time stand-in. **apply** — accurate addition (report Symbols bullet 4).
8. D-tests-coverage · §9 → Local baseline (`cargo test --workspace`) — 416·0·4 / 114 lines → 430·0·4 / 120 lines. **apply** — expected amendment 5.
9. D-tests-coverage (dependent-of) · §9 → Local baseline (`-p blitz-tests` note) — +6 files / +14 tests, per-crate count not re-measured. **apply** (group with 8).
10. D-tests-coverage · §1 → Coverage scope (tests/blitz-tests) — stand checks + falsy-disabled regression. **apply** — accurate addition.

## obs-plan — 1 proposal
Commentary stripped: none beyond the rationale.
1. D-obs-stack · §3 → Logging stack — "escher's stand installs `escher_telemetry::init`" → `seven_guis_native` installs it; the headless stand `seven_guis::stand` installs no subscriber. **apply** — accurate reconcile (report Symbols bullet 5, census gate); obs §3 ↔ test-plan §3 bind unaffected (harness commands unchanged).

## a11y-plan — 4 proposals
Commentary stripped: D-a11y-obs-schema no drift (schema NOT YET MEASURED, no log format changed).
1. D-a11y-surface · §5 → Script and framework exposure — falsy `disabled`/`checked` cleared by dioxus-native-dom; other boolean attributes still literal "false". **apply** — accurate addition; the `hidden: false` consequence is written `recorded, not established` (unmeasured).
2. D-a11y-surface · §5 → Focus order (line 144) — disabled-ness keyed two ways (state / `:disabled` / click target presence; focusability parsed-bool). **apply** — accurate, confirmed by the orchestrator's source re-read (see report correction above); check 6 disposes claim 3.
3. D-a11y-surface (dependent-of 2) · §5 → Focus order (line 141) — "if it is not disabled" → "if its `disabled` attribute does not parse as `true`". **apply** (group with 2).
4. D-a11y-surface · §8 → Error recovery — `stand_flight_booker.rs` asserts the invalid / disabled-Book cue; Dioxus control focus / Tab order unasserted. **apply** — accurate addition; the "re-enabled control focusability" wording re-derived from the corrected report (the fix leaves focusability unchanged).

## Orchestrator raises (Validate check 5 — expected amendments)
- Expected amendment 6 — design-system §Typography Loading: the headless stand registers bundled DejaVu Sans for every generic with system fonts off (`seven_guis::stand::font_ctx` = `build_single_font_ctx(DEJAVU_SANS)`). No detector proposed it (D-design-tokens' blind class). **raise · routine** — report Symbols bullets 3 and 5 substantiate it.
- Expected amendments 1-5, 7 — matched by proposals arch 1, 4-8; test-plan 1, 5, 6, 8; layout 1.

## Validate summary
- Re-derivation tell: none after the report correction (the one uncarried cite, test-plan 6's `harness.rs:70-101`, is re-derived, not pasted).
- Check 2 cross-contradiction: none — arch 10, a11y 2/3 and the security-plan:101 sweep hit state the same two-keyed mechanism.
- Check 3 intent-consistency: the scope record's two `widening` lines carry the delegate overseer's word (PROVISIONAL) — justified branch; the `blitz-traits` edge and boot-for-every-task deviations are justified in the report.
- Check 4: sweep claims ("only line 129", obs "no other site") re-run by the orchestrator's cascade sweep before any sidecar entry.
- Check 6 disproved claims: (1) plan premise — resolved in the impl, recorded in the report (plan.md immutable) · DISPOSED; (2) scope.md "no markup change" — scope correction carried by the report; the DOM change lands in arch 3 / a11y 1 · DISPOSED; (3) the two-keyed engine fact — matched by arch 10 + a11y 2/3 (+ security-plan:101 via the sweep) · DISPOSED.
- Escalations: 0.
