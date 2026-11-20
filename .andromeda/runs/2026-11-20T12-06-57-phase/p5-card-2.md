# P5 review (2) — 2026-11-20-browser-s-surviving-mutants: "Browser's surviving mutants"

**Plan:** `doc:2026-11-20-browser-s-surviving-mutants/plan` rev 5, with your review folded in (inputs#I2). Open it in an editor at `.andromeda/cache/docs/2026-11-20-browser-s-surviving-mutants/plan.md`. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-browser-s-surviving-mutants` diffs the chunk's records.

## What the fold changed
- **Tests out of line.** The two `render_scene` tests now live in the new `apps/browser/src/capture_tests.rs`. It is declared at the end of `capture.rs` by `#[cfg(test)]` · `#[path = "capture_tests.rs"]` · `mod tests;`, after one blank line. The form is `packages/escher-driver/src/cli.rs:556-559` → `cli_tests.rs`: a `//!` header, then `use super::*;`, the tests keeping their `capture::tests` paths. The rows that bind it, architecture `decision/unit-tests-at-file-end` and test-plan `decision/inline-cfg-test-modules`, name this form in their exceptions.
- **Entry 3** now also asserts the tests file stands: `test -f apps/browser/src/capture_tests.rs`, then no line of `capture.rs` removed or rewritten. Baseline red: the file is absent on the untouched tree, which is the subject.
- **Entry 4** counts hunks opening before line 122, plus added non-blank lines in `capture.rs` other than the three declaration lines. It must read 0.
- **Entry 5** `--allow` widened to `apps/browser/src/capture(_tests)?\.rs|\.claude/session-handoff\.md`.
- **Controls re-run** in the clone (`p5-controls.md` rev 2):
  - All three entries read green on the declaration plus the tests file.
  - Entry 3 reads red with no tests file, and with a `capture.rs` line deleted.
  - Entry 4 reads red with a line inserted at 50, and with an inline test function appended after the declaration.
  - Entry 5 reads red with `README.md` changed, and with a third `capture_extra.rs` added.
- **Research and scope** were updated to match: research rev 2 (`New files` lists `capture_tests.rs`), scope rev 5 (Boundaries).
- **The row read was not run again.** The new file is the test module of the same capture module. The rows that decide its form were already picked: the two exception rows above, plus the expect/unwrap justification and the test-document setup. Say so if you want it run again.
- **Constraints:** the inline module is recorded as rejected at your review.

## Unchanged from card 1
- The goal: catch the four `capture.rs:86:5` survivors with a direct test of `render_scene`'s returned size, `(800, 600)` at scale 1 and `(1280, 720)` at scale 2, as literals.
- The four hand controls through `gate.py control` must read `control: 4 holds`.
- `fast` returns 0.
- The operator entries: hygiene, push, CI green.
- Entries 9–10 fire only on the founder's word.
- Entry 1 baseline red (0 passed). Entry 2 baseline green (487.8 s).

## Checks
- Mechanical checks pass on rev 5: the fence parses; planlint reports 0 hits; inputs I1 and I2 resolve; research's lists parse (1 new, 1 modify); every new entry carries its baseline.
- The «unchanged» claims hold after the chunk: upstream's lines 1-122 and `Cargo.toml`.
- Validation-1 is aligned with scope rev 5.

## Capabilities
- **No claim, accepted by your review.** v040-08 is not provable by this chunk: its acceptance turns on the next boundary audit. After the word, a dated `notes` line records it.

## Beyond the plan
- (standing — .andromeda/runs/2026-11-20T12-06-57-phase/p5-card-1.md) **v040-08's claim.** After this chunk, no markerless entry carries v040-08, and its acceptance waits on the Epoch 2 boundary audit. Owner: a `CARRY:` on «Dependency and CI debts», the epoch's last entry, so that chunk claims v040-08 against the boundary audit's reading. Accepted at your review (inputs#I2).
- (standing — .andromeda/runs/2026-11-20T12-06-57-phase/p5-card-1.md) **No cargo-mutants reading if the founder declines.** If he declines entries 9–10, the four stand caught by the hand controls alone, and no later audit lists them (`--in-diff`; no line of `render_scene` changes). Owner: none; the report records his decline. Or, on his word, the operator pass fires 9–10. Accepted at your review: 9–10 are not run unless the founder says so.

Apply? (yes / review / cancel)
