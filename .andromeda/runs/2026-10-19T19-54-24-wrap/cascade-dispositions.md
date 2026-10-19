# Cascade dispositions — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

**Search:** `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`), baseline 67f17b9d.
It ran 12 patterns, each for a claim the pass amended, its old wording and its mechanism:
- `focuswithin` `focus-within`, and `nevermatch` "never match | always match false": the pseudo-class claim;
- `oldowner`: the retired CARRY owner title;
- `lastread` "last read at | 5 commits past": the upstream sync line;
- `files136`: the directory count;
- `wstally` and `perostally`: the workspace and per-OS tallies;
- `runall` "passed 546": the agent-run series;
- `redrawlist` "hover and device changes | request_redraw": the redraw triggers;
- `setfocus` `set_focus_to | clear_focus`: the focus mechanism;
- `sync1018`: the per-sync lock record;
- `wptpin`: the WPT pin.

The 13th, `tabredraw` (`Tab…redraw`, the Tab arm's own request), was refused: its control never fired on the pre-pass masters. It was dropped: no master stated the Tab arm's redraw, and `redrawlist` reaches the one site that lists redraw triggers (architecture.md:268, now :269).
Sections read: architecture §Established Decisions, §Cross-cutting Patterns, §Project Intent; a11y-plan §5; test-plan §1, §2, §3, §9; obs-plan §3; security-plan §Dependency Security.

**Counts** (copied from the listing): `total (12 patterns) · 35 rows over 11 files` · `per class · new 8/5 · standing 16/4 · leaf 11/6 · curation 0/0 · base 0/0`.
`oldowner` and `files136` read 0 rows with their controls fired: both stale sites were amended in this pass.

## Master rows
- architecture.md:85 focuswithin new · nevermatch standing edited → amended: `:focus-within` moved out of the always-false list. The "always match false" left stands for `:focus-visible`, `:valid`, `:invalid`, `:required`, `:target`, still true.
- architecture.md:103 focuswithin new · setfocus new ×2 → amended (the new [DOM semantics] detail bullet).
- architecture.md:279 focuswithin new · lastread standing edited → amended: the upstream sync line now reads `17de049b1`.
- architecture.md:151 nevermatch standing edited @c1101 → no change: "`ParseError>` never matching the scope node" is another fact sharing the token. It was read at the window.
- architecture.md:255 redrawlist @c360 → no change: a Dioxus channel then `request_redraw`, another fact.
- architecture.md:269 redrawlist standing edited → amended: Service wiring's redraw triggers gain a focus set or cleared.
- architecture.md:86 setfocus @c9030 · security-plan.md:132 setfocus @c1702 · a11y-plan.md:161 setfocus → no change: they name `set_focus_to` as a button click's focus path, still true. The not-in-document gate fires only for a node outside the document.
- security-plan.md:251 focuswithin new → amended (the new per-sync lock sibling).
- security-plan.md:250 sync1018 standing → no change: the 2026-10-18 record, true as dated.
- test-plan.md:27 focuswithin new → amended (§1 scope gains the two upstream targets).
- test-plan.md:330 wstally · perostally standing edited → amended: the new reading was appended. The older readings stand as dated history in the series.
- test-plan.md:336 wstally standing edited ×3 → amended: the re-count was appended. The 194 · 1001 reading stands as the dated 2026-10-18 entry.
- test-plan.md:118 runall standing edited → amended: the re-count was appended. 546 stands as the dated 2026-10-17 entry.
- test-plan.md:59 · test-plan.md:226 wptpin → no change: :59 is the pin of its own dated reading, and :226 names the file without a value.
- obs-plan.md:77 focuswithin new → amended (the new silence-census bullet).
- a11y-plan.md:155 focuswithin new · nevermatch edited → amended.

No intra-line duplicate of a retired claim stands on an amended line: :85, :155 and :279 were each re-read whole.

## Leaf rows (join step 3's set)
- .claude/rules/a11y.md:34 focuswithin · nevermatch → stale: "`:focus-visible` and `:focus-within` never match".
- .claude/docs/a11y-summary.md:23 focuswithin · nevermatch → stale: the same claim.
- .claude/docs/gotchas.md:41 focuswithin · nevermatch → stale: `:focus-within` in the always-false list.
- .claude/docs/workflow.md:15 lastread ×2 → stale: "last read at `fb8d6c15e`, 5 commits past it".
- .claude/docs/tests-summary.md:30 wstally · runall → stale: 194 · 1001 and `run all` 546.
- .claude/rules/a11y.md:33 · .claude/docs/services/blitz-dom.md:15 setfocus → true: a button click's focus path. Re-derived with their groups.

## Leaves
`cascade.py leaves --add` (the six leaf rows above) printed 11 groups in pass mode, 24 leaves. `re-pointed .andromeda/design-system.md` (digits alone) is no changed source. `unmapped .claude/rules/host-linux.md (host)` is the host's own leaf: no source states it, so no comparer and no step-3 edit.
One comparer per group ran in one batch inside a snapshot pair: before `tree 639df991…`, after `tree 89a53764…`, index and head unchanged, 11 `A` lines. Apply total: `total · blocks 9 · applied 9`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 3 | 3 (gotchas.md:41 stale; services/blitz-dom.md:15, :19 omission) | 0 | 0 |
| architecture-2 | pass | 1 | 1 (workflow.md:15 stale) | 0 | 0 |
| architecture+security-plan+test-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan+obs-plan+a11y-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan | pass | 0 | 0 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 0 | 0 | 0 | 0 |
| test-plan-1 | pass | 2 | 2 (tests-summary.md:21 omission, :30 stale) | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 1 | 1 (obs-summary.md:49 omission) | 0 | 0 |
| a11y-plan | pass | 2 | 2 (a11y-summary.md:23, rules/a11y.md:34 stale) | 0 | 0 |

Each finding's source side was checked against the body as amended this pass (architecture.md:85, :103, :279; a11y-plan.md:155; obs-plan.md:77; test-plan.md:27, :336). Every one of the six leaf rows the sweep marked stale is written.
