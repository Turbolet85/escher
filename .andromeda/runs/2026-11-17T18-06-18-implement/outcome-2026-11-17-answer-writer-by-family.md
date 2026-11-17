# Implement outcome — 2026-11-17-answer-writer-by-family

**Outcome:** green. Smoke skipped: no boot path or UI surface changed, and the plan lists no smoke or self-verify entry.
**Run:** `2026-11-17T18-06-18-implement`

## Files changed
- modified `packages/escher-driver/src/json.rs`: 1899 → 303 lines. It keeps the primitives (`string`, `number`, `ratio`, `Object`, `list`), the shot, `impl Coverage`, `shot_without_image`, `impl Outcome` (its arms now call `caused::`, `diff::`, `lint::`, `graph::`, `actions::` and `layout::`), `impl Refusal`, `impl SessionError`, `started`, `status` and `stopped`. It declares the seven private children and the `#[path = "json_tests.rs"]` tests.
- new `packages/escher-driver/src/json/snapshot.rs` (95), `diff.rs` (57), `caused.rs` (92), `layout.rs` (170), `graph.rs` (84), `actions.rs` (26), `lint.rs` (50).
- new `packages/escher-driver/src/json_tests.rs` (1066): the old `mod tests` body, dedented one level.

## Deviations from the plan, with justifications
1. **The tests' `use` lines grew beyond step 5's list.** Step 5 named `snapshot::{NodeParts, node, reading_word}` and `layout::{cause_word, length, placer_word, shift_word}`. The tests also call `layout::placement`, which is now `pub(super)`. Through `use super::*` they also reached about 20 types the parent had imported and no longer does: `Busy`, `GraphNode`, `GraphState`, `Length`, `PlacedBy`, `Reading`, `ShiftBy`, `SizeCause`, `SizeKeyword`, `SizeReason`, `SnapshotDiff`, `WriteFlag`, `ActionNode`, `Caused`, `CausedPast`, `LayoutExplanation`, `Placement`, `Placer`, `ShiftReading`, `Bounds`, `Finding` and `Reason`. They are imported by name. Only `use` lines changed; no test body, name or literal moved. `caused::*` is not imported, so the tests' own `caused_item` fixture stays unshadowed.
2. **The seven `mod` lines are in alphabetical order** (actions, caused, diff, graph, layout, lint, snapshot), not step 3's family order. `cargo fmt`'s default `reorder_modules` sorts them, and the `fmt` leg gates on that.
3. **`json/layout.rs` imports `crate::lint::Bounds`** for `box_object`. The parent imported it from the same path before.
4. **`NodeParts`' fields are widened to `pub(super)`** along with the struct. `diff::nodes` and the tests build it from outside `json::snapshot`.

The scope record has no lines: `gate.py scope` reads `clean — changed 9 · listed 9 · recorded 0`.

## Verbatim check
A scratch verifier compared the split with `d875c3d22:packages/escher-driver/src/json.rs`. `json_tests.rs` reads identical, `use` lines aside. Every body line of the children and the parent comes from the original, and every original writer line has a home (`{work_dir}/evidence/after.md`).

## Gates
`entries 15 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 3` on the first firing. The trail is `.andromeda/runs/2026-11-17T18-06-18-implement/gate-2026-11-17-answer-writer-by-family.json`, and the lines are in `{work_dir}/evidence/after.md`. Entries 13–15 (hygiene, push, CI read) are `leg = 'operator'`. No `gate.py guard` entry. No deferral.

## Verification matrix
No capability is owned by this chunk (`coverage where chunk = …` reads 0 rows), so no ref was written.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started that outlived its entry | — | `gate.py census --comm escher-session --comm cargo --comm rustc --comm seven_guis_native` reads `census: none running` |

## Decisions & corrections
- A guard refusal: `bash-guard.sh` refused a `cd` into the session scratchpad, which would have moved the session cwd. It was re-issued once with absolute paths.
- A sweep hazard: a test module that reaches its parent through `use super::*` also reaches the parent's private `use` imports, so moving items out leaves the tests without those names. This time it was 22 type names, found by the first compile after the move.

## Next
The OPERATOR PASS: entries 13 (hygiene), 14 (push) and 15 (`ci.py conclusion`, including the escher-driver coverage floor of 97 %). Then `/andromeda-wrap-session`.
