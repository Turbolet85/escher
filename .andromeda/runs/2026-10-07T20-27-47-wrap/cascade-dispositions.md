# Cascade dispositions — 2026-10-07-refusal-detection

Written from the listing of `cascade.py sweep` over `cascade-patterns.toml` (21 patterns), run
twice: after every body was amended (79 lines) and again after the leaves were re-derived. The
trail is `cascade-2026-10-07-refusal-detection.json`. Baseline: `9b758f6c`, the parent of the
chunk's one pre-CI commit. Every pattern's known-positive control fired on the pre-pass masters.

## What was searched

- **Wordings and mechanisms of the retired claims**: the verb count and the verb list written
  out (`five-verbs`, `verb-list`, `four-acting`); which causes code returns (`yet-returns`); the
  session keeping nothing between calls (`keeps-no-id`); the lookup answering `not-found` alone
  (`not-found-only`); a click on a target that cannot take it running (`click-runs`); the
  `disabled` cause named and not detected (`named-only`); the six check files and their counts
  (`six-act`, `readers`, `counts`); one scroller per programmatic scroll (`one-scroller`); the
  closed `Outcome` list (`outcome-list`); `bounds` as viewport-relative without exception
  (`bounds-claim`).
- **Old coordinates** of every citation the chunk's edits moved, by file (`cite-scroll`,
  `cite-execute`, `cite-session`, `cite-schema`, `cite-harness`, `cite-common`, `cite-checks`).
- **Scope**: the seven masters, every file under `.andromeda/registries/`, the three curation
  homes, the two judgment bases and the leaf bodies.
- **Not searched for**: the old `off-screen` texts (0 hits before the pass by fixed-string count,
  `fanout-results.md` check 4); wordings of claims this pass did not retire.

## Zero-row patterns (after the pass)

`four-acting` · `click-runs` · `readers` · `one-scroller` · `cite-scroll` · `cite-execute` ·
`cite-session` · `cite-schema` · `cite-harness` · `cite-common` · `cite-checks` — each control
fired on the pre-pass text, so each zero says the retired wording or coordinate stands nowhere in
the swept set. `five-verbs`, `yet-returns` and `named-only` read zero master rows after the first
sweep and zero leaf rows after the second.

## Master and registry rows — every one

| row | disposition |
|---|---|
| `architecture.md:133` `verb-list` new | no change — this pass's own text: "`press` and `advance` are refused for no screen-level cause", a true claim sharing the two tokens |
| `security-plan.md:76` `verb-list` new | no change — the same new sentence in the schema row |
| `test-plan.md:179` `verb-list` edited | amended — the list now ends `· scroll` |
| `session-lifecycle.md:5` `verb-list` edited | amended — the list now ends `· scroll` |
| `security-plan.md:116` `keeps-no-id` edited | amended — "the lookup is made anew on every call" stands and is true; "the session keeps no id" is retired for the record |
| `architecture.md:133` `not-found-only` edited | amended — "resolve their id first, on the screen as it reads now", answering `stale` or `not-found` |
| `test-plan.md:26` `not-found-only` ×2 | no change — both true: an id no element carries and no screen read is `not-found` (`stand_act_ids`), and a `scroll` naming no element is `not-found` (`stand_act_scroll`, this pass's text) |
| `test-plan.md:115` `six-act` edited | no change to the hit — the dated re-count "at 2026-10-07-act-by-id … the six `stand_act_*` files" is true of that chunk; this chunk's re-count is appended after it |
| `test-plan.md:324` `six-act` edited | no change to the hit — the same dated history; this chunk's re-count appended |
| `obs-plan.md:69` `six-act` edited | no change to the hit — the provenance tail "the six `stand_act_*` checks at …act-by-id" is that chunk's measurement; this chunk's is appended |
| `session-lifecycle.md:11` `six-act` ×2 | no change to either hit — "the six driver-action files and the crate's 26 at …act-by-id" and "all six driver-action files read as run" (CI run 37633611745) are dated history; the current count line reads nine |
| `test-plan.md:99` `counts` edited | no change to the hit — "26 rows" is the `rendered(task)` table, another subject; the line's driver-action count was amended |
| `architecture.md:136` `bounds-claim` edited | amended — the claim now carries its measured exception |

Every amended line was re-read whole for a second statement of its retired claim; none stands.

## Curation homes and judgment bases

No row: no pattern fires in CLAUDE.md's `USER:session-learnings`, in a rule file's
`## Session Additions`, in `.claude/docs/session-learnings.md`, in `playbook.md` or in
`drift-base.md`. One Session Addition restates a list this pass grew — `testing.md`'s generated
body, not its additions — and is covered below.

## Leaf rows

First sweep, 21 leaf rows over 8 files; each leaf was re-derived from its amended master, then
swept again. Rows that still fire after the re-derivation, and why they stand:

| row | disposition |
|---|---|
| `CLAUDE.md:37`, `:105` `verb-list` | re-derived — both lists now end `· scroll` |
| `.claude/rules/testing.md:15` `verb-list` | re-derived — the call builders now end `· scroll`, with the two refusal readers |
| `.claude/docs/tests-summary.md:30` `six-act` | no change to the hit — the dated baseline entry for act-by-id; this chunk's entry leads the line |
| `.claude/docs/services/escher-driver.md:19` `outcome-list` | re-derived — the list now ends `· Scrolled { … }` |

Leaves re-derived in this pass, by source: **architecture** → `CLAUDE.md` (modules: `blitz-dom`,
`escher-driver`; the pointer table's Driver executor row; the architecture block),
`.claude/docs/commands.md`, `.claude/docs/services/{escher-driver,blitz-dom,blitz-test-harness,dioxus-native-dom}.md` ·
**security-plan** → `.claude/docs/security-summary.md`, `.claude/rules/security.md` ·
**design-system** and **layout-templates** → `.claude/docs/design-summary.md` · **test-plan** and
its key file → `.claude/docs/tests-summary.md`, `.claude/rules/testing.md`,
`.claude/rules/verification-harness.md` · **obs-plan** → `.claude/docs/obs-summary.md`,
`.claude/rules/observability.md` · **a11y-plan** → `.claude/docs/a11y-summary.md`.

Left as they stand — searched, not read whole: one grep over every file under `.claude/docs/`
(bar `session-learnings.md`) and `.claude/rules/` for the amended subjects (`scroll_into_view`,
`Session::run`, `not-found`, `stale`, `stand_act`, `session_common`, `verbs`, `bounds`, the counts
629 · 149 · 97 · 26 files, `apple_keybinding`, `off-screen`, `covered`, `keeps no`, `clamp`,
`Smooth`, `nested`, `VERBS`, `hit`) returned no line stating an amended claim in
`.claude/docs/conventions.md`, `gotchas.md`, `stack.md`, `workflow.md`,
`.claude/docs/services/{blitz-paint,blitz-shell,blitz-traits,escher-telemetry,seven_guis}.md` or
`.claude/rules/a11y.md` (its `disabled` bullets state the two keyings, not the driver's cause).
CLAUDE.md's overview, warnings and workflow blocks are unchanged: no listener, port, env var,
crate or command moved.

## Binds

- `test-plan §3 ↔ obs-plan §3`: both name nine `stand_act_*` checks and fourteen readers of
  `session_common/mod.rs`; the agent-run contract's five commands, status shape and log format are
  untouched (`scripts/agent-run.sh` unedited).
- `a11y-plan schema ↔ obs-plan schema`: neither moved.
- `registry.py check --project . --master .andromeda/test-plan.md`: 0 defects.
