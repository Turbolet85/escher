# Fan-out results — 2026-10-06-stand-test-contract

Seven Explore doc-agents, one parallel batch; detector counts per prompt arch 2 · security 3 · design 1 · layout 1 ·
tests 3 · obs 3 · a11y 2 = 15 = the drift-base's `doc:` names. Entity probe: no `&lt;` / `&gt;` / `&amp;` in any
return (entities=0). Raw twins: the three `proposals: []` returns that stripping changed (`.raw-fanout-{doc}.md`;
the a11y twin's one absolute basis path written repo-relative).

## Verdicts
- **architecture** — 2 proposals. Stripped: trailing "sweep notes" (no dependents: neither amendment retires a
  claim; :139 "the one fork-CI artifact outside `target/ci-logs/`", :144 ports none, :147, :151 env vars all stay true;
  D-arch-decisions no drift — bash/python3/cargo/git already in §Stack).
- **security-plan** — 0 proposals (`[]`, commentary stripped → twin). Noted: §Input Validation has no row for the
  agent-run argv boundary (invariant holds).
- **design-system** — 0 proposals (`[]`, commentary stripped → twin).
- **layout-templates** — 1 proposal. Stripped: "no duplicate-occurrence; :65 marker stays true".
- **test-plan** — 3 proposals.
- **obs-plan** — 4 proposals. Stripped: D-obs-instrumentation / D-obs-pii no drift; keyed contract untouched.
- **a11y-plan** — 0 proposals (`[]`, commentary stripped → twin).

## Proposals and dispositions

### architecture
1. D-arch-resources · warning · §Standard Contracts → CI contracts — register the `scripts/agent-run.sh` five-verb
   contract (exit grammar 0/1/2/3, usage before not-booted, selections, JSON-line events, ps1 pass-through,
   `test_agent_run.py` 14 cases in the `ci-scripts` leg). basis architecture.md:139.
   → **apply** (check 1: playbook "Accurate this-chunk addition" — every named thing is in the report's Changes; also
   the plan's P5-approved Expected amendment names this site and change).
2. D-arch-resources · warning · §Occupied Resources → Filesystem — add `target/agent-run/{status.json,events.jsonl,
   run.log}`, boot-recreated, cleanup-removed, `run.log` unscrubbed like `target/ci-logs/`, gitignored, not a CI upload.
   basis architecture.md:147. → **apply** (check 1: Accurate this-chunk addition + Expected amendment).

### layout-templates
3. D-layout-surface · warning · §Surface: cli → Primary screens — add a `scripts/agent-run.sh` bullet (usage, exit
   grammar, JSON-line output), pointing to test-plan §3 for the full contract.
   → **apply** (check 1: "Accurate this-chunk addition" — the section lists the repository's CLI entry points one per
   item (`paint_bench`, `bump`), and this is the chunk's new entry point; "Registry over-reach" fails its precondition —
   this section is per-entry-point, not category-grain. Applied as a usage/output pointer only, the contract kept in
   test-plan §3. The plan's Provenance line "Design and layouts carry no domain coverage" was a phase coverage reading,
   not a direction against this entry.)

### test-plan
4. D-tests-obs-harness · warning · §3 Test Harness Contract — add the measured agent-run contract block (verbs, exit
   grammar, three operator-ruled points, state area, event schema cross-referenced to obs-plan §3). basis test-plan.md:100.
   → **apply** (check 1: Accurate this-chunk addition; Expected amendment test-plan §3 names it; the three points are
   the operator's recorded ruling this session).
5. D-tests-obs-harness · warning · §3 — narrow the NOT YET MEASURED marker to the test-data bootstrap. dependent-of 4.
   → **apply** (with 4; Expected amendment names "the marker narrows to the test-data bootstrap").
6. D-tests-coverage · warning · §4 Unit Test Strategy — `test_agent_run.py` (`AgentRunTest`, 14 cases) joins "What unit
   tests cover"; CI-scripts leg count 23 → 37. basis test-plan.md:114.
   → **apply** (check 1: Accurate this-chunk addition; Expected amendment test-plan §4).

### obs-plan
7. D-obs-stack · warning · §3 Observability Harness Contract — the agent-run harness JSON-line event schema, encoder,
   `target/agent-run/` location, harness metadata not an escher sink; :83 marker drops "a JSON log schema, log file
   location". basis obs-plan.md:83. → **apply** (check 1: Accurate this-chunk addition; Expected amendment obs-plan §3).
8. D-obs-stack · warning · §6 Log Coverage — the harness JSON-line sink beside escher's text sink; :142 marker scoped to
   escher's own sink. dependent-of 7. basis obs-plan.md:142. → **apply** (Expected amendment obs-plan §6).
9. D-obs-stack · warning · §9 CI Integration artifact table — a row for the local `target/agent-run/` state area, never
   uploaded. dependent-of 7. basis obs-plan.md:302-303. → **apply** (Accurate this-chunk addition; the table already
   carries the sibling `target/ci-logs` row).
10. D-obs-stack · warning · §8 PII Scrubbing — reach: the harness log sits outside escher's scrub, carries no
   content-named field and no captured output; `run.log` unscrubbed by design. dependent-of 7. basis obs-plan.md:277.
   → **apply** (Accurate this-chunk addition; the scrub-reach list enumerates the sinks outside the scrub).

### Orchestrator raises (check 5 — Expected amendments reconciliation)
11. security-plan §Logging & Monitoring — the harness log: no content-named field, no captured output; `run.log`
   unscrubbed under `target/`. No detector proposed it. → **apply** (routine — the report's Schema / config + Coverage
   bullets substantiate it; the plan's Expected amendment names the change).
12. security-plan §Input Validation — a `CLI arguments (agent-run.sh)` row: verb allowlist, arg count, selection
   `^[a-z0-9_]+$` + file-exists, else exit 2. Raised from the security detector's own note (no row covers the new argv
   boundary). → **apply** (routine — Accurate this-chunk addition; it records an existing validation, it widens no
   boundary: the selection admits only test-file stems that already exist).

## Validate checks
- check 1 (playbook): 12 apply · 0 reject · 0 escalate. No two rules collide (Registry over-reach fails its
  category-grain precondition for #3 and is named there).
- check 2 (cross-contradiction): #4 (test-plan §3) ↔ #7 (obs-plan §3) bind — applied with one event schema, written
  identically; #1 ↔ #4 the same contract — arch carries the registration and points to test-plan §3. No opposing pair.
- check 3 (intent-consistency): the report's three deviations are the operator's recorded ruling this session
  ("your three choices stand … the wrap writes them into test-plan §3 as built") — the justified branch; scope record:
  none (`gate.py scope` clean, 0 lines).
- check 4 (absence needs evidence): #3 / #12 / #10 add, they retire no claim; the markers narrowed in #5 / #7 / #8 rest on
  the report's measured contract. Sweeps run at cascade step 2.
- check 5 (expected amendments): test-plan §3 → #4/#5 · test-plan §4 → #6 · obs-plan §3/§6 → #7/#8 · arch Filesystem +
  Standard Contracts → #1/#2 · security-plan §Logging & Monitoring → #11 (raised). Leaf `.claude/rules/verification-harness.md`
  → cascade step 3. Every entry covered.
- check 6 (disproved claims): the report lists none — nothing to dispose.

Escalations: 0.
