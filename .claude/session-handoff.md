# Session Handoff

**Last Updated:** 2026-10-10T02:42:17Z
**Branch:** build/escher-0.1.0 · 0 ahead of origin/build/escher-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-10-10-upstream-sync-agent-surfaces — upstream/main 7832c177 merged (61 commits); the masters reconciled to the merged engine

## Position
- Done: 2026-10-10-upstream-sync-agent-surfaces — upstream's 61 commits merged at `9462a7e4`, one edit of ours to upstream-owned code (`visible_region`'s reader call), fork CI green 16/16 on the merge (CI#38013740580).
- Next: /andromeda-phase on "Driver CLI" (working-route.md:73). It carries one new CARRY (a count over a fork CI log) and the WATCH below.

## Work done
- The wrap ran in two windows: P1 in the first (stopped there on the operator's word), P2–P7 in this one, resumed from `report.md`. Run dir `.andromeda/runs/2026-10-10T01-56-50-wrap/`.
- The citation sweep after the merge: 780 numbers re-pointed by the tool across the seven masters, 123 master rows read by hand (`citation-dispositions.md`): 22 re-pointed by hand, 26 claims read false and amended.
- 57 amendments in five masters (architecture 24, security-plan 11, test-plan 18, a11y-plan 3, design-system 1), 12 sidecar entries, 14 leaves re-derived (`fanout-results.md`, `cascade-dispositions.md`, `leaf-read.md`).

## Drift resolved
- Everything the merge made false or incomplete is restated: Parley a registry version (0.12), the merged dependency set and ICU4X, `text-transform-icu` in four default lists and `writing-mode` opt-in, Taffy's tree traits on `LayoutPassState`, the bounds reader renamed `physical_unrounded_geometry`, `autofocus` read by presence, the default link rule on `a[href]`, the CI-scripts leg at 78 tests over six files, the workspace baseline at 158 result lines · 719 passed, one test target per blitz-tests file with upstream's `all` target unbuilt.
- Two escalations, both answered by the operator through this wrap's question dialog (`escalations.md`): the font-source widening recorded as PROVISIONAL; the `text-indent` hanging/each-line sentence kept and marked `recorded, not established`.

## PROVISIONAL — one mark stands, for the founder's ruling at the Epoch 5 boundary
- Upstream #1109, arrived by the merge: an `@font-face` source with no format hint and no URL extension (a `data:` URL) is now fetched and byte-sniffed where it was skipped. No test of ours or of the delta covers it; escher routes no agent- or user-supplied URL through that path today.
- Sites: `security-plan.md` §Input Validation, the `@font-face` source row · `security-plan-amendments.md`, the entry "a hint-less `@font-face` source is fetched and sniffed…" · `.claude/rules/security.md`, Untrusted input · `.claude/docs/security-summary.md`, Font sources.

## Route owners placed at this wrap (the wrap's placement where the report named none — move any that reads wrong)
- "Driver CLI" (:73): `CARRY:` a plan entry that counts cargo lines in a fork CI log uses a pattern proven on a known positive · `WATCH:` a fork CI job hanging in its package-install step, 1 of 3 green runs (CI#38013740580; neither bound seen firing) · its scrolled-box CARRY now names the renamed reader.
- "Upstream sync ahead of polish and ship" (:82): three `CARRY:`s — our merge surface in `dioxus_document.rs` (58 / 0, 4 hunks) · in `scrolling.rs` (159 / 6, 6 hunks; `visible_region` calls an upstream crate-private function with two cfg bodies) with `input.rs` (+21 −1) · our standing two-line difference in `tests/blitz-tests/Cargo.toml`.
- "Quality gates" (:94): three `CARRY:`s — the licence-gate question, for the founder, undecided (the operator's word in this wrap's arguments) · upstream's blitz-dom clippy feature-set line the fork does not run · the per-OS test tallies of CI#38013740580, unattributed (139 result lines on three jobs; Windows 712 passed · 8 ignored).

## Notes
- **Proposed, not written — needs the operator's word:** a facet for the playbook's Boundary-widening rule, from the operator's note at this wrap's escalation: a widening that arrives by an upstream merge is recorded PROVISIONAL for the founder like any other. The playbook grows by approved appends only.
- **Named for the operator to relay to the pipeline owner:** the plan's fork-CI count entry was authored with a pattern the project's own learning (session-learnings, 2026-10-07) already said finds nothing — the check belongs to the step that authors a plan's CI-log entry (`curation.md`, the recurrence) · the citation sweep after a large merge leaves a hand read per row with no reading aid (129 rows here), and a `changed` row that holds at its old number prints `held` at the next sweep unless its citing line was touched.
- Carried from the last handoff, not confirmed relayed here: the five pipeline and host items of the Epoch 4 review (`.andromeda/runs/2026-10-09T21-37-22-wrap/curation.md`).
- Not measured, stated so in the masters: whether `text-indent`'s `hanging` / `each-line` work on the merged engine · the hint-less font-source path · what the ios and android CI `test` steps ran · unchanged from before: the two seven_guis binaries' stderr by level in a workspace-wide build, typed text in a sink-installing host's log, the windowed stand by level.
- Still owed from earlier chunks, unchanged: `Harness::scroll_into_view` has no check of its own; `session_common/mod.rs:3` is an over-long doc line (cosmetic). On disk outside git, unchanged: two build copies under `target/mutants-tmp/` (13 GB and 9.1 GB), not deleted on the operator's word.
- Next citation sweep: about six master citations that hold at an unmoved number on an untouched citing line (e.g. `examples/screenshot.rs:41-51`, `wpt-post-results.yml:29-49`) will print `held`; each is dispositioned `holds — stands`.
- No gated record, no PREREQ; one WATCH on the tail (above). Last failed command: none.

## Deferred learnings
- The cargo-test streams line deferred at the last wrap is applied (`.claude/rules/testing.md`, Session Additions). Nothing is deferred by the cap.
- `recurrence-despite-learning: "A fork CI run with one job still open is not green, and how to read and re-run a hung job"` (session-learnings, 2026-10-07) — its third paragraph already said a pattern anchored on `Running tests/` finds nothing in a `gh run view --log` log; this chunk's plan wrote exactly that pattern. The entry is extended with the Windows separator and the known-positive rule; the remedy is a check in the step that authors the plan entry, not a third entry.
