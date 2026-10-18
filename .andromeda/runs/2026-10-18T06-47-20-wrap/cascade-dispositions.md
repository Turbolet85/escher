# Cascade dispositions — 2026-10-18-upstream-sync-determinism-and-traces

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-18-upstream-sync-determinism-and-traces.json`), baseline `fb842671`, run after the last of the pass's eight body edits. Seven patterns, each the amended claim's own wording and its mechanism's verbs:
- `sync-read` — the sync line's verbs: «last merged at», «`main` (last) read at», «read at `1e5479ae`», «of which the pin took», «wait for the next sync», «cherry-pick».
- `owner-title` — the CARRY 3 owner title retired at `architecture.md:90`.
- `cursor` — «over no box», «Default otherwise», «hides the cursor», `get_cursor`, `set_cursor`.
- `taffy-pin` — «Taffy stays the registry», «registry "0.15"», «no git dependency», «Taffy-main».
- `dom-tests` — the §1 blitz-dom counts' old wording.
- `hover-tests` — «two `hover_state_tests`», `hover_state_tests`.
- `tally` — the retired and the skipped readings: CI#39056637407, CI#39100630123, 187 · 971, 971 passed, 187 result lines, 192 · 990, 990 passed.

Sections read: architecture §Established Decisions and §Project Intent; security-plan §Dependency Security; design-system §Iconography; test-plan §1, §8, §9. Every control fired on the pre-pass masters.

total (7 patterns) · 22 rows over 9 files · per class · new 3/2 · standing 13/4 · leaf 6/5 · curation 0/0 · base 0/0

- `.andromeda/architecture.md:233` sync-read standing edited mixed ×5 → amended (the sync line; «last merged at 23b9b26a9» stays true, the base unmoved; read once whole for an intra-line duplicate: none)
- `.andromeda/security-plan.md:246` sync-read new → amended (this pass's Pinning record)
- `.claude/docs/workflow.md:15` sync-read leaf ×4 → leaf, re-derived at step 3
- `.andromeda/architecture.md:160` cursor standing @c32995 → no change: `covered`'s hit over no box reading as the root element, another subject (window read at @c32995)
- `.andromeda/architecture.md:233` cursor new → amended (the sync line names the taken commit)
- `.andromeda/design-system.md:316` cursor standing edited mixed ×2 → amended
- `.andromeda/design-system.md:317` cursor standing ×2 → no change: `ShellProvider::set_cursor` takes an optional icon and `None` hides the cursor — true, `None` still flows for `cursor: none`
- `.andromeda/architecture.md:72` taffy-pin standing → no change: [Dependency pinning], no git dependency — true
- `.andromeda/architecture.md:233` taffy-pin standing edited ×2 → amended
- `.andromeda/security-plan.md:242` taffy-pin standing → no change: no git dependency — true
- `.andromeda/security-plan.md:245` taffy-pin standing ×2 → no change: the 2026-10-17 sync record, true as dated
- `.andromeda/security-plan.md:246` taffy-pin new ×3 → amended (this pass's record)
- `CLAUDE.md:51` taffy-pin leaf → leaf, re-derived at step 3
- `.claude/rules/security.md:28` taffy-pin leaf → leaf, re-derived at step 3
- `.claude/docs/gotchas.md:7` taffy-pin leaf → leaf, re-derived at step 3
- `.claude/docs/workflow.md:15` taffy-pin leaf ×2 → leaf, re-derived at step 3
- `.andromeda/test-plan.md:11` dom-tests standing edited → amended
- `.andromeda/test-plan.md:293` hover-tests standing edited mixed ×3 → amended (the two standing tests' clause kept, true; the third added)
- `.andromeda/security-plan.md:239` tally standing @c3471 → no change: the audit ignores' dated reading on CI#39056637407 (window read at @c3471)
- `.andromeda/test-plan.md:325` tally standing edited mixed ×5 → amended (CI#39056637407 kept as the series' prior reading the move is counted from; CI#39100630123 named as the skipped reading, on the operator's direction, inputs#I7 (b))
- `.andromeda/test-plan.md:331` tally standing edited mixed ×4 → amended (187 · 971 kept as the 2026-10-17 re-count and the move's origin; 192 · 990 named as never written)
- `.andromeda/test-plan.md:331` tally @c+17356 → amended (the same line's new clause)
- `.claude/docs/tests-summary.md:30` tally leaf @c282 → leaf, re-derived at step 3

`owner-title`: 0 rows, its control fired (`architecture.md:90` pre-pass) — no master, registry, leaf, curation or base site still names the retired owner.

## Leaves
`cascade.py leaves` (pass mode — the chunk closes no epoch): 9 groups · 21 leaves; `unmapped .claude/rules/host-linux.md (host)` — the host leaf, derived from no master, no comparer. One batch of nine comparers inside one snapshot pair (before `tree a7fbf660…`, after `tree 5944f21e…`: nine `A` lines, index and head unchanged). `leaf-design-system.md` was refused by `leaves --apply --dry-run` (`read:` 44 lines stated, 43 held) → moved aside to `.raw-leaf-design-system.md` and re-spawned once in its own snapshot pair (before `tree 169cd24c…`, after `tree dd548d3e…`: one `A` line), 0 findings.

total · blocks 2 · applied 2

- architecture-1 · pass · 0 findings
- architecture-2 · pass · 0 findings
- architecture-3 · pass · 1 finding · applied 1 (`.claude/docs/workflow.md:15` stale — the sync line restated as `architecture.md:233` now reads) · by hand 0 · declined 0
- architecture+security-plan+design-system+test-plan (CLAUDE.md) · pass · 0 findings
- architecture+test-plan · pass · 0 findings
- security-plan · pass · 0 findings
- design-system · pass · 0 findings (the re-spawn)
- test-plan-1 · pass · 1 finding · applied 1 (`.claude/docs/tests-summary.md:30` stale — the local tally, `test-plan.md:331`'s new re-count) · by hand 0 · declined 0
- test-plan-2 · pass · 0 findings

Leaf rows of the step-2 sweep: `workflow.md:15` re-derived (applied); `CLAUDE.md:51`, `.claude/rules/security.md:28`, `.claude/docs/gotchas.md:7` — compared, true, no finding (no git dependency left, Taffy the registry "0.15"); `tests-summary.md:30` re-derived (applied).
