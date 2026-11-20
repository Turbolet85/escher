# Curation — 2026-11-20-browser-s-surviving-mutants (wrap 2026-11-20T14-24-05-wrap)

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A cargo-mutants `--output` written into a chunk's evidence carries host paths that hygiene refuses"
  Filters: 2 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B
```

## Applied
- Tier 3 · `.claude/docs/session-learnings.md` (tx 2691) — «A cargo-mutants `--output` written into a chunk's evidence carries host paths that hygiene refuses». Confidence 1.0: the operator's direction +0.4 («Delete nothing: MOVE debug.log, log/ and outcomes.json of both runs to target/mutants-raw-2026-11-20/, keep caught.txt, missed.txt and mutants.json in evidence, then land»), verified by a real gate refusal +0.4, a technical detail with context +0.2. Tier 3: the learning spans the mutation run and the pipeline's hygiene read, and no rule file's `paths:` covers it (testing.md scopes `tests/**`).
  Proof: `evidence/operator-pass.md` §Raw output moved out of evidence — «Hygiene refused 7 files of the first run's output for host paths (6 in-root, 1 home: `outcomes.json:46`), and `rec.py land` wrote nothing».

## Decisions & corrections — each weighed
- implement: the Bash guard refused a `cd` into the skill's references and the call was re-issued as Read calls → not a learning: a hook already refuses the act (Filter 4's hook clause), and the cost was its one re-issue.
- After implement: the operator's cap of at most 2 mutants at a time (`-j 2`) → rejected, Filter 4 (0.1): the operator's direction for this pass alone (+0.4), one-off without "from now on" (-0.3).
- After implement: the sweep hazard — cargo-mutants' `--re` is an unanchored regex over the whole mutant name, location included; anchor on the verb, `--list` first → rejected, Filter 1 duplicate. This wrap's P2 landed it in test-plan (`decision/mutation-filter-anchored-on-verb`, tx 2686), and the cascade wrote it into the generated bodies of `.claude/rules/testing.md:48`, `CLAUDE.md:110` and `.claude/docs/commands.md:30` (a dedup-reject-only match).
- After implement: the operator's ruling «The red is the entry's filter, not the code» → rejected, Filter 2 task-specific: a ruling on this chunk's one gate entry; its general half is the filter convention above.
- After implement: delete nothing, move raw output that hygiene refused → applied (Tier 3, above).
- After implement: CI's 2634 s wall passed entry 8's `--wait 2400` → rejected, Filter 1 duplicate of `.claude/docs/session-learnings.md` «A plan's `ci.py conclusion --wait {s}` entry is a bound, not a verdict»; its figures are task-specific too.
- After implement: the wrap directive (inputs#I5) → not a learning: a ruling for this wrap's P5, carried there.
- This wrap: the Bash guard refused a `cat` heredoc append to a run-dir file, re-issued once as an anchored Edit → not a learning: a hook already refuses the act.
- This wrap: the citation sweep's out-of-range row `.claude/docs/session-learnings.md:391` (`host.rs:350`) → no change. It is an illustrative prefix example inside a learning (`host.rs:35` heading `host.rs:350`), not a claim about the tree, so it is neither a correction nor a learning.
