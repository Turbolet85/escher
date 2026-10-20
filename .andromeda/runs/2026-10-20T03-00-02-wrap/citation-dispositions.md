sweep: base: 165cfe44 — the previous sweep (2026-10-20T00-54-29-wrap) · re-pointed 43 · changed 3 · stretched 14

Rows as `cites.py apply` printed them (trail `cites-2026-10-20-load-wait.json`), each read at its citing line (by
offset, `cascade.py window`) and at the cited lines in the work tree, then dispositioned.

held .andromeda/architecture.md:128 Cargo.toml:47-58 — refused at 2026-10-13T15-50-37-wrap, not touched since «blitz-dom =…» → holds — stands (Cargo.toml outside this chunk's changed set: `git diff --name-only 165cfe44` over the seven held rows' files reads 0)
held .andromeda/architecture.md:135 examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (file outside the chunk's changed set)
changed .andromeda/architecture.md:192@c837 packages/escher-driver/src/lib.rs:119-158 → ? — the end rewritten → holds — re-pointed by hand to 119-158, the digits unchanged: line 119 is still `#![deny(missing_docs)]` and line 158 still the last re-export, rewritten to `pub use session::{LOAD_WAIT, Session};` — the range is still the module list and re-exports the claim names; the list of names it ends is amended under the report's fact `reexp`
stretched .andromeda/architecture.md:192@c2413 packages/escher-driver/src/session.rs:16-205 → 55-244 — changed inside → holds — 55-244 runs from `valid_label` to the end of `Session::act`, the session's body the claim («the crate names no app and no stand type, so the boot and the time step are the caller's») cites; the inside change is the act's settle through `settle_waiting`
changed .andromeda/architecture.md:192@c25212 packages/escher-driver/src/lib.rs:122-158 → ? — the end rewritten → holds — re-pointed by hand to 122-158, the digits unchanged: 122-158 still the module declarations and re-exports, the three schema modules with no `cfg` gate (line 158 now also names `LOAD_WAIT`)
stretched .andromeda/architecture.md:192@c40334 packages/escher-driver/src/execute.rs:95-253 → 95-254 — changed inside → holds — `Session::run` and its doc, the doc's settle sentence rewritten (report Symbols / APIs, `execute.rs:126-127`); no `NodeId` crosses still true
stretched .andromeda/architecture.md:192@c40380 packages/escher-driver/src/execute.rs:302-531 → 303-532 — changed inside → holds — the executor's arms through `settled_step`, which now settles through `settle_waiting` (`execute.rs:462`)
stretched .andromeda/architecture.md:192@c43150 packages/escher-driver/src/execute.rs:453-470 → 454-471 — changed inside → holds — `settled_step`, its span fields unchanged
stretched .andromeda/architecture.md:204@c27952 packages/escher-driver/src/execute.rs:453-470 → 454-471 — changed inside → holds — `settled_step` returns the step's diff, as the claim says
changed .andromeda/architecture.md:348@c4210 packages/escher-driver/src/lib.rs:121-158 → ? — the end rewritten → holds — re-pointed by hand to 121-158, the digits unchanged: 121-158 still every `mod` line and every re-export, the crate root's re-exports beside `Busy` and `Settled` the claim states
stretched .andromeda/security-plan.md:83@c9584 packages/escher-driver/src/execute.rs:95-253 → 95-254 — changed inside → holds — `Session::run`, validated first; the doc sentence change is the load wait (report fact `wait`)
stretched .andromeda/security-plan.md:125@c16701 packages/escher-driver/src/execute.rs:453-470 → 454-471 — changed inside → holds — `settled_step`, the diff before and after a step
held .andromeda/security-plan.md:235 .github/workflows/wpt-post-results.yml:29-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Fin…» → holds — stands (file outside the chunk's changed set)
held .andromeda/security-plan.md:352 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (file outside the chunk's changed set)
held .andromeda/security-plan.md:386 examples/screenshot.rs:41-42 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_co…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:36 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:46 examples/seven_guis/src/tasks/counter.rs:59-72 — refused at 2026-10-14T19-50-50-wrap, not touched since «background-…» → holds — stands (file outside the chunk's changed set)
held .andromeda/design-system.md:134 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (file outside the chunk's changed set)
stretched .andromeda/test-plan.md:22@c10001 packages/escher-driver/src/session.rs:207-336 → 246-438 — changed inside → holds — 246-438 is `session.rs`'s whole `#[cfg(test)] mod tests`, now holding the four load-wait tests (report fact `duunit`)
stretched .andromeda/test-plan.md:22@c10139 packages/escher-driver/src/schema_tests.rs:1-357 → 1-371 — changed inside → holds — the whole file (371 lines), the settled-help pin added
stretched .andromeda/test-plan.md:146@c10782 packages/escher-driver/src/session.rs:207-336 → 246-438 — changed inside → holds — as test-plan.md:22@c10001
stretched .andromeda/test-plan.md:146@c10921 packages/escher-driver/src/schema_tests.rs:1-357 → 1-371 — changed inside → holds — as test-plan.md:22@c10139
held .andromeda/test-plan.md:200 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…» → holds — stands (file outside the chunk's changed set)
held .andromeda/obs-plan.md:41 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (file outside the chunk's changed set)
stretched .andromeda/obs-plan.md:119@c6772 packages/escher-driver/src/execute.rs:453-470 → 454-471 — changed inside → holds — `settled_step`, its span fields unchanged
held .andromeda/a11y-plan.md:12 packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = […» → holds — stands (file outside the chunk's changed set)
held .andromeda/a11y-plan.md:16 tests/blitz-tests/Cargo.toml:17 — refused at 2026-10-10T01-56-50-wrap, not touched since «blitz-dom =…» → holds — stands (file outside the chunk's changed set)
stretched .andromeda/registries/contracts/test-plan/session-lifecycle.md:5@c5587 packages/escher-driver/src/session.rs:27-205 → 66-244 — changed inside → holds — 66-244 from the session's record of ids to the end of `act`; the recording's run path the claim cites unchanged
stretched .andromeda/registries/contracts/test-plan/session-lifecycle.md:5@c5633 packages/escher-driver/src/execute.rs:95-253 → 95-254 — changed inside → holds — `Session::run`
out-of-range .claude/docs/session-learnings.md:187 host.rs:350 — the file held 231 lines at the base → curation home — P3 (and no citation of the tree: the learning quotes `host.rs:350` as the example a `host.rs:35` replace would hit — no change)

unresolved 2 — `map`'s rows, no disposition owed.
