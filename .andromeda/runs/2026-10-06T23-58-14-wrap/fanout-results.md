# Fan-out results — 2026-10-06-compact-snapshot-serialization

Seven doc-agents, one batch. Detector counts per prompt: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = 15 = the drift-base's 15 `doc:` names.
Entity probe: the seven returns were read whole; none holds an HTML entity (`&lt;` `&gt;` `&amp;` — 0), so no decode was applied.

## Verdict lines
- **architecture** — 11 proposals (all D-arch-resources: 1 primary + 10 dependents). D-arch-decisions: no drift. Stripped: a trailing comment block (the D-arch-decisions verdict; five sites read and left unchanged — the second `no wire form` on line 134 is the actionable-key check's; unmoved citations; line 207's README wording; line 259's non-exhaustive coverage list; §Occupied Resources has nothing to register).
- **security-plan** — 7 proposals (all D-security-input: 3 `escalate`, 4 `warning`). D-security-auth, D-security-deps: no drift. Stripped: a leading comment block (the three verdicts; a SWEEP HAZARD — line 107 holds `no wire form` twice and only the first is the snapshot's; a note that it re-read line numbers in the tree; unrelated hits left alone).
- **design-system** — `proposals: []`. Stripped: commentary (no UI; every Coverage row `tokens n/a`). Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: commentary (no surface added; a forward note that the Driver CLI chunk is where a `cli` surface entry would become due). Raw twin: `.raw-fanout-layout-templates.md`.
- **test-plan** — 6 proposals (all D-tests-coverage: the invariant itself holds; the proposals are the test-plan's own records of the tests). D-tests-framework, D-tests-obs-harness: no drift. Stripped: a leading comment block (the three verdicts).
- **obs-plan** — `proposals: []`. Stripped: commentary (no new log site; obs-plan.md:68 still holds; no moved citation in obs-plan). It states "my own read-only count over" the three sources "also reads 0" — a re-derivation from the codebase, against its prompt; no proposal rests on it. Raw twin: `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripped: commentary (no interactive product element; no schema change; a flag that the plan's expected amendments for a11y-plan fall outside both detectors). Its flag repeats the report's first, wrong, `:337-390 → :340-393` map — that citation is `stand_snapshot.rs`'s and unmoved (the report was corrected before validation). Raw twin: `.raw-fanout-a11y-plan.md`.

## Two report errors the pass caught (both corrected in report.md before any disposition)
1. `snapshot.rs:` also matches `stand_snapshot.rs:` — 5 of 22 hits are the unchanged test file's; the edited file has 17 citations, 5 moved ranges. Found by the orchestrator reading a11y-plan.md:314.
2. `no wire form` hits 6 times; 4 are the snapshot's (architecture 134 @12181 · security-plan 107 @1987, 108, 109), 2 are the actionable-key check's and stand (architecture 134 @14711 · security-plan 107 @2930). Found by the security-plan detector, confirmed by the architecture detector and by an offset read.

## architecture — proposals and dispositions
A1 (primary) · §Standard Contracts → Dioxus DOM bridge · register `Snapshot::to_text` (private module `snapshot_text`, `accessibility`-gated; a pure function; the line grammar; `Debug`-form quoting; no `Display`, parser, JSON, truncation), `SNAPSHOT_TEXT_BUDGET = 10_000` and its re-export, the four screen sizes; cites snapshot_text.rs:16, :36, lib.rs:32-33, stand_snapshot_text.rs:1-7; "do not write ratified by the founder".
 → **reject as proposed, raise and apply** — the re-derivation tell: it cites `stand_snapshot_text.rs:1-7`, a location the report did not carry. The fact is the plan's expected amendment (architecture §Standard Contracts) and the report substantiates it → raised by the orchestrator under check 5, routine (playbook: accurate this-chunk addition; the registry-over-reach rule does not govern — this contract enumerates dioxus-native-dom's public items one by one, and earlier chunks registered `snapshot`, `MASKED_VALUE` and `UnkeyedActionable` here). The `:1-7` coordinate was measured by the orchestrator and added to the report.
A2 (dependent) · same section · replace "the snapshot has no wire form, and no driver, CLI or MCP command exposes it yet" with: one text form, returned to its caller only (PROVISIONAL), no log, event, socket, file or command, callers are tests; leave the second `no wire form` (the check's).
 → **ESCALATE** (check 1: boundary widening, the never-routine class, judged by its subject — the snapshot's first serialized form; group E1 with S1–S3).
A3 (dependent) · same section · the `value` clause: the mask widens from a password input to a password or file input; the DOM attribute and form submission still hold the path.
 → **apply** (check 1: the operator's recorded answer at P4 — a file input reads the fixed mask — and the plan's expected amendment name the change; it narrows what the snapshot carries).
A4 (dependent) · same section · the feature sentence names the text form among what `accessibility` gates.
 → **apply** (routine: accurate this-chunk addition).
A5 (dependent) · same section · citation `snapshot.rs:109-208` → `:109-211`.
 → **apply** (routine; the report's line map).
A6 (dependent) · same section · citation `lib.rs:28-29` → `:30-31`.
 → **apply** (routine; the report's line map).
A7 (dependent) · same section · citation `lib.rs:23-24` → `:25-26`.
 → **apply** (routine; the report's line map).
A8 (dependent) · §Existing Scopes → dioxus-native-dom · add module `snapshot_text` to the row; citation `lib.rs:14-22` → `:14-24`.
 → **apply** (routine; the plan's expected amendment; the report's Crates / modules and line map).
A9 (dependent) · §Cross-cutting Patterns → Config management · "its `snapshot` and `actionable` modules" → names `snapshot_text` too. A site the report's lists did not carry (line 189; none of the swept tokens), found by reading for the claim.
 → **apply** (routine; the report's Crates / modules bullet substantiates it).
A10 (dependent) · §Conventions → Feature gating · citation `lib.rs:43-66` → `:47-70`.
 → **apply** (routine; the report's line map).
A11 (dependent) · §Conventions → Feature gating · citation `lib.rs:57-60` → `:61-64`.
 → **apply** (routine; the report's line map).
The group's primary is rejected only as a transcription source; its fact is applied through check 5, so the dependents stand with it.

## security-plan — proposals and dispositions
S1 (primary, escalate) · §Input Validation → `id` row · in the `DioxusDocument::snapshot` clause only, retire "the snapshot has no wire form…": one text form `Snapshot::to_text`, the id a quoted `id=` field, returned to its caller only (PROVISIONAL), no log, event, socket, file or command, the platform adapter stays the id's only exit; add citations; leave the `unkeyed_actionable` clause.
 → **ESCALATE** (group E1).
S2 (dependent, escalate) · §Input Validation → accessible-names row · retire "the snapshot has no wire form and reaches no log or event": names are written quoted into the text, returned to its caller only (PROVISIONAL).
 → **ESCALATE** (group E1).
S3 (dependent, escalate) · §Input Validation → masked-value row · retire the closing "the snapshot has no wire form…": the text carries the mask, never the literal; returned to its caller only (PROVISIONAL).
 → **ESCALATE** (group E1).
S4 (warning) · §Input Validation → masked-value row · widen from password to password and file: the key cell, the reader's description, "Password- and file-only, and snapshot-only", the DOM untouched.
 → **apply** (check 1: the operator's recorded answer at P4 and the plan's expected amendment name the change, "not provisional"; a narrowing).
S5 (dependent, warning) · §Data Protection → Local user data handled · the file-path bullet gains: the attribute and form submission keep the path; the snapshot and its text read the mask; the reader's half measured, the dialog's write read from code.
 → **apply** (routine; the plan's expected amendment).
S6 (warning) · masked-value row · citation `snapshot.rs:184-208` → `:184-211`.
 → **apply** (routine; the report's line map).
S7 (warning) · `disabled` row · citation `snapshot.rs:447-494` → `:511-558`.
 → **apply** (routine; the report's line map).
S4's basis quotes source text the report does not carry (`eq_ignore_ascii_case("file")`); its location (`snapshot.rs:184-211`) and its fact (compared ASCII-case-insensitively) are the report's, so it stands; the applied text is re-derived from the report.

## test-plan — proposals and dispositions
T1 (primary) · §1 Coverage scope → dioxus-native-dom · 41 unit tests in six files; the snapshot group fourteen, adding the file-input mask; citation `:210-527` → `:213-591`; a new group of seven on `Snapshot::to_text`; cites snapshot_text.rs:78-354.
 → **reject as proposed, raise and apply** — the re-derivation tell: it cites `snapshot_text.rs:78-354`, a location the report did not carry. The fact is the plan's expected amendment (test-plan §1) and the report's Counts bullet substantiates it → raised by the orchestrator under check 5, routine. The `:78-354` coordinate was measured by the orchestrator and added to the report.
T2 (dependent) · §1 → dioxus-native and stylo_taffy · the list of dioxus-native-dom files holding tests gains `snapshot_text.rs` (line 24 — a second statement of "five files" without the token).
 → **apply** (routine).
T3 (dependent) · §3 Crate-local test helpers → dioxus-native-dom · citations `:216-221` → `:219-224`, `:447-494` → `:511-558`.
 → **apply** (routine; the report's line map).
T4 · §1 → tests/blitz-tests · the stand coverage list gains the text form and the size budget; `stand_snapshot_text.rs:1` joins the citation list.
 → **apply** (routine; the plan's expected amendment).
T5 · §3 Agent-run contract → Proof · append the re-count: `run stand` 54 `ok` stand events (+6 `stand_snapshot_text`); the `run.end` ignored count not restated.
 → **apply** (routine).
T6 · §9 Pipeline facts → Local baseline · append the re-count: 130 result lines, 518 · 0 · 5, on the pre-CI commit d13935da.
 → **apply** (routine).

## Raised by the orchestrator (check 5 — the plan's expected amendments no detector proposed)
R1 · a11y-plan §7 Accessibility tree output (line 270) — the text form is a further reader of the model with no second role or name mapping (one line per snapshot node; `focused` on exactly `back-btn` after a Tab press); the mask covers a file input; citation `snapshot.rs:184-208` → `:184-211`. → **apply** (routine: the report substantiates each fact; the detectors' invariants do not cover it — the a11y-plan return says so).
R2 · a11y-plan §2 Feature exposure (line 55) — the feature also gates the snapshot's text form, which the crate doc names; the new module's lines join the citation. → **apply** (routine).
R3 · obs-plan §3 / §8 — the plan's entry says "no claim changes"; the obs-plan detector confirms obs-plan.md:68 still holds. → **no amendment** (not carried; this chunk changed neither value).
a11y-plan.md:282 (`lib.rs:7-9`) is unmoved; a11y-plan.md:314 cites `stand_snapshot.rs:337-390`, an unchanged file — no change at either.

## Check 6 — the report's disproved claims
- The plan's `str::escape_debug` claim → disposed by the operator's direction (the serializer uses the `Debug` form); the plan is immutable; the std fact goes to curation.
- "no wire form" (architecture 134, security-plan 107, 108, 109) → A2, S1, S2, S3 (group E1).
- security-plan 109 "Password-only" → S4.
- The route's file-input hypothesis → P5 (the entry's CARRY is spent; its frozen line is compacted at the flip).
- CLAUDE.md "its serialization and diffs are still to come" → the cascade's re-derivation.

## Escalations
E1 — A2 · S1 · S2 · S3 (one dependent group across two masters): the snapshot's first serialized form, and the answer that it leaves the process through the returned value only, PROVISIONAL since the plan's P5 review. Resolution: the operator, asked at this wrap (2026-10-07), chose "Record as provisional" over "Ratify now" and "Hold the wrap" — the four are applied; the rows and the two sidecar entries say one text form, returned to its caller only, PROVISIONAL pending the founder's word at the Epoch 3 boundary; the handoff keeps it on the boundary list. No playbook rule proposed (the never-routine class).

## Applied
24 proposals: 18 applied as routine, each re-derived from the report (A3–A11 · S4–S7 · T2–T6); 4 applied after the escalation E1 (A2 · S1 · S2 · S3); 2 rejected as transcription sources, their facts applied through check 5 (A1 · T1). Raised by the orchestrator: R1, R2 applied; R3 no amendment. Four sidecar entries (architecture · security-plan · test-plan · a11y-plan), each `on-form`, appended through `splice.py` and read back as the file's last line.
