# Cascade dispositions — 2026-10-07-audit-corrections

Written from the step-2 sweep listing (`cascade.py sweep`, patterns `cascade-patterns.toml`, baseline `48f8b5f2`,
trail `cascade-2026-10-07-audit-corrections.json`), after every body edit of the pass and before any sidecar entry.

## The search
Eleven patterns, each with a control that fired on the pre-pass masters:
- `unit-census` `(49|forty-nine) unit tests` · `seven-files` `(in|other) seven files` · `test-files` `snapshot_text\.rs and snapshot_diff\.rs` — the dioxus-native-dom census and its restatements.
- `one-file-per` `one file per behaviou?r` (case-insensitive) · `per-file` `repeated per file|restat` · `inline-tests` (the four wordings of "unit tests sit inside the source file") — the two layout conventions the pass qualified.
- `baseline-542` `542 passed` — the workspace count.
- `cite-elemid` and `cite-stand` — every old `file:line` range of the 21 moved citations and the 2 read by hand, by file and range.
- `obs-census` `in it or its checks` — the headless-stand census's old scope.
- `dioxus-key` `carries its Dioxus key` — the security row's key sentence.

Read over: the seven masters, the six files under `.andromeda/registries/`, the three curation homes, the two
judgment bases and the leaf bodies. Not looked for: a claim worded with none of these tokens; a citation into the
ten edited files by a range the line map did not list (covered separately: a regex read of every such citation in
the masters after the pass — 126 checked, 0 past the end of its file; and the same read over the leaves — 0
citations into those files).

## Rows
| Row | Disposition |
|---|---|
| `.claude/docs/services/dioxus-native-dom.md:41` · unit-census, seven-files · leaf | re-derived: 55 unit tests in eight files, the six bridge tests named |
| `.andromeda/test-plan.md:154` · seven-files · standing | no change — "the other seven files of slice s07", a search note about another crate |
| `CLAUDE.md:13` · one-file-per · leaf | re-derived: the stand checks share `tests/common/` |
| `.claude/rules/testing.md:15` · one-file-per · leaf | re-derived: the shared module named, no test target |
| `.claude/docs/conventions.md:6` · one-file-per · leaf | re-derived: the shared module named |
| `.andromeda/test-plan.md:315` · baseline-542 · standing, edited | no change to the match — the bullet is a chain of dated re-counts; the 542 reading stays as the previous chunk's, and this pass appended its own 548 after it |
| `.andromeda/architecture.md:114` · per-file · standing, edited | no change to the match — "a pointer-event builder is repeated per file" is still true of the three files it cites; the pass added the stand checks' shared module beside it |
| `.andromeda/security-plan.md:107` · dioxus-key · standing, edited | amended line; the matched clause is the kept half of the sentence, now followed by the refused-key clause. Read for a second statement of the claim on the line: none |
| `.andromeda/architecture.md:114` · inline-tests · standing, edited | no change to the match — the convention stands; the pass added its one exception after the citation group |
| `.andromeda/test-plan.md:43` · inline-tests · standing | no change — "inside `#[cfg(test)]` modules next to the code under test" holds for the bridge tests as worded (a `cfg(test)` module in a sibling file) |
| `.andromeda/test-plan.md:63` · inline-tests · standing, edited | amended line; the match is the kept rule, now with its exception |
| `.andromeda/test-plan.md:133` · inline-tests · standing, edited | amended line; the match is the kept rule, now with its exception |
| `.claude/rules/testing.md:14` · inline-tests · leaf | re-derived: the one out-of-line module named |

Zero-row patterns (each control fired): `cite-elemid`, `cite-stand`, `obs-census`, `test-files` — no old range, no
old census scope and no old file list stands in a master, a registry file, a curation home, a judgment base or a
leaf.

No `curation` row and no `base` row: nothing routes to P3 or to the propose-approve-append channel.

## Leaves re-derived (step 3)
Beyond the sweep's leaf rows, read by provenance for what the amended sections feed:
- `.claude/docs/tests-summary.md` — Local baseline: 548 · 0 · 5 over 131 at this chunk, ahead of the chain; Coverage as built: the shared module named.
- `CLAUDE.md` pointer table, the 7GUIs stand row — the shared tables and helpers' path added.
- Read and left: `.claude/rules/observability.md:18` and `.claude/docs/obs-summary.md:12` ("the headless stand and its checks install none" / "installs no subscriber" — a faithful compression of the widened sentence); `.claude/docs/a11y-summary.md` and `.claude/rules/a11y.md` (they name the stand checks by file, no line and no table); `.claude/docs/security-summary.md` and `.claude/rules/security.md` (neither states the Dioxus-key filter); `.claude/docs/services/dioxus-native-dom.md:15` (states `{tag}[{key}]` for a Dioxus-keyed root, as architecture's unchanged grammar does); `.claude/docs/commands.md:27` (the `--test stand_*` list — the shared module is no target).

## Lateral binds
`test-plan §3 ↔ obs-plan §3` (harness commands): neither side's harness contract changed. `a11y-plan schema ↔
obs-plan schema`: no schema changed.
