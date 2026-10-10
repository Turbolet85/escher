# Curation — 2026-10-10-upstream-sync-agent-surfaces wrap, Phase 3

**Source.** This wrap resumed in a fresh window: the conversation that ran the chunk is gone, so the candidates are the report's *Decisions & corrections* section and what this window itself held — on the operator's word (the arguments of this wrap, 2026-10-10): "Curation works from the report Decisions and corrections; the deferred cargo-test streams line in your handoff lands in this wrap." A correction only the earlier conversation held is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A script that attributes `cargo test` results to a test target runs cargo with stderr merged into stdout and parses the one stream …"
  Tier 3 (.claude/docs/session-learnings.md): extended in place (below)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below the confidence threshold · 1 recurrence-despite-learning (→ handoff)
  Load-bearing: "the Windows job prints the path with a backslash … count cargo lines in a CI log only with a pattern that has first matched a known positive from that log" → Driver CLI
  Extended: T3/session-learnings.md: "A fork CI run with one job still open is not green, and how to read and re-run a hung job" + "the Windows job prints the path with a backslash; prove the pattern on a known positive from the same log"
```

## Applied

### Tier 2 · `.claude/rules/testing.md` `## Session Additions` — the cargo-test streams line
- The entry: "A script that attributes `cargo test` results to a test target runs cargo with stderr merged into stdout and parses the one stream — `Running tests/{name}.rs` is stderr and the `test … ok|FAILED` lines stdout, so two captured streams attribute nothing; print one mutation's per-target table before the full batch."
- Disposition: approved at the last wrap and held back there by the cap of three; its text was carried in the handoff's Deferred learnings, and it lands here on the operator's word above. Filter 1: no entry of the three homes states it (the nearest, the `--no-fail-fast` line beside it, is another fact about the same output). One of this wrap's three slots.
- Proof: the handoff of 2026-10-10T00:34:22Z, "Deferred learnings", holds the text verbatim; its measurement is the 2026-10-09-audit-corrections-agent-surfaces chunk's, whose curation approved it (`.andromeda/runs/2026-10-09T21-37-22-wrap/curation.md`).

### Tier 3 · `.claude/docs/session-learnings.md` — extension of the 2026-10-07 entry on reading a fork CI run
- The facet, as written: "Extended 2026-10-10: the Windows job prints the path with a backslash, `tests\{name}.rs`, so a `/`-only pattern misses that job even with the colour codes handled — count cargo lines in a CI log only with a pattern that has first matched a known positive from that same log, such as the bare target name."
- Filter 1: the candidate — the report's first three sweep hazards — matches that entry's third paragraph, which already records the literal colour codes between `Running` and the path. The matched entry is a defect record, so the hit is a recurrence (below), and the candidate's remainder — the Windows separator and the known-positive rule — is a facet the entry lacks.
- Filter 4: verified by measurement, +0.4 (it falsified the plan's last gate entry and the acceptance that names its count) · a specific technical detail with context, +0.2 · load-bearing for the next entry, +0.2 (its other signals total exactly 0.6; "Driver CLI" is the first markerless entry and its plan will carry the same fork-CI count; at this phase the entry does not carry the fact) → 0.8. Tier 3, the matched entry's own tier and location, so the write form is the in-place amend. One slot.
- Proof: report, *Spec claims disproved by measurement*, second bullet — the pattern as written read `0` at exit 1 on a green 16/16 run, CI#38013740580; read again with the codes dropped and either separator, 12; "the Windows job prints a backslash path". *Decisions & corrections*, the first three sweep hazards. The readings are in `escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/evidence/ci.md`.

## Recurrence despite a learning (→ the handoff's Deferred learnings)
- `recurrence-despite-learning: "A fork CI run with one job still open is not green, and how to read and re-run a hung job"` — its third paragraph says a pattern anchored on `Running tests/` finds nothing in a `gh run view --log` log because the colour codes stand as literal text. This chunk's plan wrote a gate entry with exactly that pattern, and the session's first tolerant read was written for the ESC byte and read 0 as well. The entry did not prevent it; a second entry is not the remedy — the check belongs to the step that authors a plan's CI-log entry.

## Filtered
- "`git diff-tree -r -m` on a merge commit lists files against both parents; the chunk's change is `git diff --name-only HEAD^1 HEAD`" — a specific technical detail, +0.2; nothing else: below the threshold.
- "Every region takes our side: take the whole file with `git checkout --ours`, re-apply the one kept line, prove it by the one-line diff" — a specific technical detail, +0.2: below the threshold.
- "Three claims in an evidence record were written ahead of their reading and corrected when read" — three instances in one record, +0.3, a specific detail, +0.2 → 0.5: below the threshold; the corpus already holds the rule ("A probe's write-up claims only what the probe read").

## Not candidates
- The operator's decisions the report lists (the test shape, the licence table, the two cause classes, the delegated operator pass) are decisions of the chunk, each with its home in a master or the plan.
- The guard refusal cost its one re-issue: the guard working, not a learning.
- The operator's note at this wrap's escalation — a boundary widening is the founder's own word also when it arrives by an upstream merge — is a judgment-base matter: it is recorded in the security sidecar and proposed for the playbook at the wrap's close, never written here.
