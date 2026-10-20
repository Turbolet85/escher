# Cascade dispositions — 2026-10-20-select-by-id (wrap 2026-10-20T05-49-12)

Search: `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-20-select-by-id.json`),
17 patterns derived from every amendment of the pass — the retired wordings («fourteen» driver-action checks, a
`<label>` bound to an `<input>` alone, `aria-selected` read on every element, `display: none` excluding every node,
the reader counts 27 and 21, 141 `.rs` files, 46 stand files, `run stand` 185, the 1062 tally, `accessibility_label`
10, «a click on any other element», every PROVISIONAL mark, `from_str`'s mapping, `type`'s help, the snapshot
builder's eighteen / 73) and the new fact's token (`select`). Every pattern's control fired.

Counts, copied from the listing: `total (17 patterns) · 119 rows over 25 files` · `per class · new 44/7 · standing
21/5 · leaf 47/16 · curation 1/1 · base 6/1`.

## new (44 rows) — this pass's own text
Every `new` row (aria-every ×3, hidden-excl ×1, fromstr ×1, select ×39) is a line this pass wrote, re-read for an
intra-line duplicate of a retired claim: none stands — each states the select model, the exception or a count as
amended.

## standing (21 rows)
- obs-plan.md:70 act14 @c5360 — no change: the census names «the fourteen driver-action checks» with their ordinals
  through `stand_act_transformed`, then «the fifteenth driver-action check, `stand_act_select`» beside it, so the
  fifteen are listed (the count left in place keeps that parenthesis's history anchored — classification rule 7)
- a11y-plan.md:105 aria-every — amended (the select-option exception stands on the line)
- a11y-plan.md:112, :272 hidden-excl — amended (the exception stands on each line)
- test-plan.md:119 stand46 @c10560 · run185 @c10521 — no change: the load-wait re-count clause, history; this pass's
  re-count follows it on the same line
- test-plan.md:334 tally1062 @c4471 · :340 ×2 — no change: the load-wait readings, history; this pass's follow them
- test-plan.md:119 acclabel10 @c9165 · :340 @c16844 — no change: the upstream sync's re-count clauses, history
- architecture.md:86 · security-plan.md:132 · a11y-plan.md:161 other-elem — amended (a click on a `select` named
  before «any other element» on each)
- architecture.md:196 · security-plan.md:364 typehelp — amended (the select sentence follows on each)
- architecture.md:99, :178 · security-plan.md:121 · a11y-plan.md:147, :150, :151 select — no change: true claims
  sharing the token (`select` among the form controls `is_disabled` reads; focusability)

## leaf (47 rows over 16 files)
Each leaf joins cascade step 3's set (`cascade.py leaves --add`): CLAUDE.md, `.claude/rules/{testing,a11y,security}.md`,
`.claude/docs/{obs-summary,tests-summary,a11y-summary,security-summary,design-summary,commands,gotchas,workflow}.md`,
`.claude/docs/services/{escher-driver,seven_guis,blitz-dom,blitz-test-harness}.md`.

## curation (1 row)
- docs/session-learnings.md:64 snap18 — no change: «73 unit tests» there is escher-driver's count in a learning's
  example, not the snapshot builder's

## base (6 rows)
- playbook.md:48, :50, :54, :62, :66, :70 provisional — no change: the rules' own wording of the PROVISIONAL class,
  not a mark on a ruled item

## Leaves
`cascade.py leaves` (pass mode; this chunk closes no epoch): 13 groups · 25 leaves, `--add` of the step-2 leaf rows'
16 files; `unmapped .claude/rules/host-linux.md (host)` — a host leaf no source states, no comparer's, no change.
One comparer per group, one batch inside one snapshot pair (before `tree 513841af…`, after `tree f7a6c05c…`, exactly
13 `A …/leaf-{group}.md`, index and head unchanged). Every finding's source side read at its cited line; every
`retired` finding drops a PROVISIONAL mark the masters dropped (the founder's ruling) or keeps the visual lint's
«in process only» with its owed verb.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | pass | 20 | 20 | 0 | 0 |
| architecture-1 | pass | 13 | 13 | 0 | 0 |
| architecture-2 | pass | 1 | 1 | 0 | 0 |
| architecture+layout-templates | pass | 2 | 2 | 0 | 0 |
| architecture+obs-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan | pass | 7 | 7 | 0 | 0 |
| architecture+test-plan | pass | 6 | 6 | 0 | 0 |
| design-system+layout-templates | pass | 1 | 1 | 0 | 0 |
| masters | pass | 10 | 10 | 0 | 0 |
| obs-plan | pass | 4 | 4 | 0 | 0 |
| security-plan | pass | 5 | 5 | 0 | 0 |
| test-plan-1 | pass | 12 | 12 | 0 | 0 |
| test-plan-2 | pass | 4 | 4 | 0 | 0 |

Copied from the apply call: `total · blocks 85 · applied 85`. After it, `git grep PROVISIONAL` over the masters, key
files and leaves (sidecars, the playbook's rule text, the handoff and `docs/session-learnings.md` aside) reads 0.
