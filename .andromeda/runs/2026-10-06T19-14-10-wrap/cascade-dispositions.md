# Cascade dispositions — 2026-10-06-snapshot-model

The sweep ran after every body of this pass was applied (`cascade.py sweep`, trail `cascade-2026-10-06-snapshot-model.json`,
patterns `cascade-patterns.toml`, baseline `ee88e85e`). It ran twice: once after the master edits, once after the leaf
re-derivation; the rows below are the second listing. Every pattern's control fired on the pre-pass masters.

## What was searched
- `override-only` (`accessibility_tree` + "override") and `gets-override` — the claim that dioxus-native-dom's `accessibility` feature gates only the override.
- `module-list` — dioxus-native-dom's module list.
- `lib-rs-cites` — every citation into `packages/dioxus-native-dom/src/lib.rs`.
- `only-exit` — the id's and the names' only exit from the process.
- `count-454`, `count-25-ok` — the workspace baseline and the stand `ok` count.
- `three-unit`, `two-files` — dioxus-native-dom's unit-test count and the search note beside it.
- `stand-coverage` (`Tab order`) — the stand coverage enumeration and its restatements.
- `snapshot-planned` — any text still describing the snapshot as not built.
- Not searched: the Stylo sense of "snapshot" (element-snapshot invalidation), which this chunk does not touch.

## Rows
| Row | Disposition |
|---|---|
| architecture.md:134 `override-only` new @c6525 | amended — this pass's own snapshot clause ("builds from that `accessibility_tree` override") |
| architecture.md:189 `override-only` new | amended — "gates its `accessibility_tree` override and its `snapshot` module" |
| a11y-plan.md:55 `override-only` standing edited | amended — the same duplicate claim; now names the snapshot model, citation `lib.rs:7-8` · `17-18` |
| CLAUDE.md:33 `override-only` leaf | re-derived — the dioxus-native-dom module line now names the snapshot |
| .claude/docs/conventions.md:25 `override-only` leaf | re-derived — "and its `snapshot` module" |
| .claude/docs/services/dioxus-native-dom.md:15 `override-only` leaf | no change — the stable-id bullet is still true |
| .claude/docs/services/dioxus-native-dom.md:16 `override-only` leaf | re-derived — the new Snapshot model bullet |
| architecture.md:134 `gets-override` standing edited @c6060 | amended — "gets the override and the snapshot model only by naming the feature" |
| architecture.md:9 `lib-rs-cites` standing | no change — `lib.rs:3` is an unmoved line |
| architecture.md:105 `lib-rs-cites` standing | no change — `lib.rs:1` is an unmoved line |
| architecture.md:110 `lib-rs-cites` standing edited ×3 | amended — `33-56` → `38-61`, `47-50` → `52-55`, `5-10` → `5-11`; the window at each offset read |
| architecture.md:134 `lib-rs-cites` new @c8391 | amended — this pass's own citation `lib.rs:23-24` |
| architecture.md:251 `lib-rs-cites` standing edited | amended — `12-16` → `13-19`, the row now lists `snapshot` |
| a11y-plan.md:55 `lib-rs-cites` standing edited ×2 | amended — `lib.rs:7` → `7-8`, plus `17-18` |
| a11y-plan.md:281 `lib-rs-cites` standing edited | amended — `lib.rs:7` → `7-8` |
| security-plan.md:107 `only-exit` standing edited ×2 | amended — both hits read at their offsets: the adapter clause stands with its ratification stamp, and the new clause says the adapter stays the id's only exit |
| security-plan.md:108 `only-exit` standing edited | amended — the names row gains the in-process snapshot reader; the adapter claim stands |
| test-plan.md:314 `count-454` standing edited @c2025 | amended — the 454 link stays as history; a 471 · 0 · 5 link over 125 result lines follows it |
| .claude/docs/tests-summary.md:28 `count-454` leaf | re-derived — the baseline now opens at 471 · 0 · 5, with 454 as its predecessor |
| test-plan.md:111 `count-25-ok` standing edited | amended — the 25 link stays as history; a 33 link follows it |
| test-plan.md:24 `two-files` new | amended — the search note now names four files |
| test-plan.md:25 `stand-coverage` standing edited @c1075 | amended — the sentence gains the snapshot checks and `stand_snapshot.rs:1` |
| a11y-plan.md:269 `stand-coverage` standing | no change — the accessibility-id bullet is still true; the snapshot bullet follows it |
| .claude/docs/tests-summary.md:21 `stand-coverage` leaf | re-derived — the coverage line gains the snapshot checks |
| architecture.md:207 `snapshot-planned` standing | no change — the sentence describes README.md, which this chunk did not touch and which still lists the snapshot as planned (no snapshot command exists yet) |

A row of the first listing no longer printed in the second: a11y-plan.md:313 `stand-coverage` ("no test asserts a
Dioxus control's focusability or Tab order"). It was stale before this chunk (`stand_accessibility_ids.rs`'s
`tab_order_is_unchanged` already asserted the focus sequence) and this chunk's `state_reads_the_engine` contradicts it
too, so it was amended in this pass to name both checks. Its leaf, `.claude/docs/a11y-summary.md:23`, already read true.

Zero-row patterns: `module-list` and `three-unit` — both controls fired on the pre-pass text, and the amended lines no
longer carry the retired wording.

## Leaves re-derived (step 3)
- architecture → CLAUDE.md (`modules`: dioxus-native-dom · `pointer-table`: a Snapshot model row · `architecture`: the snapshot's data model built) · `.claude/docs/conventions.md` · `.claude/docs/services/dioxus-native-dom.md` (Publishes to · Entry points · Testing) · `.claude/docs/commands.md` (the stand-check command gains `stand_snapshot`) · `.claude/docs/services/seven_guis.md` (the stand-check file list).
- test-plan → `.claude/docs/tests-summary.md` (coverage line · local baseline). `.claude/rules/testing.md` and `verification-harness.md`: read, nothing derived from an amended passage.
- a11y-plan → `.claude/docs/a11y-summary.md` (the identity owner line: snapshot leg built) · `.claude/rules/a11y.md` (the override line names the snapshot model).
- security-plan → `.claude/docs/security-summary.md` and `.claude/rules/security.md`: read (grep `element_id`, `author_id`, `stable element id`: 0 hits in either), nothing derived from the two amended rows.
- Curation homes and judgment bases: 0 rows on every pattern.
