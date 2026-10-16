# Cascade dispositions — 2026-10-16-handler-splits-and-readme

Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-16T14-54-43-wrap/cascade-patterns.toml`, baseline
d236e918 (the pre-CI parent). Patterns, each spanning the seven masters and the registries for the amended claims' own
wording and mechanism:
- `front-page` — the README's retired 0.1.0-era phrasings (`built vs planned|being built|is built in six stages|Beyond 0\.1\.0`, ignore case);
- `old-cites` — the five pre-chunk citations whose cited text moved into a new function;
- `arms` — the mechanism names the moved bodies were cited under (`button arm|range arms?|parse_float|keep their signatures`);
- `focus-mech` — the button and range click's focus mechanism (`generate_focus_events`), whose call moved into `click_button`.
Every control fired on the pre-pass masters (architecture.md:210, :87).

Listing: total (4 patterns) · 17 rows over 8 files · per class · new 0/0 · standing 11/3 · leaf 6/5 · curation 0/0 · base 0/0.

- architecture.md:210 front-page standing edited — amended (this pass's Front page edit; the one standing hit is the amendment's own "where it read built vs planned", naming the retired reading) → no further change
- old-cites — 0 rows (control fired): every pre-chunk citation re-pointed by the citation sweep (`citation-dispositions.md`) → nothing standing
- architecture.md:87 arms ×2 — "the text-input and range arms" (Enter's order) and "a button arm" (the 2026-10-13 change's history) — true claims; the arm still matches and calls `click_button` → no change
- architecture.md:123 arms ×6 (+2) — Enter's order, the button arm citation (amended, `click_button` cited), and the signatures sentence (amended this pass) → amended / no change
- security-plan.md:127 arms — `parse_float` reads the four attributes, true; its steps cited this pass → amended
- security-plan.md:128 arms ×2 — Enter's order, the button arm (amended, `click_button` cited) → amended / no change
- a11y-plan.md:166 arms — Enter's order after the text-input and range arms, true → no change
- architecture.md:87 · :123 · security-plan.md:128 · a11y-plan.md:158 · :161 focus-mech — focus through `generate_focus_events` (/ `set_focus_to`): still the mechanism, now inside `click_button` (pointer.rs:890-907) for the button and in the range click arm (pointer.rs:685-697) unchanged; the button sites gained `click_button`'s citation this pass → amended / no change
- CLAUDE.md:75 arms leaf — the pointer-table row naming "the button arm of `handle_click`" → leaf, joins step 3's set
- .claude/rules/security.md:17 arms leaf — `parse_float` reading → leaf, joins step 3's set
- .claude/docs/security-summary.md:13 arms leaf → leaf, joins step 3's set
- .claude/docs/services/blitz-dom.md:15 arms ×2 · focus-mech leaf → leaf, joins step 3's set
- .claude/rules/a11y.md:33 focus-mech leaf → leaf, joins step 3's set

## Leaves
`cascade.py leaves` (pass mode; `--add` the five step-2 leaves): 10 groups · 24 leaves · unmapped 1. One comparer per
group for eight groups, one batch inside one snapshot pair (before `tree 31d2ccb9…`, after `tree b3fa8be1…`, index and
head unchanged; `git diff-tree` printed exactly the eight `A {run_dir}/leaf-{group}.md` lines).
- `test-plan` (tests-summary.md · testing.md · verification-harness.md) and `obs-plan` (obs-summary.md · observability.md) —
  no comparer: those masters differ from the baseline only by the citation sweep's digit re-points (a re-pointed master
  is not a changed source, citation-contract §Dispositions), and the sweep listed no row in their leaves → declined, no change.
- `unmapped .claude/rules/host-linux.md (host)` — the host's leaf, stated by no master → no comparer, no change.
- architecture-1 · pass · findings 2 · applied 2 · by hand 0 · declined 0 (blitz-dom.md:15 ×2, omission: the range arms'
  and the button arm's bodies now private named functions)
- architecture-2 · pass · findings 0
- architecture-3 · pass · findings 0
- architecture+security-plan+test-plan+obs-plan+a11y-plan (CLAUDE.md) · pass · findings 1 · applied 1 (CLAUDE.md:75, the
  pointer row naming `click_button`)
- architecture+test-plan · pass · findings 0
- architecture+obs-plan · pass · findings 0
- security-plan · pass · findings 0
- a11y-plan · pass · findings 0
Tool: `total · blocks 3 · applied 3`.

Not looked for: the README's own text (outside the masters, read by the plan's README entries 9–12); a master stating
the chunk's complexity figures (the P1 site sweep's `complexity` pattern read 2 standing hits, both a11y-plan §8
"Cognitive Accessibility", another subject).
