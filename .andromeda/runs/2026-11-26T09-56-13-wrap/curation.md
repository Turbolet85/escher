CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "The engine focuses a custom-widget element on pointer down" (confidence 0.8)
  Filters: 3 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 below threshold
  No-other-home: "The engine focuses a custom-widget element on pointer down"
  Load-bearing: "a real gate.py --operator firing of the push entry writes the committed trail before its guard reads" → the first markerless entry of Epoch 5 (its operator pass carries a push entry)
  Extended: T3/.claude/docs/session-learnings.md: "2026-10-13 — `gate.py run … --operator {n} --dry-run` writes the run dir's gate trail" + "a real firing through --operator writes it too, before the guard reads"
  CLAUDE.md size: 169/200 · T1 6.9 KB, 0 over 600 B

Applied:
- T3 new — "The engine focuses a custom-widget element on pointer down" (0.8: measurement +0.4, specific detail +0.2, no-other-home +0.2).
  Proof: control C6 read the app's `set_focus` inert (implement report, Reverted / negative API facts; `{work_dir}/evidence/control-1.txt`); fit read the architecture add decision/widget-focused-on-pointer-down `not_master` type 4 (`architecture-fit.md`), so no master holds it.
- T3 extended — the 2026-10-13 `--operator --dry-run` trail entry (0.8: real gate failure +0.4, specific +0.2, load-bearing +0.2). Also a recurrence despite that entry (→ handoff, recurrence-despite-learning).
  Proof: `{work_dir}/evidence/operator-pass.md`, entry 16 — `gate.py run --operator 16` red, exit 1, 0 B log; `git status --short`: ` M .andromeda/runs/2026-11-26T07-21-03-implement/gate-2026-11-26-canvas-keyboard-path.json`.

Dispositions of every *Decisions & corrections* entry:
- implement · the operator (inputs#I7): the Tab trap an engine defect, fixed PROVISIONAL — not a learning: a decision, its home architecture decision/widget-tab-pass-through (landed P2).
- implement · measured C6: the engine focuses a custom-widget element on pointer down — applied (T3 new).
- implement · sweep hazard: a `data-circles` write sets no app state — rejected, duplicate: test-plan decision/circles-apart-written-through-attribute now states it (P2) and testing.md line 35 was re-derived to it.
- implement · sweep hazard: `circle-close` below the 600 px view — rejected, duplicate: layout-templates decision/circle-drawer-canvas-keeps-size (P2).
- implement · guard refusals (a `cat` heredoc with a file target, a `cd` out of the root) — rejected, duplicate: host-linux.md §Transports and §Paths state both.
- implement · clippy `wrong_self_convention` refused a consuming `as_*` method, renamed `into_*` — rejected below threshold (0.6: gate failure +0.4, specific +0.2; the clippy gate itself refuses the act, so no-other-home cannot fire).
- implement · recurrence of testing.md 2026-11-25 (sweep a fixed text by its opening clause) — not a learning: the standing entry was followed and worked.
- close · the operator (inputs#I8): the Tab pass-through moved into escher's own function — not a learning: architecture decision/fork-additions-in-own-functions already rules it.
- close · the operator, the pass: «Trail-only fix commit», then a record-only commit and entries 16–18 by hand — not a learning on its own: the ruling's lesson is the extension above.
- close · measured: `gate.py run --operator` on a clean-tree guard writes its trail first — applied (T3 extension).
- close · the wrap directive (inputs#I9): whose word, owners, residuals, the upstreamable flag, the epoch batch not discharged — not learnings: rulings carried by P2 (sidecar Why), P5 (route) and the handoff.
- this wrap · the site sweep's TOML literal string with a doubled quote, refused by cascade.py — rejected task-specific (TOML's own grammar; one event).
- citation sweep · `.claude/docs/session-learnings.md:450` cites `host.rs:350`, out of range — no change: an illustrative example of a prefix-match hazard, not a citation of the file's content.
