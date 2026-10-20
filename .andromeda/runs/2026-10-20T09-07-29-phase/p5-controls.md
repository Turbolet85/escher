# P5 controls — 2026-10-20-large-screen-in-parts

Every control below was fired AS THE ENTRY (`gate.py run --plan … --entry {n} --project-root {clone}`, no run dir) over a
shared scratch clone of the project at HEAD c1a2e5b85e7c0e5bfc5aabfa85fe5fc21b87f38d,
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/7618ca6e-4eb9-4f41-87aa-caf15ab634aa/scratchpad/clone`. The
scripts are `…/scratchpad/controls.sh` (entry numbers of the 21-entry fence) and `…/scratchpad/controls2.sh` (the two
census entries after the split into 22), and `…/scratchpad/controls3.sh` (the `gate.py guard` entry that replaced the
hand write-set guard and the hand untracked read at the operator's review, the fence then 21; the clone carrying a copy
of the chunk folder's plan.md, scope.md and research.md, which the guard reads). Each input was made in the clone and
reset after its reading (`git checkout -- .`, `git clean -fd`, the scope record removed). Entry numbers below are those
of the final 21-entry fence; the two hand guards' readings are kept at the end as they were read.

## The clone, unmodified — each read what the real tree read
- 6 (the new check's census): red, exit 2 (the file absent) — as on the real tree
- 7 (session_common's print and log census): green, exit 1, last line 0
- 8 (snapshot_text.rs sites): green, exit 0, last line 6
- 9 (escher-driver's span sites): green, exit 0, last line 2
- 10 (no manifest, stand or shared module change): green, exit 0, last line 0
- 11 (`gate.py guard`): green, `guard: clean`, changed 0

## The guard (entry 11), each fired as the entry
- must-pass — `packages/escher-driver/src/execute.rs` edited and `tests/blitz-tests/tests/stand_snapshot_parts.rs`
  minted: green, `guard: allowed 2 — packages/escher-driver/src/execute.rs, tests/blitz-tests/tests/stand_snapshot_parts.rs`
- must-fail — `packages/escher-driver/src/mcp.rs` edited, unrecorded: red, `guard: outside 1 files`
- must-fail — `tests/blitz-tests/tests/extra.rs` minted untracked, unrecorded: red, `guard: outside 1 files`
- must-pass — `packages/escher-driver/src/describe.rs` edited, recorded
  `` - `packages/escher-driver/src/describe.rs` · companion · serves step 4 · self ``: green,
  `companions excluded 1 — packages/escher-driver/src/describe.rs (serves step 4)`
- must-pass — `packages/escher-driver/src/part_tests.rs` minted untracked, recorded as a companion serving
  `packages/escher-driver/src/execute.rs`: green, `companions excluded 1 — packages/escher-driver/src/part_tests.rs
  (serves packages/escher-driver/src/execute.rs)`
- must-fail — `packages/escher-driver/src/describe.rs` edited, recorded as a companion serving step 99: red,
  `companion … serves step 99 — the plan has no step 99: counted`, `guard: outside 1 files`

## The two hand guards the guard replaced (read in the 22-entry fence, before the review)
- write-set guard: unmodified green, exit 1, last line 0; must-pass green with execute.rs edited; must-fail red, exit 0,
  with mcp.rs edited
- untracked read: unmodified green, exit 1, last line 0; must-pass green with stand_snapshot_parts.rs minted; must-fail
  red, exit 0, with extra.rs minted

## Inputs
- 6 must-pass — `tests/blitz-tests/tests/stand_snapshot_parts.rs` minted silent (`//! silent`, an empty `main`): green, exit 1
- 6 must-fail — the same file holding a `println!`: red, exit 0
- 7 must-fail — a `println!` appended to `tests/blitz-tests/tests/session_common/mod.rs`: red, exit 0
- 8 must-fail — a function holding `.unwrap()` appended to `packages/dioxus-native-dom/src/snapshot_text.rs`: red, last line 7
- 9 must-fail — a third `info_span!` appended to `packages/escher-driver/src/execute.rs`: red, last line 3
- 10 must-pass — `packages/dioxus-native-dom/src/snapshot_text.rs` edited: green, last line 0
- 10 must-fail — `Cargo.toml` edited: red, last line 1

## The literal atoms
- Entry 3's `contains 0 failed` is libtest's own summary form, read in entry 4's output on the real tree
  (`test result: ok. 6 passed; 0 failed; …`); its red baseline is cargo naming no target `stand_snapshot_parts`.
- Entry 21's job-log read: its known-positive control on CI#39387656399 is in the entry's own `note`.
