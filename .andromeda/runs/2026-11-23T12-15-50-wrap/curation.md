CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "While an editor's cargo runs on the shared `target/`, cargo can read a crate `Fresh` against sources just edited … `cargo clean -p {crate}` clears it"
    Proof: implement run 2026-11-23T10-01-43-implement — gate 4 firings 2–3 and gate 5 firings 1–3 read the pre-edit blitz-dom while rust-analyzer's `cargo check --workspace` (pid 2979501) ran; a `touch` and a re-run did not clear it, `cargo clean -p blitz-dom` (2.8 GiB) did (report Deviations · Decisions & corrections, fct-wrcglkpd).
  Tier 3 (.claude/docs/session-learnings.md): extended + corrected (below)
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Extended: T3/session-learnings.md: "2026-11-22 — A lock move is proven by a locked build, not by the audit leg" + "a targeted `cargo update -p` is not a lock-only move on this lock; a patch bump edits the version and checksum lines alone"
    Proof: `evidence/cookie-lock-move.diff` — `cargo update -p cookie --precise 0.18.3` moved the `syn` edges of `bindgen` and `cssparser-macros` 3.0.6 → 2.0.119; the hand edit read `3+2` by `git diff --numstat`, built `--locked` with no warning and audited `advisories ok, licenses ok` (report fct-cbi3t2u2, fct-wgpw7fkq).
  Corrected (in place, cap-exempt): T3/session-learnings.md "2026-10-12 — `cascade.py sweep --pattern` takes its ids after one flag" → "is a list option": a repeated `--pattern` adds to its list.
    Proof: this wrap's P1 re-fire with eight repeated `--pattern` flags printed `total (8 patterns)` (`cascade-2026-11-23-canvas-placement-and-text-pre-pass.json`); amendment-flow §Tool (cascade v1.37) states the flag repeats.
  CLAUDE.md size: 166/200 · T1 5.7 KB, 0 over 600 B

Decisions & corrections, each weighed:
- The operator's hold (inputs#I6) — not a learning: a one-run instruction, executed.
- The operator's go (inputs#I7) — not a learning: a one-run instruction.
- The plan revision (inputs#I8, I9) — not a learning: the chunk's own scope.
- The operator's word at the P4 stop (inputs#I10) — not a learning: carried by residual res-evkq636g (P5).
- Sweep hazard, `cargo update -p cookie` re-resolves unrelated edges — extended (T3, above).
- Sweep hazard, `Fresh` against edited sources on the shared `target/` — applied (T2, above).
- Red re-fires with no edit between them not written down first — not a learning: the implement skill's own rule, its miss telemetry of the tool, not curated.
- Guard refusals re-issued once (implement, both runs) — not a learning: guard-refused, the host rule already states the forms.
- The predicted red readings held exactly — not a learning: the chunk's own measurement, the checks pin it.
- After implement: the wrap directive (inputs#I11) — not a learning: this wrap's instructions, executed.
- After implement: the operator pass ran by hand, green — not a learning.
- Citation sweep row (`citation-dispositions.md`): `.claude/docs/session-learnings.md:431` `host.rs:350` out-of-range — no change: an illustrative example of a citation prefix, not a citation of `host.rs`.
- This wrap: the leaf comparer refused for quoting the last non-marker line of CLAUDE.md — not a learning: telemetry of the tool (evolve).
