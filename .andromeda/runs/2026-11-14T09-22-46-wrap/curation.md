CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A control for a font-family reading removes nothing from a one-font context — … make the red control a font context holding other fonts" (confidence 0.8)
    Proof: report Deviations 1 and Decisions & corrections 1 — control `serif-append` DOES NOT HOLD (in place green, removed green; `{work_dir}/evidence/control-1.txt`), substitute `families-host` holds (removed red; `{work_dir}/evidence/control-2.txt`). Signals: verified by measurement +0.4 · technical detail +0.2 · no other home +0.2 (P2 rejected test-plan's `except` for it — no master holds it).
  Tier 3 (.claude/docs/session-learnings.md): + "A sweep alternative `cursive` matches «recursive»" (confidence 0.8)
    Proof: report Decisions & corrections «Sweep hazard» — 4 of the `families` pattern's 10 standing rows false hits (`cascade-2026-11-14-bundled-fonts-pre-pass.json`). Signals: measured +0.4 · detail +0.2 · no other home +0.2.
                                              + "A `gate.py control` controls file: a `run` holding a newline takes a `'''` literal" (confidence 0.8)
    Proof: implement outcome deviation 3 — the first `control-1.toml` refused as not TOML, re-written with `'''` and re-fired (`.andromeda/runs/2026-11-14T08-42-37-implement/outcome-2026-11-14-bundled-fonts.md`). Signals: a tool refusal measured +0.4 · detail +0.2 · no other home +0.2.
  Filters: 0 dup · 1 task-specific · 0 conflict (→ handoff) · 1 deferred (→ handoff)
  No-other-home: "a font-family control on a one-font context is vacuous" · "`cursive` matches «recursive»" · "a controls-file `run` with a newline takes `'''`"
  CLAUDE.md size: 164/200 · T1 4.6 KB, 0 over 600 B

Decisions & corrections — each entry's disposition:
1. One-font fontique collection; a registration-removal control is vacuous → applied (Tier 2, testing.md).
2. The operator's word: the expected design-system row states the fallback → not a learning — a ruling, landed in architecture `decision/stand-typography-dejavu-sans` (tx 862) on Q1 A.
3. The working directive (founder away; forks to founder-forks.md; escalations with fit verdict; read-backs with --run-dir) → not a learning — this wrap's operating directive, restating inputs#I1.
4. `gate.py control` `case` is the test name alone (inputs#I5) → deferred (→ handoff), Filter 5 cap: four candidates at 0.8, this the one measured on another project and already met by this chunk's controls.
5. A controls file's `run` holding a newline takes a `'''` literal → applied (Tier 3).
6. Digests read once from a scratch print, then the print removed → rejected — Filter 2/4: a one-off procedure of this chunk (0.2 − 0.3).
7. Bash guard refusals (a `cd` into the scratchpad; a tool output piped into `tail`) → not a learning — the guard hook already refuses the act.
8. Sweep hazard `cursive` inside «recursive» → applied (Tier 3).
9. The P5 card's Beyond-the-plan row 1 resolved to nothing deferred → not a learning — a resolved condition of this chunk.

This window's conversation:
- Q1 A (the operator, 2026-11-14): one home per fact; the stand's typography is architecture's → not a learning — the architecture row itself carries the placement clause.
- Citation disposition `.claude/docs/session-learnings.md:316 host.rs:350` (out-of-range, curation home) → no change: an illustrative number inside a learning about prefix matches, not a claim about host.rs.
