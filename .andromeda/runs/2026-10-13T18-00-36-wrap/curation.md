CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "Write a founder's word only as he gave it — his chosen label verbatim, in its own script (Cyrillic stays Cyrillic) — never a relay's transliteration of it, in any file a skill writes, a run-dir relay copy included." (confidence 0.7)
    Proof: the operator's correction at this wrap's resume (inputs#I4): «the founder's label is «Ратифицировать», in Cyrillic, exactly as arch-amend :534 holds it … don't write it anywhere» — the transliteration had already landed in relay-3.md and the I3 copy, both respelled on his word (I3 reads `copy altered`). Signals: explicit user correction +0.4, never-language +0.3 = 0.7.
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred · 4 below threshold · 2 recurrences (→ handoff)
  CLAUDE.md size: see the P7 report (health.py check 1)

Decisions & corrections, one line each:
- The background-image read through `resolved_style_value` rather than a `style` dependency — not a learning: a design decision of this chunk, now stated in architecture §Standard Contracts → Test harness (P2).
- blitz-tests reads the harness's colours through methods and fields, so it takes no `color` dependency — rejected, Filter 4 (specific technical detail +0.2, one-off −0.3).
- The `Cargo.lock` line already written before the first `--locked` run, its writer unmeasured — rejected, Filter 4 (no measurement; a hypothesis).
- The gate tool's header prints `$TMPDIR/andromeda-gate/…` with `TMPDIR` unset, and a read through the literal variable failed — rejected, Filter 1: `docs/session-learnings.md` "`$TMPDIR` is unset in the Bash tool's shell on this host" (2026-10-07) records the defect → recurrence-despite-learning, handoff; the header itself is a pipeline letter, logged to evolve at implement.
- Hygiene refused the implement outcome file for a host temp path in its prose — rejected, Filter 1: `docs/session-learnings.md` "Committed run-dir and evidence text must not spell a host temp path, even as prose" (2026-10-06) → recurrence-despite-learning, handoff.
- The Bash guard refused a `cd` into `.andromeda/` (P1) and, at this resume, into a skill reference dir and the run dir; each re-issued once — rejected, Filter 1: `host-linux.md` "`cd` only inside a subshell" covers it; a refusal at its one re-issue is not a recurrence.
- `cascade.py sweep --pre-pass` refused pattern ids over 16 characters — rejected, Filter 2 (the tool's documented limit, amendment-flow §Tool; pipeline, not project).
- The operator's directions (the operator pass and its stop; «run P1 only and stop»; the resume's word, inputs#I3) — not learnings: directions this wrap carried.
- The standing word, inputs#I1 (no mutation run; pipeline tools bare or redirected) — not a learning here; its one recurrence at this resume (`inputs.py verify` piped into `tail`) → recurrence-despite-learning against `host-linux.md` 2026-10-13, handoff.
- The P5 card's four CARRYs (inputs#I2) — not a learning: route freight, written at P5.

This conversation, beyond the report:
- The operator's correction of the founder's label (inputs#I4) — applied, Tier 1 (above).
- A recursive grep over the repository root ran past the 120 s Bash timeout (target/ walked) — rejected, Filter 4 (specific technical detail +0.2, one-off −0.3).
- The report's site sweep reached its pattern wording only (obs-plan.md:70 "17 of which") — rejected, Filter 1: report-template's own clause "a listing reaches its patterns' wording and no further" (pipeline); evolve holds it.
- The splice `--lines` count asserted one too high for a payload with a trailing blank line — rejected, Filter 2/4 (a one-off typing slip, the tool caught it).
- test-plan proposal 5's count, derived by its detector and re-raised with a measured basis — rejected, Filter 2 (the opening rule already states it).

The citation sweep's curation-home row: `.claude/docs/session-learnings.md:71 host.rs:350` (out-of-range) — read: the
entry's own example ("`host.rs:35` heads `host.rs:350`"), a quoted string, not a citation of the tree — no change,
preserved verbatim.
