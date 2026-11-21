CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A kept check read back by `Check::from_bytes` gives each call's arguments in name order (serde_json's map), not the schema order the file writes — compare read-back arguments by name, and pin the file's order from its text."
    Proof: implement's report, Deviations and Decisions & corrections — `recording_tests`' round trip read the arguments back `ctrl` before `key` while the file writes `key`, `ctrl`; the test was rewritten to compare by name and pin the text (implement run 2026-11-21T06-09-05-implement). Score: measured +0.4 · specific detail +0.2 · no other home +0.2 = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 4 below threshold
  No-other-home: "A kept check read back by `Check::from_bytes` gives its arguments in name order"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

Dispositions of every Decisions & corrections entry:
- implement · the operator's pause and resume of the run («start NOTHING after it … wait for my word»; «Re-run entries 2 and 6 …») — not a learning: a directive for that run alone.
- implement · a handler's panic unwinds out of dioxus-core's `Runtime::handle_event`, so a write after the dispatch needs a drop guard — rejected, Filter 4: 0.6 (measured +0.4, detail +0.2); its home is architecture `decision/reached-record-before-dispatch`, amended this wrap (tx 2813), so no conditional signal applies.
- implement · on a focused `multiple` select a `press space` after plain arrows flips the chosen row out — rejected, Filter 4: 0.6 (measured +0.4, detail +0.2, paraphrased from implicit behaviour −0.2, no other home +0.2) — the engine's own rule (Space flips the current row) already stated in the rules' body.
- implement · a Cells cell's click opens its editor unfocused, so a key needs a click on `cell-input` first — rejected, Filter 2/4: one example app's behaviour (could be task-specific −0.2), 0.6.
- implement · a kept check's arguments read back in name order, not schema order — applied, Tier 2 `testing.md` (above).
- implement · Bash guard refusals (a `cd` moving the cwd; a `cat` heredoc with a file target) — not a learning: the hook already refuses the act (no-other-home excluded), telemetry of the tool.
- after implement · the operator's stop of the pass for a host restart — not a learning: one run's event.
- after implement · the wrap directive `inputs#I3` — not a learning: this wrap's own direction, carried out.
- this window · the citation sweep's row `.claude/docs/session-learnings.md:405 host.rs:350` (out of range, curation home — P3) — no change: an illustrative example of a prefix collision inside a Tier 3 entry, not a citation of `host.rs`; preserve-verbatim.
- this window · three leaf comparers missed stale lines the step-2 sweep listed — not a learning: pipeline telemetry (evolve record).
