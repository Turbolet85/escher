# Curation — 2026-10-10-driver-cli wrap, Phase 3

Session-local. This wrap ran in two windows: the first authored the report and stopped on the operator's word, and its conversation is gone. So the candidates here are this window's conversation and the report's *Decisions & corrections*; a correction only the first window's conversation held is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "Before calling a standing check unaffected by a change to data it could pin, search the tests for the table's name and for the call builders" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A hand re-point of a citation can match a longer citation that starts the same" (confidence 0.8)
  Filters: 3 dup · 1 task-specific · 0 conflict · 0 deferred · 1 below threshold
  Load-bearing: "Before calling a standing check unaffected by a change to data it could pin…" → MCP surface
  No-other-home: "A hand re-point of a citation can match a longer citation that starts the same"
```

## Applied

- **Tier 2 · `.claude/rules/testing.md` · Session Additions** — the standing-check search. Signals: verified by a real gate failure (+0.4), a specific technical detail with context (+0.2), load-bearing for the next entry (+0.2) = 0.8.
  Proof: the report, *Spec claims disproved* item 7 and *Sweep hazards found this chunk* — the plan's gate note implied `stand_act_disabled` unaffected, and it read red on implement's first full gate pass because it pinned the append reading of `type`; `stand_act_spans`, which read every row of the verb table back, is the second (both in `scope-record.md` as companions). Next entry: "MCP surface" joins tools to the one verb table at its promotion; its line carries no such annotation, and no master was amended with the rule.
- **Tier 3 · `.claude/docs/session-learnings.md`** — the prefix twin. Signals: verified by a real failure (+0.4), a specific technical detail with context (+0.2), reached no other durable home this wrap (+0.2) = 0.8.
  Proof: this wrap's P2 citation read — the anchored Edit re-pointing `examples/seven_guis/tests/common/mod.rs:1-4` in `.andromeda/test-plan.md` was refused on two matches, the second the citation `…/common/mod.rs:1-40` the sweep tool had just written on line 10; the replace was re-issued with its closing parenthesis, and every replace-all re-point of the pass was then searched for a longer twin (0 found).

## Filtered

- duplicate → recurrence-despite-learning: "a record that quotes a word naming a temp-dir path is refused by `gate.py hygiene`" (the report's sweep hazards) matches `.claude/docs/session-learnings.md`, "2026-10-06 — Committed run-dir and evidence text must not spell a host temp path, even as prose", a defect record — and the defect recurred: the first draft of `evidence/operator-pass.md` quoted the operator's sentence with the temp-dir path in it, and the hygiene entry refused the record itself. Logged in the handoff's Deferred learnings; not a third entry.
- duplicate: "a scratch directory with a long path cannot hold a session" — the escher-driver service note's first gotcha and the Session lifecycle key's session-state already say a socket address has about a hundred bytes.
- duplicate (this wrap's own master text): "an `attach` or a `status` is an answered request and restarts a host's idle expiry" — amended into test-plan §2 and security-plan §API Security at P2 and re-derived into `.claude/rules/testing.md`'s body.
- task-specific: the operator's ruling on acceptance criterion 12 — one chunk's criterion.
- below threshold: "under `cargo test -- --nocapture` a test's first printed line shares its line with `test {name} ... `, so a line-anchored pattern misses it" — stated in the report with no measurement beside it, and the conversation that met it is gone: a specific technical detail (+0.2) alone.
- not a candidate (a master carries it): `Role` and `BoundingRect` are not re-exported by dioxus-native-dom — test-plan §1 now states why the JSON writer's node rows are tested over plain parts.
