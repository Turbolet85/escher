# Evolve Diagnosis — escher-0.2.0 · Epoch 1 — Foundation · 2026-10-17T08:51:13Z

## Mechanism health
- **Records:** 71 (47 step / 24 friction) over 3 chunks: `2026-10-16-handler-splits-and-readme`, `2026-10-16-file-splits` and `2026-10-16-fast-failing-mutants`, plus 3 `new-session/orientation` records with `chunk: null`.
- **Coverage vs expected** (phase 5 · implement 3 · wrap 5):
  - handler-splits-and-readme 5 · 3 · 5, no gap.
  - file-splits 5 · 3 · 5, no gap.
  - fast-failing-mutants 8 · 5 · 5, no gap. A phase revision after the halted fix-loop re-ran research · plan · validate and implement's code · fix-loop. That is two passes, not a gap.
  - new-session: 3 orientation records. How many session starts happened cannot be read from the ledger alone, so a missed orientation is **not measurable** here. The fast-failing wrap ran in two windows.
- **Unparseable:** 0 over the whole ledger (1,211 lines). **Malformed ts:** 0.
- **Untyped rate:** 8 of 24 (33 %).
  - phase/distill 1/1 · phase/research 3/4 · phase/validate 3/5 · wrap/reconcile 1/5.
  - Every other step: 0.
- **Problem-fact fill:** 10 of 47 step records (12 facts). **Id fill:** 71/71.
- **Retractions:** 0 retracted over the whole ledger, 1 unresolvable, both clause retractions outside this epoch.
  - The unresolvable one is `2026-10-07T04:21:25Z-a` (`unknown-id-or-scope`, `id: null`, "in step record 2026-10-07T04:21:04Z-a (a step record, so n…"). It predates this epoch and is reported for the founder's manual discount.
  - No retraction targets a retraction.
- **Signal line:** guard-refused 11 over 47 step records · exact-hit 3 over 3 curation records.
  - Of the 11 refusals, 6 were a `cd` into a skill references dir or a run dir, and the rest were a pipeline tool piped into a filter. Context only, never a proposal.
- **Calibration boundaries in range:** every boundary in `diagnosis-pass.md` (deviation scan · universal types · `contract.in-pass-correction` · `operator-pass` entry · implement-outcome's ok reading · `new-text-*` words · guard-refused/exact-hit · the ladder gloss) predates 2026-10-16. Every field is live in this range, with no era effects.
- **Threshold reading used:** F-2 / F-4 at n ≥ 3 in-epoch, or n ≥ 2 in-epoch with a halt or soft exit (typed), or n ≥ 2 in-epoch that also occurs in a prior epoch (themes, clusters). A theme at n = 1 in-epoch stays below threshold however often it recurred earlier, and is marked *recurring* in the appendix.

## Proposals (typed patterns)

### P1 — (any step)/`recall.corpus-recurrence` — 3 cases · weight 4
**Pattern:** In every chunk of the epoch (3 of 3), a host-linux.md rule loaded on every turn recurred anyway: twice the «Processes» rule (a `ps` census counting its own probing shell) and once the 2026-10-13 rule that pipeline tools are read whole. This continues an unbroken recurrence: 0.1.0 Epochs 1–6 read 5 · 7 · 7 · 4 · 7 · 17 records of this type (read-only lookback, `q-lookback-types.txt`).

**Evidence:** all 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-16-handler-splits-and-readme (wrap/gates) | the P7 report's health.py check capture was read through a grep for its check 1 row instead of read whole, against host-linux.md Session Additions 2026-10-13; the row read was correct | retries 0 | this wrap's P7 step 6 health read |
| 2026-10-16-file-splits (wrap/curation) | implement's process census by ps piped into grep -c counted its own probing shell twice, the failure host-linux's Processes rule already states; re-taken on the comm column it read 0 | retries 1 | escher-0.2.0/chunks/2026-10-16-file-splits/report.md |
| 2026-10-16-fast-failing-mutants (wrap/curation) | both implement runs' census grep over ps args matched its own probing shell and grep, read on the comm column — the «Processes» rule states the remedy correctly; carried to the handoff as recurrence-despite-learning | extra_reads 1 | escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/report.md |

An untyped sibling of the same class sits at handler-splits-and-readme phase/validate (`2026-10-16T14:26:59Z-b`: gate.py baselines read through `sed`). It is outside the count. The fact-side companion is file-splits reconcile (`2026-10-16T17:49:24Z-a`#1: cites.py stdout sent to `/dev/null`).

**Absorbed mid-epoch:** the bash-guard now refuses a pipeline tool piped into a filter at the moment it is typed (the guard-refused signals at fast-failing validate and reconcile). The epoch's remaining recurrences are the forms the guard cannot see: a saved capture read through a filter afterwards, a `/dev/null` redirect, and a `ps` census.

**Proposal:** a direction that moves the remaining rules from recall to mechanism:
- a census helper that the playbooks name for "is anything still running", which excludes its own process tree and reads by `comm`;
- pipeline tools that print a single verdict line plus a trail path, so a filtered read of a capture is never needed;
- a guard arm for `> /dev/null` on a pipeline tool.

Each of these replaces a rule the agent keeps re-learning with a form that cannot be taken wrongly.

## Cross-step chains (starting heuristics)
No (producer, artifact, consumer) shape repeats across ≥ 2 chunks under the strict join, so there is no cross-step pattern candidate. The hypotheses below are recorded for the founder's reading.

### X1 — phase/validate →plan→ wrap-session/gates — 1 chunk (file-splits)
- **Producer** `2026-10-16T16:55:35Z-a` (validate) read `ok`, with signals `control-caught-vacuity` and `control-caught-preexisting-hit`. Its own untyped friction says two must-fail controls read green because their `sed` substitution matched nothing.
- **Consumer** `2026-10-16T17:56:52Z-a` (gates) read the plan `thin`: entry 13's git-diff probe reads untracked new files as absent, so it is green only before the pre-CI commit. Its friction `2026-10-16T17:56:52Z-b` (`tooling.result-not-run-stable`) carried halted 1 · dialogue 1.
- **Hypothesis:** validate's controls are taken on an untouched tree, never in the state the entry is read in at the wrap (after the operator's commit).

### X2 — phase/research →research→ implement/fix-loop and phase/validate →plan→ implement/fix-loop — 1 chunk (fast-failing-mutants)
- **Producers:**
  - `2026-10-16T19:21:10Z-a` (research, `ok`, `unresolved-questions`; its two untyped frictions concern cargo-mutants output);
  - `2026-10-16T19:34:20Z-a` (validate, `ok`, `guard-refused:1`).
- **Consumer** `2026-10-16T20:11:30Z-a` (fix-loop, `halted-resolved`) read the research `thin` and the plan `wrong`.
  - Its friction `2026-10-16T20:11:30Z-b` (`contract.instrument-validity`, soft_exit 1 · dialogue 1) says the instrument was validated under `--baseline=skip` and never in the plan's default-baseline form.
  - The closing friction `2026-10-16T20:18:32Z-b` sits at the re-run research.
- **Hypothesis:** the same shape as X1. An instrument or control is validated in one form or state and run in another.

**X1 + X2 together:** the strict consumer step differs (gates vs fix-loop), but the producer pair (phase/validate →plan) and the hypothesis repeat across both chunks, and both ends halted. That is two chunks of "validated in a form other than the run form". **Direction, if the founder reads it as one shape:** a validate check that each gate entry's control is taken in the state and form the entry runs in (post-commit tree, default baseline), or that the plan states why that is not possible.

### X3 — (prior wrap) →handoff→ new-session/orientation and phase/take-up — 1 session
- Orientation `2026-10-16T14:06:35Z-a` read the handoff `wrong`. Its friction `2026-10-16T14:06:35Z-b` (`input.handoff-git-mismatch`) says the Branch and Position Next named 0.1.0.
- Take-up `2026-10-16T14:08:52Z-a` read it `wrong` again in the same session.
- The producer is outside this epoch: the last 0.1.0 wrap, followed by the epoch-6 audit and the 0.2.0 route commits, which rewrote no handoff line beyond Session End Status.
- **Hypothesis:** commits made between a wrap and the next session (an audit, a route commit) have no handoff-rewrite step.

### X4 — wrap-session/report →report→ wrap-session/reconcile — 1 chunk (fast-failing-mutants)
- Producer `2026-10-17T08:05:45Z-a` (`ok`, `new-text-generated`, `new-text-rows:9`, no friction).
- Consumer `2026-10-17T08:26:39Z-a` read it `thin`, with friction `-b` (`input.report-insufficient`): the bridge-six pattern missed test-plan.md:24's «six on the bridge».
- The producer carried no quality signal, so this is not a formally-ok-but-signalled chain. It is recorded only.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 2 facts (+2 typed correlates at one site, +1 prior-epoch fact)
**Facts:**
- 2026-10-16-handler-splits-and-readme · wrap/reconcile · process · workaround — "the pass-mode leaf set listed the test-plan and obs-plan groups as changed only because the citation sweep re-pointed their digits; no comparer was spawned for those two groups, on citation-contract's rule that a re-pointed master is not a changed source" (`2026-10-16T15:11:14Z-a`#0)
- 2026-10-16-fast-failing-mutants · wrap/reconcile · process · workaround — "services/escher-driver.md:60 held the stale 73 / mcp_tests.rs 11 counts, which test-plan §1 states and its group's only source (architecture) does not; no comparer could raise it, so the sweep's leaf rows were written by hand" (`2026-10-17T08:26:39Z-a`#0)
- Typed correlates, both at `.claude/docs/services/escher-driver.md:60`:
  - file-splits `contract.cascade-miss` (`2026-10-16T17:49:24Z-c`: the count was written by hand);
  - fast-failing-mutants `contract.structural-blind-spot` (`2026-10-17T08:26:39Z-c`: "a comparer told 'a statement no source states is not yours' cannot flag it").
- Prior epoch (lookback): 0.1.0 Epoch 5 · 2026-10-12-self-description · reconcile · workaround. The comparer prompts' `{source_paths}` named the plans' key files though `cascade.py leaves --all` printed the master alone.

**Level hypothesis:** the cause appears to live in the pipeline. cascade.py's leaf→source group model holds one provenance source per leaf and counts a group as changed on any master diff, a digit re-point included. The fixes so far live in the project: hand-written leaf rows, a hand-skipped comparer spawn, and a Tier 3 learning (fast-failing curation, exact-hit:1). The same leaf line needed a hand fix in two consecutive chunks.

**Proposal:** a direction in the cascade tool, not a project rule:
- a leaf's group may list every master (and key file) whose statements the leaf restates, not its provenance alone;
- a group whose only diff is citation re-points reads as unchanged by the tool itself, so the comparer-spawn decision does not rest on the orchestrator's reading of citation-contract.

### L2 — override — 2 facts (+1 halting correlate, +1 prior-epoch fact)
**Facts:**
- 2026-10-16-file-splits · wrap/reconcile · process · overridden — "at the P1 resume reads the operator-directed .gitignore line made cites.py added read 'the listing differs' (HALT); the operator dropped that part of the directive, the line reverted, nothing recorded" (`2026-10-16T17:49:24Z-a`#0). Its friction `2026-10-16T17:49:24Z-b` is `contract.jointly-contradictory-instructions` (halted 1 · dialogue 1): "the operator's resume directive … and the 2a-resume rule that the new-text listing re-reads 'unchanged' … cannot both hold".
- 2026-10-16-fast-failing-mutants · wrap/report · process · overridden — "the operator directed the residuals.md open line (route-resolve's P5 write) and the .gitignore incubator line to be made at P1 before cites.py added, so both land with this chunk's new-text listing (inputs#I4)" (`2026-10-17T08:05:45Z-a`#0)
- Prior epoch (lookback): 0.1.0 Epoch 5 · 2026-10-10-driver-cli · reconcile · overridden — "the resumed report gained two lines … although a 2a resume reuses the report as is".

**Level hypothesis:** the operator corrected the same rule in consecutive chunks: a wrap holds the chunk's tree from P1, and a 2a resume re-reads the P1 listing unchanged. The first time it halted, and the second time the operator routed around it by front-loading the edits at P1. The deferred `.gitignore` line rode one wrap's handoff before it could land. The rule appears to have no sanctioned slot for a tracked edit the operator directs during a wrap. The project has absorbed this as operator practice: edit before P1.

**Proposal:** a direction in the wrap skill:
- a named intake for operator-directed tracked edits at a wrap (made before `cites.py added`, recorded as an input);
- or an exclusion form in the 2a-resume listing check for edits recorded as directed.

Either way, the next such directive meets a rule written for it, not a halt.

*Chronic-degrade:* no hit. `tooling.result-not-run-stable` recurs across epochs (0.1.0 E1, E6, 0.2.0 E1), but its in-epoch case halted and the specifics differ. No `ok-degraded` outcome occurred.

*Deferred-forever:* no hit. No deferred fact and no `deferral-open` signal occurred. The file-splits route-resolve note's operator-deferred `.gitignore` line closed in fast-failing-mutants (L2's second fact).

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/distill, phase/research (cross-step) — 2 cases, recurring from 0.1.0 Epoch 6 → proposed type `tooling.path-anchor-drift`
**Cluster:**
- `2026-10-16T14:13:59Z-b` (handler-splits, distill) — "a compound command opening with a cd into the run dir (cd X 2>/dev/null; …) passed the Bash guard and moved the session cwd into the run dir; one extra call returned it to the project root"
- `2026-10-16T14:18:21Z-b` (handler-splits, research) — "a code-graph query's stdout was redirected to a relative path climbing four levels from the run dir, which landed outside the project root; one extra call moved it into the scratchpad"
- Previous epoch: `2026-10-13T21:41:41Z-d` (range-input-interaction, reconcile) — "a redirect target composed as $RD/../../../.andromeda/runs/x resolved inside the tree and wrote a stray file".
- Fact-side recurrence of the cwd half: `removed-cause` at 0.1.0 E5 mcp-surface distill and E6 stand-contrast-harness distill ("a cd into the run dir, which the guard passed"), and workarounds at 0.1.0 E1 ×3 and E2 ×1. This epoch's 6 `cd` guard refusals show the reach for `cd` persists where the guard does catch it.
- Neighbouring universal `tooling.host-shell` covers cwd persistence only partly and a `../` climb not at all.

**Draft criteria line:** `tooling.path-anchor-drift` — a command's working directory or a relative path resolved somewhere other than the author meant (a `cd` that passed the guard and persisted across calls, a `../` climb composed from a run dir), costing a recovery call or a stray write. Record the form (persisted cwd or relative climb) and whether the guard saw it. A candidate for the universal list, since its cases span phase and wrap steps.

## Below threshold — no action
Typed (per step, and universal by type alone):
- phase/validate `contract.mechanical-check` — n 2 · weight 4, fast-failing-mutants: mutation-run records into the tracked evidence/; TOML literal strings with an apostrophe read UNPARSED.
- `contract.instrument-validity` (not universal; per step) — fix-loop n 1 (soft_exit) and research n 1, fast-failing-mutants. One event recorded at its finding and its closing step (X2).
- `contract.jointly-contradictory-instructions` (universal) — n 1 · halted, file-splits reconcile (L2's correlate).
- `tooling.result-not-run-stable` — n 1 · halted, file-splits gates (X1). *Recurring* by type (0.1.0 E1, E6).
- `contract.cascade-miss` and `contract.structural-blind-spot` (universal) — n 1 each, same site `services/escher-driver.md:60` in two chunks (L1's correlates).
- `input.plan-step-ambiguous` — n 1, file-splits implement/code (moved items vs the parent's shrunken imports).
- `retry.synthesis-rework` — n 1, file-splits plan (a sha256 prefix no reading had produced).
- `input.handoff-git-mismatch` — n 1, orientation (X3).
- `ambiguity.tier-routing` — n 1, file-splits curation (no rule file's paths cover `src/`).
- `input.report-insufficient` — n 1, fast-failing reconcile (X4).

Untyped clusters (`q-untyped-clusters.json`):
- U2: a third-party tool's output read on an assumed shape, or its reason hidden on stderr. n 2, one chunk (fast-failing research: the outcomes.json float duration; `--list` without `-p` silent), no previous-epoch member. *Emerging*, watch next epoch.
- U3: the validate `sed` read of gate.py baselines. An existing type fits (`recall.corpus-recurrence`), so this is a type-application miss (P1's sibling).
- U4: a must-fail control green because its mutation matched nothing (file-splits validate, X1).
- U5: a scratch script imported by another ran its main at import (file-splits reconcile).
- U6: plan-template's "output removed first" vs the guard's no-recursive-rm (fast-failing validate). The existing universal `contract.skill-reference-drift` fits.

Note themes (`q-level.json`):
- T3: freshness-by-removal refused by the guard. n 1 in-epoch, *recurring* (0.1.0 E2, E4 ×2).
- T4: a two-window wrap's resume skipped Setup step 7's code-graph refresh. n 1, *recurring* (0.1.0 E4, E5 ×2).
- T5: rustc `unused_imports` on intra-doc-link-only items after a split. 2 facts of one record, product-logic, curated Tier 3.
- T6: tokei panic over a scratch dir. n 1.
- T2: cites.py stdout to `/dev/null`. n 1, P1's fact-side companion.

Overrides at n 1 in-epoch:
- light gate red on the probe's form → record-and-proceed (file-splits gates). *Recurring* (0.1.0 E2 project-readme).
- fix-loop red → phase revision (fast-failing).
