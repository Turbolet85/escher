# Cascade dispositions — 2026-11-08-what-can-i-do-here (wrap 2026-11-08T15-29-03)

## The search
- One call, `cascade.py sweep` (cascade v1.29), baseline `7e2e8be6` (the parent of the oldest pre-CI commit
  `f9a91327`), over the thirteen apply writers' `@pattern` records (`apply-{a11y-plan, architecture.p1-p3,
  design-system, layout-templates, obs-plan, security-plan.p1-p3, test-plan.p1-p3}.md`, prefixes a11y · arch1-3 · lt ·
  obs · sec1-2 · tp1-3) and main's own two for the texts it wrote (`cascade-patterns.toml`: `m-common` —
  `mod common;` by a count; `m-snaphelp` — the old snapshot help wording `the screen as one text[:,]`).
- Its listing: `sweep-pass-1.1.txt` (217 lines), read whole. Its totals, copied: `total (74 patterns) · 74 rows over 23
  files` · `per class · new 4/3 · standing 17/11 · leaf 53/10 · curation 0/0 · base 0/0`. Every zero-row pattern's
  control fired (the listing's `0 rows (control fired)` lines). Patterns that span the claim's own verbs and phrasings
  (the verb counts in words and digits, «instance verbs», «tools», «field kinds», the reader counts, the needle
  helpers, the listener mark) are the writers'; the forms the tool compiled are printed under each pattern.
- Not looked for: wording of facts this pass did not amend; the masters' citations' digits (the citation sweep's).

## Master rows — new (4)
- `existing-scopes.md:106` m-common · fct-4zmppehn · amended — main's edit (edits-main.md E2), «by 23 stand checks», `stand_actions` the twenty-third
- `existing-scopes.md:144` m-common · fct-o6vmxcuj · amended — apply-architecture.p3 E9, «by 23 checks»
- `test-strategy.md:58` m-common · fct-7m7lxsmy · amended — apply-test-plan.p1 E8, «by 23 `stand_*.rs` checks»
- `standard-contracts.md:204` m-snaphelp · fct-t7zq5feb · amended — main's edit (edits-main.md E1), the help quoted as `schema.rs` holds it; the citation's digits left to the next citation sweep (`rec put` refused them, `digit moved`)

## Master rows — standing (17)
- `test-plan.md:187` arch1-01 ×2 · fct-63656bwh · amended — edits-main-2.md E1, «the fourteen verbs» twice
- `registries/contracts/test-plan/session-lifecycle.md:9` arch1-01 @c119 · fct-76fzwvj7 · amended — edits-main-2.md E3, «fourteen verbs — the ten instance-level ones»
- `registries/contracts/test-plan/session-lifecycle.md:9` tp1-05 @c140 · fct-76fzwvj7 · amended — the same edit
- `registries/contracts/test-plan/session-lifecycle.md:5` arch2-08 edited @c4816 · fct-3ylhu62k · amended — edits-main-2.md E4, «of the ten instance verbs, a `lint`, a `graph` and an `actions` among them»
- `test-plan.md:188` arch2-15 · fct-szqqqnrt · amended — edits-main-2.md E2, «exactly the fourteen `mcp__escher__{verb}` tools»
- `obs-plan.md:76` arch3-04 @c1239 · fct-icom2cq7 · amended — edits-main-2.md E5, «thirty-four checks», `stand_actions` in the list
- `obs-plan.md:76` tp1-11 @c1239 · fct-icom2cq7 · amended — the same edit
- `test-plan.md:153` arch3-05 · fct-osorhgow · amended — edits-main-2.md E6, «32 checks … 24 `stand_*` files», `stand_actions` in the list
- `security-plan-detail/logging-monitoring.md:14` tp1-10 edited · fct-4hqe6pzr · amended — edits-main-2.md E7, «38 calls per mode» (`stand_act_spans.rs:767`, `run.calls == 38`, the count one layout mode's run holds; the report's Counts [span-calls])
- `obs-plan-detail/pii-scrubbing-compliance.md:9` tp1-10 edited · fct-hlttx6z4 · amended — edits-main-2.md E8, «38 calls per mode», the same basis
- `architecture.md:293` arch2-02 · fct-7lmrueug · no change — «fifty-seven commits», a merge-base commit count sharing the token
- `architecture.md:293` tp1-10 · fct-7lmrueug · no change — «thirty-six of the fifty-seven commits», the same merge-base count
- `architecture-detail/project-intent.md:9` tp1-10 · fct-4ncs7geh · no change — the merge-base commit count
- `security-plan-detail/dependency-security.md:31` tp1-10 · fct-7aitszps · no change — the merge-base commit count
- `test-plan-detail/quality-gates-coverage-targets.md:16` tp1-10 · fct-fx4qekph · no change — the merge-base commit count
- `test-plan-detail/test-scope-summary.md:14` m-common edited @c2344 · fct-k5y3m6cu · no change — «by all eight» is seven_guis' `tests/common/mod.rs`, whose reader count this chunk did not move
- `test-plan-detail/test-strategy.md:64` m-common · fct-7m7lxsmy · no change — «by all eight», seven_guis' module, unmoved

## Leaf rows (53 over 10 files) — each leaf joins step 3's set
CLAUDE.md · .claude/rules/security.md · .claude/rules/verification-harness.md · .claude/rules/testing.md ·
.claude/docs/security-summary.md · .claude/docs/services/escher-driver.md · .claude/docs/services/dioxus-native-dom.md ·
.claude/docs/tests-summary.md · .claude/docs/commands.md · .claude/docs/workflow.md — each re-derived at step 3, never
edited here (the rows the listing names at lines 146-211).

## Leaves
- The set: `cascade.py leaves` (pass mode — the chunk does not close its epoch: Epoch 3 holds one markerless entry),
  `--add` the sweep's ten leaf files; 15 groups · 25 leaves; four sources printed `re-pointed` (citation digits alone, no
  changed source); `unmapped .claude/rules/host-linux.md (host)` — the host leaf, no source states it, no comparer's
  and no step-3 edit.
- The batch: 15 comparers (`--batch leaf`), 15/15 pass; each file read through `leaves --apply --dry-run`, its rows
  naming every block; each finding's `source:` a fact quote, receipts ok.
- Per group, mode pass — findings · applied · by hand · declined:
  - a11y-plan-1 (a11y-summary) 1 · 1 · 0 · 0 — a11y-plan-2 (rules/a11y) 1 · 1 · 0 · 0
  - architecture-1 (conventions, gotchas, blitz-dom, blitz-paint, blitz-shell, blitz-traits) 0 · 0 · 0 · 0
  - architecture-2 (dioxus-native-dom, stack, workflow) 4 · 4 · 0 · 0
  - architecture+layout-templates (seven_guis) 4 · 4 · 0 · 0 — architecture+obs-plan (escher-telemetry) 2 · 2 · 0 · 0
  - architecture+security-plan+test-plan-1 (escher-driver) 32 · 32 · 0 · 0
  - architecture+test-plan (commands, blitz-test-harness) 6 · 6 · 0 · 0
  - design-system+layout-templates (design-summary) 3 · 3 · 0 · 0
  - masters-1 (CLAUDE.md) 17 · 17 · 0 · 0
  - obs-plan (obs-summary, rules/observability) 11 · 11 · 0 · 0
  - security-plan-1 (security-summary) 12 · 12 · 0 · 0 — security-plan-2 (rules/security) 11 · 11 · 0 · 0
  - test-plan-1 (tests-summary) 21 · 21 · 0 · 0 — test-plan-2 (rules/testing, rules/verification-harness) 7 · 7 · 0 · 0
- Total, copied from the tool's line: `total · blocks 132 · applied 132`. Every omission applied names the `actions`
  verb, its answer or the one actionable rule where the leaf's statement would otherwise read thirteen verbs, nine
  instance verbs or the mark-based listener leg — a reader acting on it would act against the source.

## Curation and bases
- curation 0 · base 0 (the listing's per-class line). The citation sweep's one `curation home — P3` row
  (`.claude/docs/session-learnings.md:301`, an illustrative `host.rs:350` prefix) goes to P3.
