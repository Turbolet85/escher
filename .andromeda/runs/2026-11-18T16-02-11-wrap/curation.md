# Curation — 2026-11-18-served-error-page

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "On android the native prelude's glob shadows `Result`"
    Proof: CI#44308500657 `Test (android)` (job 134823305568) on `ab8e18b89` — E0107 · E0277 · E0308 at
    `examples/seven_guis/src/tasks/order.rs:90`/`:92`, the `Result` in scope being manganis 0.7.10's
    `src/android/java.rs:7` one-parameter alias through `dioxus_native::prelude`'s glob; fixed by `bc9be0469`
    (`std::result::Result`), read green by job 134855048119 (`evidence/operator-pass.md`). Score: verified by a real gate
    failure +0.4 · specific technical detail +0.2 · reached no other durable home this wrap +0.2 = 0.8. Tier 3: a
    gotcha with no rule file whose `paths:` scope an app crate; landed tx 2216.
  Filters: 6 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "On android the native prelude's glob shadows `Result`"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Dispositions
Implement's *Decisions & corrections*:
- The operator's directive for the re-entry run (gate 10 alone) — not a learning: a one-run directive.
- Guard refusal of a recursive `rm` in the smoke — rejected, Filter 1 dup: `host-linux.md` «`rm -rf` + `mkdir` + launch
  compounds get denied … no `rm` in a launch path» states it.
- Recurrence of testing.md 2026-11-14 (`gate.py control` reads no unittest result line) — rejected, Filter 1 dup; the
  entry's remedy (read by hand) was followed, so no defect recurred.
- Sweep hazard (a count-worded test name pinned as a witness) — rejected, Filter 1 dup: this wrap's P2 landed it in
  test-plan `decision/count-and-rename-sweeps` (`except`), and the cascade wrote it into `testing.md`'s body.
- A component keyed by HTML ids under no keyed element reads no owner — rejected, Filter 1 dup: P2 landed test-plan
  `decision/caused-owner-pinned-as-graph-reads`.
- No tracing re-export reaches an app crate through dioxus-native's prelude — rejected, Filter 1 dup: P2 landed
  architecture `decision/app-crate-takes-own-tracing`, and the cascade wrote it into `conventions.md`.
`## After implement`'s *Decisions & corrections after implement*:
- The pass's directives (stop on red, wait for the run, re-run a no-runner red up to three times) — not a learning: this
  pass's own directives; the CI-wait rule already stands in CLAUDE.md (2026-11-16).
- The android `Result` alias — applied, Tier 3 (above).
- The macOS/iOS no-runner cancels — not a learning here: an infra recurrence with no cause in the tree, routed at P5
  (the Recurrence-watch row).
- The wrap directive (inputs#I7) — rejected, Filter 2 task-specific: this wrap's own instructions.
This conversation:
- The Bash guard refused four calls of mine: a pipeline tool piped into `head`, then into `cut`, a `cd` out of the
  root, and a command substitution with `ls | head`. Each was re-issued once in the guard's form. The first two touch
  `host-linux.md` 2026-10-13 («never piped into `head`, `tail`, `grep`, `cut` …») — Filter 1 dup; no recurrence, since
  the guard held the rule at the cost of one re-issue each (the step records' `guard-refused`).
Citation sweep: `.claude/docs/session-learnings.md:356` `host.rs:350` out of range (`citation-dispositions.md`) — no
change: the line is an illustrative prefix example, not a claim about `host.rs`.
