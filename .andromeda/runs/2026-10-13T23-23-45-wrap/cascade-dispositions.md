# Cascade dispositions — 2026-10-13-stand-keyboard-harness (P2 Apply, resumed 2026-10-14)

## The search
Patterns: `cascade-patterns.toml` in this run dir, nine patterns, each the retired claim's own verbs and phrasings:
- `disabled-parse` — focusability parsing `disabled` as a bool, "stays focusable", "read/keyed two ways";
- `click-no-focus` — a click leaving no element focused;
- `inert-claim` — "no longer makes content inert", the unmeasured falsy `inert`;
- `names-27` — the 27-name count;
- `common-readers` — the shared module's 18 readers;
- `crud-keys` — the CRUD Tab order unchanged, no keyboard reaching an option, the list with no `tabindex`;
- `old-counts` — 181 · 915, `run stand` 141, 120 files · 119 targets, 6 + 6 + 3, 40 tests, `stand_act_keys` 2, the 9 changing
  controls, the eight bounds-only Book nodes;
- `keyup-default` — KeyUp's default action (none before);
- `apple-binding` — no standing check calling `apple_keybinding` on Linux or Windows.

Sections read: every body section the 43 proposals name, and each master row below at its line. Not looked for: a
restatement of the retired claims in words none of the nine patterns holds — the detectors' own sweeps of their
masters stay the control for reach.

## The listing
First run: `cascade-sweep.txt`. Its reads found three master sites no proposal carried, folded into this pass as
routine amendments (playbook :28, :64):
- architecture.md:133 (`click-no-focus`, standing): Driver session's `covered` reading said "after the accepted click
  no element reads `focused`" — stale after F2. Amended: architecture 10.
- a11y-plan.md:280 (`crud-keys`, standing): "with the Tab order unchanged", in the present tense — stale for CRUD after
  F5. Amended: a11y-plan 15 (bound to its chunk, CRUD's restatement named).
- registries/contracts/test-plan/session-lifecycle.md:11 @c2011 (`old-counts`, standing): `stand_act_keys` 2 and the
  thirteen driver-action files' 46 — stale after `stand_act_keys` +1. Amended: test-plan 16 (3, and 47); `registry.py
  check` 0 defects.

Second run, after the folds: `cascade-sweep-2.txt` — total (9 patterns) · 35 rows over 12 files · per class new 5/3 ·
standing 12/6 · leaf 18/6 · curation 0/0 · base 0/0. `click-no-focus` and `apple-binding` read 0 rows, their controls
fired. Every row:

### new (5) — this pass's own text
- architecture.md:91 `disabled-parse` — "before it focusability parsed the value as a bool": the retired reading named as
  history. No change.
- a11y-plan.md:148 `disabled-parse` — the same history clause. No change.
- architecture.md:123, security-plan.md:127, a11y-plan.md:166 `keyup-default` — the new KeyUp default action. No change.

### standing (12)
- architecture.md:263 `inert-claim` @c4927 — the amended text "neither a truthy nor a falsy `inert`". True. No change.
- architecture.md:136 `names-27` @c1188 — "the 27 names before `inert` the module-private `BOOLEAN_ATTRIBUTES` list":
  the retired list's own count. True. No change.
- security-plan.md:144 `names-27` — the same history. True. No change.
- architecture.md:263 `common-readers` @c5678 — the amended text, `stand_contrast` the eighteenth beside `stand_keyboard`
  the nineteenth. True. No change.
- test-plan.md:101 `common-readers` @c3206 — `session_common`'s eighteenth reader, `stand_mcp_tools`: another module.
  True. No change.
- obs-plan.md:70 `common-readers` ×2 — the amended text and its as-measured list. True. No change.
- a11y-plan.md:280 `crud-keys` — the folded text, "unchanged at that chunk". True. No change.
- a11y-plan.md:282 `crud-keys` @c1332 — the amended text, "at that chunk", followed by this chunk's restatement. True.
- test-plan.md:116 `old-counts` @c7264 — the dated re-count chain, 141 at 2026-10-13-stand-contrast-harness, followed
  by this chunk's 153. True. No change.
- test-plan.md:330 `old-counts` ×2 — the dated chain, 181 · 915 at 2026-10-13-range-input-interaction, followed by this
  chunk's 182 · 928. True. No change.
- session-lifecycle.md:11 `old-counts` @c4932 — CI run 37673662374's reading "`stand_act_keys` 2 read as run": a
  dated record of that run. True. No change.
- a11y-plan.md:173 `keyup-default` — "Every key press and release is dispatched as KeyDown or KeyUp after shell shortcuts
  are checked": the shell's dispatch. True. No change.

### leaf (18 rows over 6 files) — each leaf joins step 3's set
- `.claude/rules/a11y.md`:31, :32 ×2, :32 (`disabled-parse`, `inert-claim`)
- `.claude/docs/a11y-summary.md`:22 ×2, :24 ×3, :24, :24 (`crud-keys`, `disabled-parse`, `inert-claim`, `old-counts`)
- `.claude/docs/services/dioxus-native-dom.md`:17, :41, :41 (`disabled-parse`, `inert-claim`, `names-27`)
- `.claude/docs/tests-summary.md`:21 ×4, :30 (`inert-claim`, `names-27`, `crud-keys`, `old-counts`)
- `.claude/docs/services/seven_guis.md`:30 ×2, :38 (`crud-keys`, `common-readers`)
- `.claude/docs/services/escher-driver.md`:61 (`old-counts`, the twelve files' 40 tests)

curation 0 · base 0.

## Binds
- test-plan §3 ↔ obs-plan §3 (harness commands): no harness command changed — the agent-run contract and the CI legs are
  as they were; obs-plan §3's census gained `stand_keyboard`, test-plan §1 lists it. Consistent.
- a11y-plan schema ↔ obs-plan schema (violation schema): neither changed.

## Leaves
Set: `leaves-set.txt` — pass mode (this chunk does not close its epoch: `route-epoch.txt` reads Epoch 6 markerless 4),
11 groups · 25 leaves · unmapped 1 (`.claude/rules/host-linux.md`, the host leaf: no master states it, no comparer).
The step-2 leaf rows all sit in the set's groups. One batch of 11 comparers inside one snapshot pair: before `tree
746f245b… index c94b6a43… head 71af33d0…`, after `tree 1a6edc8c…`, index and head unchanged; `git diff-tree` printed
exactly the eleven `A …/leaf-{group}.md` lines. Every receipt count equals its file's block count (`leaves-dry.txt`).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | pass | 15 | 15 | 0 | 0 |
| architecture-1 | pass | 5 | 5 | 0 | 0 |
| architecture-2 | pass | 4 | 4 | 0 | 0 |
| architecture+layout-templates | pass | 2 | 2 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan (CLAUDE.md) | pass | 5 | 5 | 0 | 0 |
| architecture+test-plan | pass | 2 | 2 | 0 | 0 |
| layout-templates | pass | 1 | 1 | 0 | 0 |
| obs-plan | pass | 1 | 1 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| test-plan | pass | 7 | 7 | 0 | 0 |

`leaves-apply.txt`: total · blocks 44 · applied 44. Each finding's source side was read at its cited master text before
the write. The omissions applied are those a reader of the leaf would otherwise act against: the keyboard activation
and click focus (a reader would expect no key to activate a button), `disabled` by presence, the CRUD list's Tab stop
and keys, the `inert` limit, and `stand_keyboard` in the stand-check listings.

Re-sweep after the write, `cascade-sweep-3.txt`: total (9 patterns) · 21 rows over 10 files · leaf 4/4, each this
pass's own true text — dioxus-native-dom.md:41 (no `inert` either way), seven_guis.md:38 (`stand_contrast` the eighteenth
beside `stand_keyboard` the nineteenth), tests-summary.md:30 (the dated 181 · 915 history behind 182 · 928),
blitz-dom.md:15 (KeyUp's new default action).
