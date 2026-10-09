# Curation — 2026-10-09-audit-corrections-agent-surfaces

Source: the report's *Decisions & corrections* section (the operator's direction for this resumed wrap; the conversation that produced the chunk ended with a `/clear` before the resume, so a correction only that conversation held is not curated).

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "In a hand mutation control, save the file's bytes before the mutation, restore by writing them back and compare sha256 … run the restore check before taking the green reading" (confidence 0.8)
                                              + testing.md: "Count the result lines of a red `cargo test` run only under `--no-fail-fast`" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "In the jobs API a step's seconds and conclusion do not say whether a guarded command ran" (confidence 0.8)
  Filters: 1 dup · 2 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 below the confidence threshold · 1 recurrence-despite-learning (→ handoff)
  No-other-home: "a hand mutation control restores by bytes and sha256" · "a red `cargo test` run's result lines need `--no-fail-fast`" · "a step's seconds do not say whether a guarded command ran"
```

## Applied

**Tier 2 — `.claude/rules/testing.md` § Session Additions — the mutation-control restore.**
Scoring: verified by measurement +0.4 · specific technical detail with context +0.2 · reached no other durable home +0.2 = 0.8.
Proof: `escher-0.1.0/chunks/2026-10-09-audit-corrections-agent-surfaces/evidence/controls.md`, step 2, reading 3 — the first restore edit of the `textarea` arm did not apply, the restore check `git diff --quiet {base} -- packages` read exit 1 and the unit command read the mutant's red a second time; step 6's four mutations were restored by writing the saved bytes back with sha256 compared, "equal in all four". Report: Deviations ("A restore edit missed once") and Decisions ("A restore is proven, not assumed").
No other home: test-plan §4 records that this chunk's four controls were restored by sha256, as a fact of this chunk, not as the rule.

**Tier 2 — `.claude/rules/testing.md` § Session Additions — `--no-fail-fast` for a red run's result lines.**
Scoring: verified by measurement +0.4 · specific technical detail with context +0.2 · reached no other durable home +0.2 = 0.8.
Proof: the same `controls.md`, step 2, reading 2 — on the mutated tree the audit unit's command exits 101 and prints six of its eight result lines; with `--no-fail-fast` it prints all eight and shows no second failure. Report: Deviations ("One run beyond the plan on the mutated tree") and Decisions ("A red under cargo's default stops at the first failing target").

**Tier 3 — `.claude/docs/session-learnings.md` — a step's seconds do not say whether a guarded command ran.**
Scoring: verified by measurement +0.4 (it falsified a figure in the plan's gate note) · specific technical detail with context +0.2 · reached no other durable home +0.2 = 0.8.
Proof: the report's *Spec claims disproved by measurement* — nine steps call the install script and eight executed it in CI#37985678276; the `Test CI scripts` step took 0 s and printed nothing after its command header (`gh run view 37985678276 -R Turbolet85/escher --log`, read 2026-10-09T20:27:16Z), while apt's own output is present in the eight other jobs. The step-name facet: the report's sweep hazards ("in the jobs API a `run:` step's name is `Run {the command}`"), folded into this entry rather than written as a second one.
No other home: the masters state the measured reading (the guarded call not reached in that run), not how to read the jobs listing.

## Filtered

- **Duplicate (Filter 1):** "a process census by a word of the command line matches other sessions' launch arguments — select by `comm`" — `.claude/rules/host-linux.md`, Processes, already says to select by `ps -eo pid,comm,args` and act on the pid.
- **Task-specific (Filter 2):** "`textarea` and `apt-get` both hit this chunk's own frozen route line" (one line of one route, compacted at this wrap's flip) · "the new-text listing's row for the script's `while` loop ends one line early" (a pipeline tool's reading of one file; named in the handoff for the operator, not a project learning).
- **Below the confidence threshold (Filter 4):** "a bare number is not a count — `\b656\b` matches a `file:line` range; search the phrase" — specific technical detail +0.2 · repeated pattern +0.3 = 0.5.
- **Recurrence-despite-learning (Filter 1, a defect record matched):** "the cat-heredoc guard fired once in the implement session" against the Tier 3 entry "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too" (2026-10-06) and `.claude/rules/host-linux.md`. Logged in the handoff's Deferred learnings; no third entry written.

## Seen and not written (outside the directed source)

The Tier 3 entry "A chunk that moves cited source lines stales the masters' file:line citations" (2026-10-05) describes a hand re-point at wrap. This wrap's Phase 2 opened with the citation tool's sweep instead — escher's first, 40 citations re-pointed and proved by the diff map (`citation-dispositions.md`) — so that entry's procedure is now the tool's, with a hand owed only on the rows the tool lists. The entry is on the list the founder reviews as a separate step, and this wrap's curation was directed to the report's Decisions & corrections, so it was not edited here; the handoff names it for that review.
