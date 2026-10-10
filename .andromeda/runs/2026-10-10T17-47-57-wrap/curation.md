# Curation — the wrap of 2026-10-10-snapshot-states

Scope: this session's conversation and the report's Decisions & corrections. The window that held implement's
conversation and the report's authoring is gone — a correction only that conversation held is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "At a wrap's Validate a boundary widening is never resolved without a halt, even when the founder's answer is already recorded or relayed — one card citing his recorded word and listing the proposals it covers, any rule riding the ruling on the same card in full, one confirm before any apply" (confidence 0.7)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A grep for a struct literal cannot tell a spread from a full literal" (confidence 0.8)
                                              + "A green test run prints no figure the test only formats into its failure message" (confidence 0.8)
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred
  No-other-home: "A grep for a struct literal cannot tell a spread from a full literal" · "A green test run prints no figure the test only formats into its failure message"
  CLAUDE.md size: 144/200 · T1 2.0 KB, 0 over 600 B
```

## Applied

- Tier 1 — the halt at Validate. Signals: an explicit operator direction (+0.4), "do NOT" language (+0.3) = 0.7.
  Proof: the wrap's invocation arguments, kept verbatim as `relay-1.md` beside this file (`inputs#I7`) — "At
  Validate do NOT resolve a widening with no halt: stop on a card that cites his recorded word and lists the
  proposals it covers, and wait for one confirm"; the card and its answer are `fanout-results.md` §Escalate and
  `relay-2.md`.
- Tier 3 — the struct-literal sweep. Signals: verified by measurement, a plan claim falsified (+0.4), a specific
  technical detail (+0.2) = exactly 0.6; no other durable home this wrap (+0.2) = 0.8.
  Proof: the report's Spec claims disproved, item 1 — the plan's "the eleven literals … compile unchanged" met a full
  literal that failed to compile on the new fields (`E0063: missing fields`); the report's Sweep hazards names the
  grep.
- Tier 3 — the figure a green run does not print. Signals: verified by measurement, a plan claim falsified (+0.4), a
  specific technical detail (+0.2) = exactly 0.6; no other durable home this wrap (+0.2) = 0.8.
  Proof: the report's Spec claims disproved, item 2 — the two `host_log` entries' logs hold only the test's `ok` line
  and no green entry prints a text size; the sizes came from a restored mutation, the by-level figures from a kept
  instrument (`evidence/readings.md`).

## Filtered

- duplicate — the Bash guard refusing a heredoc with a file target, the scratchpad included: `.claude/rules/host-linux.md`
  already states the refusal; the guard held the rule at its one re-issue.
- duplicate — the Bash guard refusing a `cd` out of the project root: the same file's Paths section.
- task-specific — research's per-file `#[test]` count of 13 against a measured 14 (a one-off count).
- task-specific — the gate tool filing an operator firing made with no run dir under a `run-{stamp}` directory (the
  pipeline tool's own behaviour, not a project rule).
- task-specific, below the threshold — a count located by its digits also hits line numbers inside citations
  (0.2: one technical detail, no second signal); the alternative-separator spelling inside a table row (0.2).

Not candidates: the founder's closed-field rule and the three state readings — amended into the masters and the
playbook by this wrap's Phase 2.
