# Cascade dispositions — 2026-10-06-snapshot-state-fidelity

Written from the sweep's own listing, after every body of the pass was applied and after the leaves were
re-derived (the sweep was run twice: once after the bodies, once after the leaves; the rows below are the second
run's, the first run's leaf rows named where they drove an edit). Trail: `cascade-2026-10-06-snapshot-state-fidelity.json`.

## The search
`cascade-patterns.toml`, 17 patterns, each with a known-positive control that fired on the pre-pass masters
(baseline `52616c18`): the retired claim's wording (`falsy-two`, `falsy-disabled`, `three-types`, `not-established`),
its verbs and mechanism (`still-written` — "still write / still written"; `literal-false`; `hidden-false`;
`kbd-absent`; `value-reader`; `enabled-reader`), the moved counts (`unit-29`, `nine-snapshot`, `ws-490`, `ws-127`,
`stand-41`) and every old line citation of the two shifted files (`mw-cites`, 21 old `mutation_writer.rs` targets;
`snap-cites`, 7 old `snapshot.rs` targets). Read over the seven masters, `.andromeda/registries/**`, the three
curation homes, the two judgment bases and the leaf bodies.

**Not looked for:** the route's own freight (the next entry's password hypothesis — P5's), the chunk folder, the
sidecars (history, never swept), and `.claude/docs/session-learnings.md` beyond the tool's curation rows.

## Zero-row patterns (each control fired, so each is a statement about its pattern)
`not-established` · `nine-snapshot` · `mw-cites` · `snap-cites` · `three-types` — after the pass no master, registry
file, curation home, judgment base or leaf holds the retired "recorded, not established" clause, the old snapshot
unit count, an old line citation of either shifted file, or "its three types re-exported". After the leaf
re-derivation `falsy-two`, `literal-false`, `kbd-absent` and `unit-29` read zero rows too.

## Rows — masters
- `test-plan.md:25` `falsy-disabled` (standing, edited) — **no change**: the kept clause naming
  `dioxus_falsy_disabled.rs`, a file that still exists and still pins the `disabled` clearing; the new clause for the
  27-name list sits beside it.
- `architecture.md:134` `still-written` ×2 (edited) — **amended, this pass's own text**: "a truthy value of a listed
  name is still written" and "an attribute outside the list … is still written as `"false"`". The retired clause
  ("every other boolean attribute … is still written with the literal value") is gone (`literal-false` 0 rows).
- `security-plan.md:125` `still-written` (new) — **this pass's row** for the falsy clear.
- `a11y-plan.md:179` `still-written` ×2 (new) — **this pass's bullet**.
- `architecture.md:134` · `test-plan.md:25` · `a11y-plan.md:179` `hidden-false` (new) — **this pass's text**: each
  states that a `hidden: false` element is displayed.
- `test-plan.md:314` `ws-490`, `ws-127` and `test-plan.md:111` `stand-41` (standing, edited) — **no change**: each
  is the previous DATED link of a count bullet whose form is a chain of dated re-counts; the new link (504 over 129;
  48 `ok`) is appended after it and the old one stands as the dated record it is.
- `architecture.md:134` `value-reader`, `enabled-reader` (standing, edited) — **amended in place**: both clauses keep
  their opening words and gain the password exception and the pinned presence reading.

## Rows — leaves (each adds its leaf to the re-derivation; none stands in for the recompute)
First-run leaf rows and what was done:
- `.claude/docs/services/dioxus-native-dom.md:21` (`falsy-two`), `:33` (`still-written`, `literal-false`), `:39`
  (`unit-29`) — **re-derived**: the falsy-clear bullet, the gotcha about attributes outside the list, the 33-test
  census; the snapshot bullet (`:17`) gained the state readers and the mask though no pattern hit it.
- `.claude/rules/a11y.md:31` (`falsy-disabled`, `still-written`), `:34` (`kbd-absent`) — **re-derived** in the
  generated body; its `## Session Additions` untouched.
- `.claude/docs/a11y-summary.md:22` (`falsy-disabled`, `still-written`) — **re-derived**, with the Keyboard-harness
  line (`:14`), which no pattern hit ("observed absent (no Tab-order or key-event tests)").
- `.claude/docs/tests-summary.md:28` (`ws-127`) — **re-derived**: the workspace baseline leads with 504 · 0 · 5 over
  129, the earlier readings kept as its history; the coverage sentence (`:21`) gained the two new files' behaviours.
Second-run leaf rows: `.claude/rules/a11y.md:31` and `.claude/docs/a11y-summary.md:22` (`still-written`,
`hidden-false`), `.claude/docs/tests-summary.md:21` (`hidden-false`) — **this pass's re-derived text**.

## Leaves re-derived by provenance, with no sweep row
- `CLAUDE.md` (architecture → `GENERATED:setup:*`): the dioxus-native-dom module line names the mask; the pointer
  table's Snapshot row names `stand_{snapshot,snapshot_state}.rs`; the architecture paragraph states the snapshot's
  state as proven and the password mask. Overview, warnings and commands blocks re-read: no line derives from an
  amended passage. `USER:session-learnings` untouched.
- `.claude/docs/commands.md:27` and `.claude/docs/services/seven_guis.md:36` — the stand-check file lists gained
  `stand_snapshot_state`.
- Re-read and left: `.claude/docs/security-summary.md` and `.claude/rules/security.md` (neither carries the `id`,
  accessible-names, `disabled` or Dioxus-mutations rows of §Input Validation, so the new rows have no line to
  re-derive there); `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` (no `mutation_writer.rs` line
  citation; the two amended obs bullets changed citations only); `.claude/docs/conventions.md`, `gotchas.md`,
  `stack.md` (no statement from an amended section); `.claude/rules/testing.md`, `verification-harness.md` (no count
  or file list this chunk moved).

## Curation homes and judgment bases
0 rows in `CLAUDE.md` `USER:session-learnings`, any `## Session Additions`, `docs/session-learnings.md`,
`playbook.md` and `drift-base.md`.

## Lateral binds
`test-plan §3 ↔ obs-plan §3` (harness commands): the agent-run contract's shape is unchanged — only its Proof count
moved (41 → 48), which obs-plan does not state. `a11y-plan schema ↔ obs-plan schema`: no violation schema or log
schema changed. Both pairs stay consistent.
