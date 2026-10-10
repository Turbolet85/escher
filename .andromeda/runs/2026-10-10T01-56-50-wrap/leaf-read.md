# Leaf read — 2026-10-10-upstream-sync-agent-surfaces wrap, cascade step 3

The sweep reaches only wording a master's pre-pass text held. This is the second read: every leaf — `CLAUDE.md`, `.claude/docs/*.md`, `.claude/docs/services/*.md`, `.claude/rules/*.md`, the curation home `docs/session-learnings.md` excluded — searched for the amended FACTS, whatever their wording.

**The search** (one case-insensitive pattern over the 26 leaf files): `parley|comrak|accesskit_(unix|android)|icu|70 tests|78 tests|657|154 result|autofocus|a\[href\]|23354585|7832c177|unrounded|physical_unrounded|wpt_diff|wpt_area|one file per|pinning test|licen[cs]|@latest|to_file_path|url\.path|tomllib|Python 3|text-transform|writing-mode|LayoutPassState|TraversePartialTree|getSelection|innerText|inner_text|test_variants|tests/all|autotests|font-face|format hint|unsafe|110 passed|passed 110|run all|396|six test|four test|test files`, then the summaries' own headings for the amended sections.

## Re-derived (the leaf's sentence recomputed from the amended master)

| Leaf | From | What now reads |
|---|---|---|
| `CLAUDE.md` `GENERATED:setup:warnings` — the coupled-pins line | architecture [Dependency pinning] | the taffy git rev; parley is no git rev |
| `CLAUDE.md` `GENERATED:setup:overview` — the `tests/blitz-tests/` directory line | architecture §Conventions → Tests | one file and one test target per behaviour; upstream's `tests/all.rs` declared and never built |
| `CLAUDE.md` pointer table — the CI pipeline row | architecture §Standard Contracts → CI contracts | the blitz-tests target pin beside the workflow invariants |
| `.claude/docs/gotchas.md` — Coupled dependency pins | architecture [Dependency pinning] | taffy a git dep pinned by `rev`, parley a registry version |
| `.claude/docs/stack.md` — Text & fonts; CI scripts | architecture §Stack and Technologies | Parley 0.12 (registry), ICU4X 2.3 and writeable 0.6; Python 3.11 or later, upstream's `wpt_area_changes.py` |
| `.claude/docs/conventions.md` — the layout line | architecture §Conventions → Tests | each file its own target; `tests/all.rs` at `test = false`, pinned |
| `.claude/docs/workflow.md` — the upstream line | architecture §Project Intent | merged at `7832c177`, merge commit `9462a7e4`, 2026-10-10 |
| `.claude/docs/services/blitz-dom.md` — Consumes from; Internal conventions | architecture §Stack, [Default features], §Standard Contracts → Layout | Parley 0.12, ICU4X; the default list with `text-transform-icu`; `writing-mode` opt-in and what it selects; `LayoutPassState` |
| `.claude/docs/services/seven_guis.md` — the telemetry bullet | test-plan §9 → Engine features by runner | the workspace build also turns `writing-mode` on |
| `.claude/docs/tests-summary.md` — blitz-tests; Local baseline | test-plan §1, §2, §9 | one target per file with `tests/all.rs` unbuilt; 719 · 0 · 10 over 158 result lines, `run all` 396, the leg's 78 tests |
| `.claude/docs/security-summary.md` — `file:` reads; Font sources; Not yet measured | security-plan §Input Validation, §Dependency Security | the `to_file_path` read; the PROVISIONAL font-source widening; the ungated licence table and its owner |
| `.claude/docs/design-summary.md` — the UA stylesheet row | design-system §Color Palette | links are `a[href]` |
| `.claude/rules/security.md` — Untrusted input; Dependencies | security-plan §Input Validation, §Dependency Security | the PROVISIONAL font-source widening; the licence table no leg runs |
| `.claude/rules/testing.md` — Integration | test-plan §2 → Directory pattern | every other file its own target; no `autotests = false`, no second `[[test]]` table |

## Read and left

- `.claude/rules/security.md` "git deps are pinned by `rev`" — true of the one git dependency left; no change.
- `.claude/docs/security-summary.md` "comrak with `unsafe: true`" — comrak's option, another subject than the version bump.
- `.claude/docs/conventions.md` "`unsafe` blocks carry … a `// SAFETY:` comment" — a convention statement; the security plan's count is not restated in a leaf.
- `.claude/docs/services/dioxus-native-dom.md` and `.claude/rules/a11y.md`, the 27 boolean attributes with `autofocus` among them — the bridge is unchanged and still removes a falsy value; no leaf states the engine's `autofocus="true"` reading.
- `.claude/docs/a11y-summary.md`, `.claude/docs/obs-summary.md`, `.claude/docs/commands.md`, `.claude/rules/observability.md`, `.claude/rules/a11y.md`, `.claude/rules/verification-harness.md`, the other service notes — 0 hits of an amended fact; `run stand` · `run all` appear there as commands, not readings.
- `CLAUDE.md` Stack line ("Parley text") and the module table — name the library and the crates, no pin form and no amended symbol.
- No leaf states the accesskit adapter versions, the comrak version, the CI-scripts count of 70, the bounds reader's name, `wasm_hello`'s licence, the `@latest` apt action or the WPT script's CLI.

## Curation homes and judgment bases

The sweep printed 0 `curation` rows and 0 `base` rows: no preserve-verbatim home and neither `playbook.md` nor `drift-base.md` quotes a retired wording.
