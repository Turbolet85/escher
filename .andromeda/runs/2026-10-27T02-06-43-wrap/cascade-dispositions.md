# Cascade dispositions — 2026-10-26-codebase-health · wrap 2026-10-27T02-06-43

## Step 2 — the sweep
The search: `cascade.py sweep` over the apply writers' `## Patterns` — `apply-architecture.md` (arch-01: the retired
«read from the script's own text by `stand_requirement_sweep`'s leg check»; arch-02: the retired «by the named `--test`
list of `ci-leg.sh`'s own text»), `apply-test-plan.md` (tp-01: the per-OS tally record's last reading; tp-02: escher-driver's
earlier coverage reading `4918/5019`), `apply-a11y-plan.md` (a11y-01: the retired «membership held by
`stand_requirement_sweep`'s leg check»); `apply-obs-plan.md` holds `(none)` — its one edit adds a record and retires no
wording. Main authored no master text of its own, so no `cascade-patterns.toml`. Every control fired on the pre-pass
masters (baseline `19f9ff91`, the parent of the oldest pre-CI commit `3c04042e`), over the seven masters' bodies and
category sets, `.andromeda/registries/**`, the curation homes, the judgment bases and the leaves.

Listing (copied from the tool): `total (5 patterns) · 2 rows over 2 files` · `per class · new 0/0 · standing 0/0 · leaf
2/2 · curation 0/0 · base 0/0`.

Rows:
- `.claude/docs/services/seven_guis.md:38` arch-02 leaf @c7342 — re-derived: joins step 3's set (the retired
  «named `--test` list» wording of the 0.2.0 sweep's reach).
- `.claude/rules/testing.md:48` tp-02 leaf — re-derived: joins step 3's set (escher-driver's earlier coverage reading).

Zero-row patterns arch-01, tp-01, a11y-01: each a statement about its pattern alone, its control fired. No master,
registry, curation-home or judgment-base row: no other master cites the amended passages in their old words.

Binds (step 2): test-plan §3 ↔ obs-plan §3 and the log-schema binds — untouched; no harness command, log field or
violation-schema field moved in this pass.

## Leaves
Set: `cascade.py leaves --add .claude/docs/services/seven_guis.md --add .claude/rules/testing.md` — `total · pass · 10
group(s) · 22 leaves · 483334 B · unmapped 1`; `re-pointed .andromeda/security-plan.md` (citation digits alone, no
changed source); `unmapped .claude/rules/host-linux.md (host)` — the host's leaf, no source states it, no comparer's.
Mode: pass for every group (the chunk does not close Epoch 1 — «README and records» stays markerless). Comparers 10 of
10 pass. Write: `leaves --apply --only …` — `total · blocks 12 · applied 5 · held 7`.

- a11y-plan · pass · 0 findings.
- architecture-1 · pass · 0 findings.
- architecture-2 · pass · 2 findings · applied 1 · by hand 0 · declined 1 — applied `seven_guis.md:38#1` (stale, the
  sweep's reach); declined `stack.md:39#1` (omission: the dev-host complexity reader — the detail stays in
  architecture's stack set, no reader of the leaf acts against it without it).
- architecture+obs-plan · pass · 0 findings.
- architecture+test-plan-1 · pass · 1 finding · applied 0 · declined 1 — `commands.md:30#1` (omission: the complexity
  reading beside the mutation tier — detail in test-plan §10 `gate-complexity`; the leaf gives no command for it).
- architecture+test-plan-2 · pass · 2 findings · applied 1 · declined 1 — applied `escher-driver.md:61#1` (stale, the
  sweep's reach); declined `escher-driver.md:51#1` (omission: the listing test's helpers — detail in test-plan §4).
- architecture+test-plan+obs-plan+a11y-plan · pass · 1 finding · applied 0 · declined 1 — `CLAUDE.md:98#1` (omission:
  its text reads the complexity reading as «first read green on the unmutated tree», a clause of the mutation tier; the
  detail stays in test-plan §10).
- obs-plan · pass · 1 finding · applied 0 · declined 1 — `obs-summary.md:49#1` (omission: the factoring's silent
  helpers — the record stands in log-coverage; no reader acts against it).
- test-plan-1 · pass · 3 findings · applied 2 · declined 1 — applied `tests-summary.md:42#1` (stale, the coverage
  reading) and `tests-summary.md:21#1` (omission applied: without it a reader credits stand-repeat membership to the
  sweep test alone, against test-plan §1); declined `tests-summary.md:46#1` (omission: the dev-host ceiling bounds no
  function, so no reader acts against it).
- test-plan-2 · pass · 2 findings · applied 1 · declined 1 — applied `testing.md:48#1` (stale, the coverage reading);
  declined `testing.md:48#2` (omission: how the reading sums — detail in test-plan §10).
