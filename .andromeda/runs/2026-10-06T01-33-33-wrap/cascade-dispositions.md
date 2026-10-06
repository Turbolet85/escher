# Cascade dispositions — 2026-10-06-telemetry-bootstrap

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (listing: `sweep.txt`, 210 lines), baseline 1ff57e3c, over the seven masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases and every leaf. 16 patterns, each control fired on the pre-pass masters: `no-scrub` (absence-of-scrub phrasings) · `scrub-word` · `unscrubbed` (`unscrubbed|as-is`) · `fmt-init` · `native-sub` (subscriber-install verbs; 0 rows, control fired) · `gated-only` (feature-gated-only phrasings incl. "tracing gated per call site", "behind a `tracing` feature") · `examples-only` · `log-format` (`log format|JSON schema`) · `svc-identity` · `count-407` · `count-255` · `count-404` · `boot-labels` · `otel` · `panic-hook` · `publish-bump`. Dropped as uncontrollable (no pre-pass master hit — the tool refuses them): `stdout-logs` (logs to/on stdout phrasings) and `rust-log` (`RUST_LOG|EnvFilter`); controlled by hand with `grep -rn -i` over CLAUDE.md, `.claude/docs`, `.claude/rules`, playbook, drift-base — every hit is this pass's own text (seven_guis.md, observability.md, commands.md, obs-summary.md, security.md), no stale site.

**Not looked for:** wording of claims this pass did not retire (engine call-site lists, slice search records' file counts). Citation line numbers are covered by the separate re-point (fanout-results.md C1).

## Master / registry rows
- `new` rows (scrub-word 14 · unscrubbed 5 · fmt-init 5 · log-format 2 · svc-identity 8 · otel 1 · panic-hook 8 · publish-bump 1) — this pass's own text; each amended line re-read for an intra-line duplicate of a retired claim: none.
- security-plan.md:161 (no-scrub, scrub-word · edited ×2) — amended (R-S4): the s05 search record stands, qualified with escher's sink.
- obs-plan.md:279-287 (no-scrub ×16 incl. wrap rows) — no change: slice-scoped search records, true for their slices; the section now leads with the scrub and states "The searches below predate it and stand for their slices".
- obs-plan.md:258 (`## 8. PII Scrubbing & Compliance`), :273 (`**Scrubbing:**`, edited) — heading text, true.
- obs-plan.md:260 (`Values logged as-is`) — true (upstream sinks); amended with the past-scrub residue line.
- obs-plan.md:301 (CI-log "unscrubbed build output") — no change: CI logs, not the telemetry sink.
- obs-plan.md:13 · :14 · :17 (gated-only) — no change: blitz-dom / blitz-vibey-script engine-crate claims, true.
- obs-plan.md:30 (gated-only, edited) — amended (O7).
- obs-plan.md:50-53 · :61 · :63 · :71 · :97 · :98 · :124 · :130 · :251 · :253 · :254 (otel) — no change: OTel / backend absence records, still true (no OTel crate).
- obs-plan.md:68 (fmt-init, edited) — amended (O1).
- obs-plan.md:7 · :219 · :221-223 · :252 (panic-hook) — :7 amended (O5); :219 heading; :221-222 WASM hook, true; :223 WPT hook, true; :252 s06 search record, true for its slice.
- architecture.md:188 (fmt-init / gated-only @c922 / @c948, edited) — amended (A10): the "only under the `tracing` feature" clause now scopes the upstream `fmt::init()` installs.
- architecture.md:219 (gated-only, edited) — amended (A12).
- architecture.md:58 · :174 · :181 (panic-hook) — no change: stack row of `console_error_panic_hook` (citations re-pointed), the wasm stand row, the WPT runner's hook — all true.
- architecture.md:85 · :137, design-system.md:138 (count-407) — no change: source line numbers, not counts.
- architecture.md:105 · :208 · :217, security-plan.md:217 (publish-bump) — :217 amended (A4); :105 examples' `publish = false` sample, true; :208 "blitz packages versioned together", true; security-plan :217 blitz-tests `publish = false`, true.
- security-plan.md:241 (boot-labels, edited) — amended (R-S2).
- security-plan.md:338 · :362 (otel) — no change: s05 / s12 search records, true.
- security-plan.md:354 (`**Log format and backends:**`), :355 (gated-only: "Engine crates log through the `tracing` crate behind a `tracing` feature") — heading / engine claim, true; the section gained the escher-sink bullet (R-S3).
- security-plan.md:359 (examples-only, edited) — amended (R-S3 dependent).
- test-plan.md:98 (log-format, edited) — amended (T1). :284 · :285 · :287 (counts, edited) — amended (T3 · T2 · T4).
- a11y-plan.md:87 (log-format ×3, edited) — amended (Y1); "structured violation JSON schema / log format" there names the a11y violation format, still unmeasured.
- registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:3 · :4 (edited) — amended (O10). registries/obs-plan-contracts.toml:9 (labels) — no change: label names unchanged.

## Leaf rows → re-derived (step 3)
- CLAUDE.md:43 (gated-only) · :44 (no-scrub, scrub-word, unscrubbed) — re-derived (warnings block); plus overview (packages/ line), modules (escher-telemetry), pointer table (telemetry contract row) recomputed from arch §Infrastructure Patterns / §Existing Scopes / §Standard Contracts.
- .claude/rules/observability.md:20 · :22 · :28 · :31 — re-derived (Logging · PII · Not-yet-measured sections).
- .claude/docs/obs-summary.md:12 · :13 · :14 · :34 · :37 · :38 — re-derived (whole file). :28 (CI "unscrubbed build output") — kept, true.
- .claude/docs/security-summary.md:26 · :33 — re-derived (Logged values row; not-yet-measured owners). .claude/rules/security.md — env-read list gains `RUST_LOG` (security-plan Secret Management; no sweep row — found by provenance).
- .claude/docs/tests-summary.md:15 · :24 — re-derived (stand log format; counts; coverage). .claude/rules/verification-harness.md:14 — re-derived.
- .claude/docs/conventions.md:19 · :26 — re-derived (publishability; feature gating).
- .claude/docs/stack.md:36 — re-derived (tracing-log, escher-telemetry). :10 (wasm `console_error_panic_hook`) — kept, true.
- .claude/docs/services/seven_guis.md:11 — re-derived (native escher-telemetry consumption; boot smoke under Testing).
- .claude/docs/commands.md — by provenance (arch §Standard Contracts → commands.md): `RUST_LOG=info just seven_guis` added; no sweep row.
- .claude/docs/gotchas.md (arch §Cross-cutting) — read by provenance: no logging entry, no change.
- .claude/docs/a11y-summary.md — read by provenance (a11y-plan §3 amended): its "Structured violation JSON … NOT YET MEASURED" stays true; no change.

## Base rows (judgment bases — never edited by the cascade)
- drift-base.md:87 · :88 · :101 · :106 · :119 — detector invariants citing "§8 PII Scrubbing", "log format", "OTel setup", "a11y violation JSON schema": all still name live sections / binds; no retired wording; no proposal.

## Curation homes
- 0 rows across CLAUDE.md `USER:session-learnings`, every rule file's `## Session Additions`, `docs/session-learnings.md`.
