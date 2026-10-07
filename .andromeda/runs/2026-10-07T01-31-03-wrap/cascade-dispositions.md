# Cascade dispositions — 2026-10-07-change-tracking-and-diff

Written from the listing of `cascade.py sweep --patterns-file cascade-patterns.toml` (trail: `cascade-2026-10-07-change-tracking-and-diff.json`), run twice: once after every master body was amended, once after the leaves were re-derived. Baseline `e616ec1c`. Every pattern's known-positive control fired on the pre-pass masters.

## What was searched
18 patterns, each over the seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf bodies:
- the retired mechanism — `has_changes` · `changed_nodes` · `(refreshed|rebuilt|updated|refresh|rebuild)… on (a|every) poll` · `when the document (has changes|changes|is mutated|has been mutated)` · `InitialTreeRequested` · `update_tree` · `set of changed nodes`;
- the reader and module lists the diff joins — `Snapshot::to_text` · `` `snapshot`, `snapshot_text` `` · `its text form and` · `all three of which` · `snapshot_text.rs since` · `stand_snapshot_text`;
- the retired counts — `41 unit` · `six files` · ``25 `#[test]` `` · ``54 `ok` `` · `518 passed` / `130 result`.

What was NOT searched by pattern: the 181 `file:line` citations of the five edited files — they are the line-map amendment, located by full path and verified by content (below) — and the phrases "still to come" and "seed for snapshot diffs", which occur in no master (0 hits, so no control can fire): the two leaves holding them (CLAUDE.md's architecture block, `services/blitz-dom.md`) were recomputed from the amended sections.

## Master and registry rows (first sweep)
- `has_changes` — 6 rows, all `new` (this pass's text): architecture.md:120 ×3, :121 · test-plan.md:12, :25 · a11y-plan.md:36, :37. The one pre-pass site (architecture.md:121, "from `View::poll` on `has_changes()`") is rewritten. No standing row.
- `changed_nodes` — 5 `new` (architecture.md:120 ×3, :121 · security-plan.md:107 · test-plan.md:12 · a11y-plan.md:36, each inside `take_changed_nodes` or the Document core clause) and 1 `standing edited` (a11y-plan.md:37, the sentence itself, rewritten: amended).
- `on-poll` — 1 `new` (architecture.md:194, the rewritten sentence). The pre-pass sites a11y-plan.md:36 and architecture.md:194 no longer match as written: amended.
- `doc-changes` — 0 rows after the pass (control architecture.md:194 fired pre-pass): both pre-pass sites amended. A statement about this pattern only.
- `initial-tree` — 3 standing: architecture.md:121 and a11y-plan.md:36 (edited — the lifecycle sentences, amended); architecture.md:135 (the adapter's event list `InitialTreeRequested, ActionRequested, AccessibilityDeactivated`) — no change, it names the event and says nothing of the refresh.
- `update_tree` — architecture.md:121 ×2 (edited: amended).
- `to_text-readers` — 10 standing, all on edited lines, each read by offset for whether it is a list of the model's readers: architecture.md:134 @c12504 ("one serialized form, the text" — true, a diff is not a serialized form; no change) and @c13234 (the text clause; the diff clause is added after it); architecture.md:251 (the module row: `snapshot_diff` added); security-plan.md:107, :108, :109 (each gains the diff sentence); :173 (gains the diff); test-plan.md:23 (the eight diff tests added), :25 (the diff clause added); a11y-plan.md:55 (the diff added, "all four"), :270 (the diff clause added).
- `gated-modules` · `feature-list` · `all-three` · `file-list` · `units-41` · `six-files` · `tests-25` — 0 master rows after the pass; each pre-pass site amended (architecture.md:189 · :134 · a11y-plan.md:55 · test-plan.md:24 · :23 · :23 · :11).
- `stand-54` — test-plan.md:111 (edited): the 54 stands as the previous re-count in the Proof chain; the 63 re-count is appended.
- `workspace-518` — test-plan.md:314 ×2 (edited): 130 / 518 stand as the previous re-count; 131 / 542 is appended.
- `stand-files` — 7 standing, all on edited lines: citations of `stand_snapshot_text.rs` (architecture.md:134 · security-plan.md:107, :109 · a11y-plan.md:270 — unchanged, true) and the three test-plan places stand files are listed (:25 — `stand_diff.rs:1` added to the list; :111, :314 — the re-count chains).
- `set-doc` — 0 master rows (a11y-plan.md:37 amended).
- Registries: 0 rows on every pattern. Curation homes: 0 rows. Judgment bases: 0 rows.

## The line map (O2) — not a pattern
`linemap` basis: `git diff -U0 e616ec1c -- {file}` for the five edited files; 181 citations located by full path (0 in the registries, 0 in the leaves — the leaves cite no line of these files); 119 re-pointed, 62 keep their numbers. 169 moves proven by content (the old lines at `e616ec1c` equal the new lines at the new range); the 12 a hunk touches read by hand: architecture.md:109 `document.rs:206-369` → `:206-371` · :194 `document.rs:1529-1550` → `:1536-1561` and `window.rs:372-390` unmoved · :121 and a11y-plan.md:36 `window.rs:376-382` unmoved (new text) · architecture.md:110 `lib.rs:5-12` → `:5-13` · :251 `lib.rs:14-24` → `:15-27` · :134 `lib.rs:32-33` → `:37-38` · a11y-plan.md:37 `document.rs:326-327` → `:326-329` · a11y-plan.md:55 and :282 `lib.rs:7-9` → `:7-10` · a11y-plan.md:55 `lib.rs:20-23` → `:21-26`. test-plan.md:12 `document.rs:2749-3308` took the plain map (`:2760-3319`, the pre-existing modules) and the new module is cited beside it (`:3321-3552`), as the test-plan detector proposed. After the re-point each master's text with digits stripped equalled its pre-pass text with digits stripped. Per master: architecture 50 of 85 · security-plan 11 of 17 · design-system 7 of 12 · test-plan 20 of 20 · obs-plan 14 of 16 · a11y-plan 17 of 28 · layout-templates 0 of 3. Applied by a position-asserted script (each citation replaced at its measured column), before any semantic amendment.

## Leaf rows — re-derived (second sweep: every row is this pass's text or a true standing citation)
- CLAUDE.md `GENERATED:setup:modules` — the blitz-dom line (the changed set), the blitz-shell line (the drain and the gated refresh), the dioxus-native-dom line (`Snapshot::diff`); `GENERATED:setup:pointer-table` — the snapshot row gains the diff and `stand_diff`, a new row for change tracking; `GENERATED:setup:architecture` — "its diffs are still to come" replaced by the built diff and the truthful flag, the driver's return of it still to come. `USER:*` and Session Learnings untouched.
- `.claude/docs/services/blitz-dom.md:19` — recomputed from §Document core and §Invalidation (the "natural seed for snapshot diffs" sentence retired: the diff never reads the set).
- `.claude/docs/services/blitz-shell.md:18` — recomputed from the Mutation contract's shell sentence.
- `.claude/docs/services/dioxus-native-dom.md` — a Snapshot diff bullet, the entry-point list, the unit census (49 in seven files).
- `.claude/docs/services/seven_guis.md:36` — the stand-file list and the fixture list gain `stand_diff`.
- `.claude/docs/gotchas.md` — a new trap, "The changed set is not a diff", from §Invalidation.
- `.claude/docs/conventions.md:25` — the gated module list.
- `.claude/docs/commands.md:27` — the package-alone stand command gains `--test stand_diff`.
- `.claude/docs/a11y-summary.md:22`, `:24` — the diff as a reader of the model; the lifecycle sentence.
- `.claude/docs/security-summary.md:27` — the in-process content row names the diff.
- `.claude/docs/tests-summary.md:21`, `:28` — the stand narrative and the workspace baseline.
- `.claude/rules/a11y.md:26` — when the shell builds the tree; the diff beside the text form. Its `## Session Additions` untouched.
- No change, read: `.claude/docs/obs-summary.md`, `.claude/rules/observability.md`, `.claude/docs/design-summary.md` (their masters changed in citations only, and none of these leaves cites a line of the five files); `.claude/rules/security.md`, `.claude/rules/testing.md`, `.claude/rules/verification-harness.md`, `.claude/docs/stack.md`, `.claude/docs/workflow.md` (no sentence derived from an amended passage).

## The lateral binds
`test-plan §3 ↔ obs-plan §3` (harness commands): neither §3 contract changed — no verb, status shape or event field; test-plan §3's Proof gained a count. `a11y-plan schema ↔ obs-plan schema`: no schema exists or changed. Both pairs stand.

## How the leaves were written
The masters' semantic amendments and CLAUDE.md, the crate notes, the gotcha and the a11y rule went through anchored Edits. Ten single-site leaf replacements (conventions, commands, a11y-summary ×2, security-summary, tests-summary ×2, seven_guis ×2, the a11y rule's reader clause) and the line map went through scripts that assert each target string occurs exactly once (the leaves) or sits at its measured column (the map) — a departure from the letter's Edit-tool rule, taken for the map because 119 replacements include prefix-colliding ranges (`:1269` inside `:1269-1280`) an Edit's `replace_all` would corrupt.
