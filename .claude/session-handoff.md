# Session Handoff

**Last Updated:** 2026-10-09T21:22:52Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-09-audit-corrections-agent-surfaces — the surviving snapshot mutant killed on a textarea fixture; fork CI's package install bounded and retried

## Position
- Done: 2026-10-09-audit-corrections-agent-surfaces (28 master records, all complete) — the corrective entry the founder placed first in Epoch 5. It claimed no capability.
- Next: /andromeda-phase on "Upstream sync ahead of agent surfaces" (working-route.md:71). It carries two merge-surface CARRYs and one WATCH (below).

## Work done
- The chunk: a textarea on the in-file fixture of `stand_snapshot_state` and one test that reads its typed text back as the snapshot's value (the mutant the Epoch 4 audit left alive now turns that test red); `.github/scripts/apt-install.sh`, called by nine `run:` steps of `ci.yml` — 3 attempts, 120 s per `apt-get` call — and `timeout-minutes: 10` on the matrix job's apt-cache action step. Green in CI#37985678276 on `6c545ced`, 16 of 16.
- The wrap, resumed from `report.md` in a fresh window (run dir `.andromeda/runs/2026-10-09T20-50-14-wrap/`): the two items carried from the adaptation wrap are done — see the next two sections.

## Drift resolved
- **escher's first citation sweep is written** (the operator's word, 2026-10-09, at this wrap's ask: all 40, none withheld): 40 citations re-pointed by the tool in five masters, each read true first (`first-sweep-read.md`); 4 doc-header citations re-pointed by hand to `stand_snapshot_state.rs:1-7`; 20 other rows read and dispositioned, none left standing (`citation-dispositions.md`). Later wraps sweep from this commit and never ask.
- 13 amendments, 0 escalations (`fanout-results.md`): architecture 3 (the install script's contract, the CI/CD pattern, the Stack row) · security-plan 1 (an Input Validation row for the script's arguments) · test-plan 7 (the install tests and invariant, the textarea reading, the counts — CI scripts 70, workspace 657 · 0 · 10, `run stand` 110) · a11y-plan 1 · obs-plan 1 (raised from the plan's list: the install's output stays in the step log). 8 leaf lines re-derived, CLAUDE.md's CI pointer row among them (`cascade-dispositions.md`).
- **The playbook rule the founder approved is appended** (word: "Добавить" — the founder, 2026-10-09, relayed verbatim by the overseer): a spec claim a chunk's measurement disproves, about code the chunk did not edit, is amended to the measured limit and its fix pinned as a CARRY; routine.

## Route owners placed at this wrap (the wrap's placement — move either if it reads wrong)
- `WATCH:` on "Upstream sync ahead of agent surfaces" (:71): a fork CI job hanging in its package-install step, 0 of 3 green runs. No run has shown the bound or the android step's timeout fire; the stand-in tests are the bound's one witness.
- `CARRY:` on "Quality gates" (:94): a bound that fires inside `apt-get install` can leave dpkg interrupted (unhandled, not seen); and the script ran on real runners in eight jobs, not nine — the guarded `python3-yaml` call is not reached where PyYAML is on the image.

## Notes
- **A pipeline-tool finding, no route owner (it is not escher's):** the new-text listing's row for the install script's `while` loop ends one line early (`25-41`; the loop's `done` is `:42`). A range of that file cited from the listing is read at its last line first. Second one, from this wrap: the citation tool reads a backticked list of bare `:N` numbers after an uncited file name against the path cited before it (working-route.md:69, three `out-of-range` rows) — recorded in the friction ledger.
- Not shown by anything in this chunk: the install bound firing on a real mirror stall; the android step's timeout firing. Not read: the ios and android job logs beyond their install step; the MSRV job's log.
- Still owed from earlier chunks, unchanged: `Harness::scroll_into_view` has no check of its own; `session_common/mod.rs:3` is an over-long doc line (cosmetic). The windowed witness of the accessibility-tree refresh is a CARRY on "Stand a11y assertions".
- Not measured, stated so in the masters, unchanged: the two seven_guis binaries' stderr by level in a workspace-wide build · typed text in a sink-installing host's log · the windowed stand by level.
- On disk outside git, unchanged: two build copies under `target/mutants-tmp/` (13 GB and 9.1 GB), not deleted on the operator's word.
- No gated record, no PREREQ; one WATCH on the tail (above). Last failed command: none.

## Deferred learnings
- Curated at this wrap from the report's Decisions & corrections (`curation.md` in the run dir): two `testing.md` entries (a mutation control restores by bytes and sha256; a red `cargo test` run's result lines need `--no-fail-fast`) and one Tier 3 entry (a step's seconds in the jobs API do not say whether a guarded command ran). The conversation that produced the chunk was cleared before the resume, so a correction only it held is not curated.
- recurrence-despite-learning: "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) — the cat-heredoc guard fired once more at this chunk's implement (refused, re-issued as an Edit).
- Curation conflict, for the operator, unchanged: `.claude/rules/host-linux.md` says "The transport collapses a BACKSLASH PAIR `\\` to `\` before bash sees it, inside a quoted heredoc too"; the 2026-10-07-driver-command-spans report records two payloads holding backslash pairs, sent in quoted python heredocs, that "landed as written". One counter-reading; the rule stands unedited.
- Carried, for the founder's review (set by him, 2026-10-07; not held at this wrap): "The project's Bash guards refuse a heredoc written to a file and a leading cd" (Tier 3, 2026-10-06) · "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) — **its hand procedure is now the citation tool's sweep, first written at this wrap; the entry was left unedited for that review** · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05) · recurrence-despite-learning: "A per-crate `cargo clippy` is not the CI lint leg" (Tier 3, 2026-10-07).
