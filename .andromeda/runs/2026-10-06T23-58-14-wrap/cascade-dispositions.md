# Cascade dispositions — 2026-10-06-compact-snapshot-serialization

Written from the `cascade.py sweep` listing of this run (trail: `cascade-2026-10-06-compact-snapshot-serialization.json`), after every body of the pass was applied and before any sidecar entry landed.

## The search
- Patterns: the 27 of `cascade-patterns.toml` — the retired claim's wording (`wire form`, `Password-only`, `password \`input\``, `password input reads`, `never its length`), the subject token `file input`, the three statements of what the `accessibility` feature gates, the 11 moved citations (5 of `src/snapshot.rs`, 6 of `dioxus-native-dom/src/lib.rs`), and the moved counts (`33 unit`, `five files`, `thirteen on the snapshot`, `snapshot.rs and actionable.rs`, `129 result`, `504 passed`, `48 \`ok\``). Each fired on the pre-pass masters (baseline 97cc4842).
- Scope read by the tool: the seven masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases, the leaf bodies.
- Phrasings the tool could not take (no pre-pass master hit, so no control): `still to come`, `in-process only`, `no serialized form`, `serialization` — searched by hand over the masters, registries, CLAUDE.md, `.claude/docs/**`, `.claude/rules/*`, the playbook and the drift-base: 15 hits, read each. `still to come` → CLAUDE.md:100 (this pass's own re-derived text, about the diffs) and a11y-summary.md:14 (the keyboard harness — another subject). `serialization` → this pass's own report paths (7), blitz-dom's DOM serializer and `dom_string` (architecture 125, 242 · test-plan 46 · verification-harness.md 37 · design-system 80 — another subject). `in-process only`, `no serialized form` → 0 hits.
- NOT looked for: a restatement of the retired claim in a wording none of these patterns or phrasings carry.

## Rows
- architecture.md:134 `wire-form` standing @c17667 — "the check has no wire form": the actionable-key check's, true, untouched by this chunk → no change.
- security-plan.md:107 `wire-form` standing @c3453 — "it adds no crossing — no wire form…": the actionable-key check's, true → no change.
- docs/services/dioxus-native-dom.md:16 `wire-form` leaf — the actionable-key check's bullet, true → no change.
- docs/services/dioxus-native-dom.md:17 `wire-form` · `pw-input` · `pw-reads` leaf — the snapshot bullet, stale → re-derived (the mask covers a file input; the one serialized form is the text) and a new bullet written for the text form.
- architecture.md:134 `pw-input` standing @c11794 — this pass's own text ("a password `input` or a file `input`") → amended.
- a11y-plan.md:270 `pw-reads` standing @c1141 — the sentence kept, extended with the file input → amended.
- security-plan.md:109 `never-length` new — this pass's own text.
- `file-input`: architecture.md:92 standing — the blitz-dom feature list, another subject → no change · architecture.md:134 new ×2, security-plan.md:109 new ×3, test-plan.md:23 new, test-plan.md:25 new, a11y-plan.md:270 new — this pass's own text · security-plan.md:173 standing — amended (the mask clause appended) · a11y-plan.md:146 standing — the file input's generated inner button and its `tabindex`, another subject → no change.
- docs/conventions.md:25 `gated-mods` leaf — stale → re-derived (names `snapshot_text`).
- docs/services/dioxus-native-dom.md:39 `u33` leaf — stale → re-derived (41 unit tests, fourteen `snapshot`, seven `snapshot_text`).
- test-plan.md:314 `r129` · `p504` standing — the previous dated link of the re-count chain, true as dated → no change (this pass's link appended after it).
- docs/tests-summary.md:28 `r129` leaf — stale as the leading figure → re-derived (518 · 0 · 5 over 130 leads; the 504 / 129 figure kept as the dated prior link).
- test-plan.md:111 `ok48` standing — the previous dated link, true as dated → no change (this pass's link appended).
- 16 zero-row patterns (`pw-only`, `model-and-check`, `both-doc`, the 11 citations, `five-files`, `thirteen-snap`, `snap-and-act`) — each control fired on the pre-pass text; a statement about the pattern.
- curation homes 0 rows · judgment bases 0 rows.

## Leaves re-derived (step 3)
- CLAUDE.md — the Modules bullet for `dioxus-native-dom` (the text form, the file-input mask), the pointer-table row for the snapshot (the module and the new stand check), the Architecture paragraph (the text form built; diffs still to come). Read and left: the Overview, Critical Warnings, Key commands.
- `.claude/docs/services/dioxus-native-dom.md` — the snapshot bullet, a new snapshot-text bullet, the source list, the unit-test census.
- `.claude/docs/services/seven_guis.md` — the stand-check list and its fixtures.
- `.claude/docs/conventions.md` — the feature-gating bullet.
- `.claude/docs/commands.md` — the headless-stand command names the thirteenth file.
- `.claude/docs/tests-summary.md` — the blitz-tests coverage bullet, the local baseline.
- `.claude/docs/a11y-summary.md` — the state-readers bullet (the file-input mask; the text form reads the model).
- `.claude/rules/a11y.md` — the accessibility-tree bullet (the text form maps nothing again); `## Session Additions` untouched.
- `.claude/docs/security-summary.md` — a Data classifications row for snapshot content.
- Read and left unchanged: `.claude/rules/security.md` (no restated claim; always loaded, kept lean), `.claude/rules/testing.md` and `verification-harness.md` (neither enumerates the stand files or the counts), `.claude/docs/obs-summary.md` (obs-plan not amended), `stack.md`, `gotchas.md`, `workflow.md`, the other service notes.

## Binds
- test-plan §3 ↔ obs-plan §3 (harness): neither section's contract changed — the agent-run commands, status shape and event fields are as they were.
- a11y-plan schema ↔ obs-plan schema: no schema changed.
