# Curation — 2026-11-13-cognitive-load-corrective

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A control that deletes a loop's advance statement never ends, so the test binary hangs rather than reading red — neutralise the step to the loop's exit" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A master row's `verified` evidence cites lines no sweep re-points" (confidence 0.8)
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "a loop-advance control hangs; neutralise it to the loop's exit"
  No-other-home: "a row's verified evidence cites lines no sweep re-points"
  CLAUDE.md size: 163/200 · T1 4.3 KB, 0 over 600 B
```

## Applied
- testing.md `## Session Additions` (TX 692) — a loop-advance control hangs; neutralise it to the exit.
  Proof: the plan's control (b) as written («remove the `maybe_node_id = doc.nodes[node_id].parent;` walk-on») loops forever. The neutralised form `= None` reads the walk's stop red: control `b-walk-on` `holds` in 3.7 s (`.andromeda/runs/2026-11-13T20-45-07-implement/control-1.toml`, report Deviation 3). It changed the chunk's control design: +0.4 measured, +0.2 detail. No-other-home +0.2: test-plan's `dec-7jkoc3jb` set was rejected at P2 (fanout-results.md), and no route annotation carries it. Score 0.8.
- session-learnings.md (TX 693) — a row's `verified` evidence cites lines no sweep re-points.
  Proof: `cites-three` (`path\.rs:\d+` over the three files) went unfired over the masters (`cascade-2026-11-13-cognitive-load-corrective-pre-pass.json`), while `rec.py get dec-hssnx4z6` printed `verified` evidence «pointer.rs:869-872» · «pointer.rs:903-907», and `dec-2cfl26f6` printed «pointer.rs:869» — lines this chunk moved to 951-954 · 995-1000 · 953 (report, Expected amendments). +0.4 measured (it falsified the sweep's coverage), +0.2 detail. No-other-home +0.2: no master, rule or route carries it. Score 0.8. It is Tier 3 because it is context on the store's tooling, not a path-scoped directive.

## Decisions & corrections — one line each
- An engine reading met (unmatched click → `focused()` the root element) — not curated: amended into a master at P2. test-plan `contract/harness-inspect/focused` (TX 687) is its home, and the cascade carried it to `blitz-test-harness.md` and `verification-harness.md`.
- A tool refusal (`gate.py control`'s id form) — rejected, task-specific: it is an Andromeda tool's argument bound, telemetry of the tool, recorded as friction, not a project learning.
- A control's form (loop advance) — applied, Tier 2 testing.md (above).
- Guard refusals — not a learning: each was re-issued once in the form the guard passes, and the guard itself holds the rule.
- A sweep hazard (rows hold no citation; `verified` cells cite moved lines) — applied, Tier 3 (above).
- A census design point (`let … else` keeps the `.unwrap()` count equal) — rejected, task-specific: one plan's census entry. Score 0.2 (+0.2 detail, −0.2 task-specific, +0.2 detail of context): below the threshold.
- The operator's operator-pass word — not a learning: a direction for one run, consumed by it.
- The operator's wrap word (inputs#I2) — not a learning: a direction for this wrap, discharged at P2 (TX 686).
- The P4 fork (inputs#I1) — not a learning: amended into test-plan `dec-akszmc27`'s `except` (TX 687).
- citation-dispositions.md's one row (`.claude/docs/session-learnings.md:306 host.rs:350`, out-of-range) — no change: an illustrative prefix example inside a learnings entry, not a claim about `host.rs`.
