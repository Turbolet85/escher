# P5 check 4 (9) controls — 2026-10-16-handler-splits-and-readme

Scratch clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/{scratchpad}/clone` (`git clone --shared`), HEAD
`d236e9186e4ff6ca2610ddfbc9f54f590e8dd01b`. Each control fired as the entry: `gate.py run --plan
escher-0.2.0/chunks/2026-10-16-handler-splits-and-readme/plan.md --entry {n} --project-root {clone}`, no run dir. The
script is `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/{scratchpad}/controls.sh`, and its whole output was
read. Verdict lines as the tool printed them:

| entry | role | input | verdict |
|---|---|---|---|
| 5 | clone, unmodified | none | green · exit 1 · last line 0 (as on the real tree) |
| 6 | clone, unmodified | none | green · exit 0 · last line 0 (as on the real tree) |
| 7 | clone, unmodified | none | green · exit 1 · last line 0 (as on the real tree) |
| 8 | clone, unmodified | none | green · exit 1 · last line 0 (as on the real tree) |
| 9 | clone, unmodified | none | red · exit 0, 8 (as on the real tree) |
| 10 | clone, unmodified | none | green · exit 1 (as on the real tree) |
| 11 | clone, unmodified | none | red · exit 1 (as on the real tree) |
| 12 | clone, unmodified | none | red · exit 0, 3 (as on the real tree) |
| 5 | must-fail | `tests/blitz-tests/tests/stand_keyboard.rs` gains a comment line | red · exit 0 (count 1) |
| 5 | must-pass | `packages/blitz-dom/src/node/range.rs` gains a comment line | green · exit 1 (count 0) |
| 6 | must-fail | a new untracked `packages/blitz-dom/src/events/control.rs` | red · last line '1' |
| 6 | must-pass | a new untracked `escher-0.2.0/chunks/x/evidence/x.md` | green · exit 0 (0) |
| 7 | must-fail | `node/range.rs` gains `fn control() { let _ = Some(1).unwrap(); }` | red · exit 0 (count 1) |
| 7 | must-pass | `node/range.rs` gains a private `fn control_step(input: &str) -> usize` | green · exit 1 (0) |
| 8 | must-fail | `events/pointer.rs` gains `pub(crate) fn control() {}` | red · exit 0 (count 1) |
| 8 | must-pass | `events/pointer.rs` gains `fn control() {}` | green · exit 1 (0) |
| 9 | must-pass | README with prose «escher» capitalized, the three 0.1.0-era lines removed, the 0.2.0 epochs and three facts appended | green · exit 1 (0) |
| 10 | must-pass | the same README | green · exit 1 (0) |
| 11 | must-pass | the same README | green · exit 0 |
| 12 | must-pass | the same README | green · exit 1 (0) |
| 10 | must-fail | the same README plus a line naming `` `Escher-session help` `` | red · exit 0 (count 1) |

Entry 4's controls are step 1's one-shot pair at implement: the untouched tree must fail, and the merge-base copies
must pass. The instrument's reader does not exist before step 1.
