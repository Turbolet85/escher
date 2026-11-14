CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A libtest name filter is a substring match" · + "A provider that needs a Dioxus runtime, read out after the rebuild"
  Filters: 0 dup · 1 task-specific · 3 below threshold (exactly 0.6) · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "A libtest name filter is a substring match" · "A provider that needs a Dioxus runtime, read out after the rebuild"
  CLAUDE.md size: 164/200 · T1 4.6 KB, 0 over 600 B

Applied:
- T3 "A libtest name filter is a substring match" — confidence 0.8 (measurement +0.4: the plan's unit entry was shaped by it — a `history::tests` filter would select `browser_history::tests`, so the entry runs the whole crate and asserts six `contains … ok` atoms; detail +0.2; no other home +0.2: no master, rule, route annotation or contract carries it). Tier: not path-scoped to a rule file's `paths:` (gate entries live in plans) → Tier 3.
  Proof: report *Decisions & corrections* «Gate libtest hazard (plan note)»; implement's gate trail `.andromeda/runs/2026-11-14T17-29-22-implement/gate-2026-11-14-remote-pages-local-files.json` (entry 1, its six atoms).
- T3 "A provider that needs a Dioxus runtime, read out after the rebuild" — confidence 0.8 (measurement +0.4: implement deviation 2 — the plan's in-component asserts replaced; detail +0.2; no other home +0.2: test-plan's proposer answered it `@row_none` type 4, «the code and the report's test notes hold it»). Tier: an in-crate `src` test module, outside `testing.md`'s `paths:` (tests/**) → Tier 3.
  Proof: report *Deviations from intent* 2 and *Decisions & corrections* «Dioxus test pattern»; `apps/browser/src/history.rs` listing 137-159 (`thread_local!`, `navigate_app`, `current_url_after`).

Decisions & corrections, each dispositioned:
- Guard refusal (implement), `cat` heredoc → not a learning: the guard put the call on its documented path at one re-issue (guard-refused).
- Guard refusal (this wrap), a pipeline tool piped into `head` → not a learning: the same, at one re-issue; host-linux.md's 2026-10-13 entry already states it (the guard enforces it), no recurrence past the guard.
- Operator directives (inputs#I1) applied by implement → not a learning: chunk-specific.
- Controls (5 holds) → not a learning: an outcome.
- A struct literal of a type gaining a private field breaks outside sites; a moved field forces the build before the literal → rejected, Filter 4: 0.6 exactly (measurement +0.4, detail +0.2) — Rust's ordinary semantics.
- Dioxus test pattern → applied (above).
- Gate libtest hazard → applied (above).
- Sweep hazard (provider tokens in no master) → rejected, Filter 2: task-specific (names this chunk's symbols, not a hazard of a pattern form).
- Basis correction (the plan's PROVISIONAL list narrower than the store) → rejected, Filter 4: 0.6 exactly; the marks' state is now carried by this wrap's sidecar consolidation and handoff.
- Relay reading (inputs#I2's stale «Not answered» section) → not a learning: one relay's state.
- The operator's word at invocation → not a learning: a directive for that wrap.

This window:
- The founder's words for the 15 sidecar marks stood in a 0-pending wrap's inputs and adaptation record, not in a sidecar entry, which the report's query read alone → rejected, Filter 4: 0.6 exactly; the consolidation records them in the sidecars.
- The operator's guard on a digit-only cascade pattern (measured on another project, rec 1.46) → not a learning here: a directive for this wrap about a pipeline tool's behaviour, the pipeline's to fix.
- Citation sweep row `.claude/docs/session-learnings.md:326 host.rs:350` (out-of-range, curation home) → no change: an illustrative example of a prefix match inside a session-learnings entry, not a claim about `host.rs`; preserve-verbatim.
