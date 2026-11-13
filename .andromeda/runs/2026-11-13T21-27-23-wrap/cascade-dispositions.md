# Cascade dispositions — 2026-11-13-cognitive-load-corrective

## Sweep (step 2)
The search: `cascade.py sweep --patterns-file .andromeda/runs/2026-11-13T21-27-23-wrap/cascade-patterns.toml`, main's three patterns, one call, baseline 5a4d029f. No `cascade-patterns-{doc}-{k}.toml` exists: both landing dry runs (security-plan, test-plan) said `patterns: none: 1 set(s) and retire(s) changed prose alone — no file`, so the tool's own patterns are none and step 3's leaves read each changed row's delta.
- `main-unres-why` — the retired mechanism of `dec-ihosj2ae`'s why («the non-panicking resolver instead of the one that panics», every phrasing: the one that panics · non-panicking resolver · resolver that/which panics · panicking resolver): 0 rows, its control fired (security-plan, the pre-pass `dec-ihosj2ae`) — no master, key file, leaf, curation home or base still states it.
- `main-ceil-left` — `dec-akszmc27`'s qualified stance («left as they are», «engine functions over it»): 1 row.
- `main-focus-none` — the focus fallback the new contract row states («clears focus», a focused reading of none): 6 rows.
Counts from the listing's `total` line: 7 rows over 5 files; per class new 1 · standing 4 · leaf 2 · curation 0 · base 0.

Rows:
- `.andromeda/test-plan.md:532` `dec-akszmc27` (main-ceil-left, edited ×2) — amended: the row's own `except` cell (TX 687) names the exception to «engine functions over it are left as they are»; the rule cell stays true for every other engine function, with no intra-line duplicate of a retired mechanism.
- `.andromeda/architecture.md:157` `dec-svdas4ye` — no change: «a click on any other element still clears focus» is the click's behaviour, unchanged by the chunk; the root-element reading after the clear is the focus reader's, stated once in test-plan `contract/harness-inspect/focused`.
- `.andromeda/architecture.md:872` `ctr-zgwgfe4i` — no change: inert clears focus with no blur. That is a different mechanism, and it is true.
- `.andromeda/test-plan.md:240` `ctr-523awgmf` — new: the row this wrap added (TX 687), `contract/harness-inspect/focused`.
- `.andromeda/a11y-plan.md:105` `dec-3p3ll4ik` — no change: its proposed set was rejected at Validate (check 2). The rule stays true as written.
- `.claude/rules/a11y.md:33` — leaf: «still clears focus» is true. It goes to step 3's set.
- `.claude/docs/a11y-summary.md:23` — leaf: true as above. It goes to step 3's set.

## Leaves
The set: `cascade.py leaves --add .claude/rules/security.md --add .claude/docs/security-summary.md --add .claude/docs/obs-summary.md` (the operator's two contradicting leaves, inputs#I2, and the report's silent-checks leaf), pass mode, baseline 5a4d029f: `total · pass · 10 group(s) · 12 leaves · 504029 B · unmapped 1` — `.claude/rules/host-linux.md (host)`, the host's own leaf, no comparer's. The tool's total: `blocks 7`; applied 3, by hand 0, declined 4.
- added-1 (a11y-summary.md) · pass · 1 finding · declined: `:13` omission (the focused fallback) — the detail stays in test-plan `contract/harness-inspect/focused`, and the harness leaves below carry it; an a11y reader acts on no assertion here.
- added-2 (obs-summary.md) · pass · 0 findings — the silent-checks list stays true (the report: obs-plan holds no row naming `checkable_change_event`).
- added-3 (rules/a11y.md) · pass · 1 finding · declined: `:33` omission — the line states the engine's click behaviour, which is true and unchanged; the harness reading belongs to the harness leaves.
- security-plan-1 (security-summary.md) · pass · 0 findings — its `:13` clause (`resolve_url` answers none) no longer contradicts the row, whose false why is gone (TX 686).
- security-plan-2 (rules/security.md) · pass · 1 finding · declined: `:16` `retired` — the clause «read through `doc.url.resolve_relative`, `resolve_url` itself answering none for an unresolvable URL since the 2026-11-13 upstream take» is true (the report: `document.rs:1649-1651`) and the corrected row no longer contradicts it. Cutting a true mechanism from the always-loaded rule would lose the reason the guard holds.
- security-plan+test-plan-1 (services/escher-driver.md) · pass · 0 findings.
- security-plan+test-plan-2 (CLAUDE.md) · pass · 0 findings.
- test-plan-1 (commands.md · services/blitz-test-harness.md) · pass · 1 finding · applied: `blitz-test-harness.md:26` omission. A test writer reading the harness's crate notes would otherwise assert no focus after a clearing click.
- test-plan-2 (tests-summary.md) · pass · 1 finding · declined: `:13` omission — the summary's helper list names `focused`, the full promise is one hop away in its master, and the two harness leaves carry it.
- test-plan-3 (rules/testing.md · rules/verification-harness.md) · pass · 2 findings · applied both: `testing.md:48` omission (the ceiling's one exception: a reader would otherwise grow `handle_click` back past the ceiling unseen) · `verification-harness.md:37` omission (the path-scoped harness rule a check writer loads).
