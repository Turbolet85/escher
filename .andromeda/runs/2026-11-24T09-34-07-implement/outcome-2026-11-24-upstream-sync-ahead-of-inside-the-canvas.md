# Outcome — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas

**Run:** 2026-11-24T09-34-07-implement · **Outcome:** green (1 iteration) · smoke skipped — no smoke or self-verify entry in the plan; the stand's boot is read headlessly inside `fast` and `stand-repeat`, and the windowed drop source and the two root examples are not run (`evidence/limits.md`).

## Files changed
The merge of `29a439e0563cbed6252c8f1d8ab4ed68afe03968`, staged with `MERGE_HEAD` standing (`git -c rerere.enabled=false merge --no-commit --no-ff`). 28 files: 6 new, 22 modified, every one in research's two lists (`gate.py scope`: `scope: clean — changed 28 · listed 28 · recorded 0`).
- Edited by hand (2): `packages/dioxus-native-dom/src/events.rs` (the conflict's two import blocks; `NativeDragData`'s modifiers through `convert_modifiers`); `packages/blitz-dom/src/layout_reason.rs` (`taffy::CalcHandle::from_ptr`).
- As merged (26): `Cargo.toml`, `Cargo.lock`, `examples/dnd.rs`, `examples/file_drop.rs`, `packages/blitz-dom/src/{document,events/driver,events/mod,layout/construct,layout/mod,layout/replaced,layout/table,node/custom_widget}.rs`, `packages/blitz-paint/src/render.rs`, `packages/blitz-shell/Cargo.toml`, `packages/blitz-shell/src/{application,dnd,lib,window}.rs`, `packages/blitz-traits/src/events.rs`, `packages/blitz-traits/src/events/datatransfer.rs`, `packages/dioxus-native-dom/src/dioxus_document.rs`, `packages/stylo_taffy/src/convert.rs`, `tests/blitz-tests/tests/{all,drag_and_drop,flex_br,pointer_leave}.rs`.
- The staged tree is byte-identical to the trial probe `2cddaea0a` outside `.andromeda/`, `.claude/` and `escher-0.4.0/` (`git diff --stat 2cddaea0a` prints nothing).
- Evidence written: `evidence/{lock,readings,sink,limits}.md` (`founder-forks.md` and `left-out-recheck.md` are the phase's, unchanged).

## Gates
`entries 23 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 4` — entries 1–19 green on the first firing; 20–23 `not run — leg operator`. Trail `.andromeda/runs/2026-11-24T09-34-07-implement/gate-2026-11-24-upstream-sync-ahead-of-inside-the-canvas.json`. No `gate.py guard` entry in the plan. Coverage: dioxus-native-dom `4308/4863 = 88.59 % (floor 83 %)`, the plan's forecast exactly.

## Deviations, with justifications
- **The left-out count is 14, not 15** (plan step 11, acceptance «left out»). The set holds 14 distinct commits. The last sync's lock.md headed its table «14» but listed 13 (`5d4a1f7b2` · `2c76b3f3c` share a row), then added `ca0b97b05` for «15». `evidence/lock.md` lists all 14 with their reasons and names the miscount. No commit was dropped.
- **Research's print attribution**: the 8 `println!` lines are all `examples/dnd.rs`'s (6 live, 2 commented out), not split across the two examples. `evidence/sink.md` records it.
- **Plan step 13's fourth limit, narrowed**: `examples/file_drop.rs` refuses the image and HTML-*file* previews (`img src` / `iframe src` of a `file:` URL). Its `srcdoc` preview of dropped HTML *data* and its text preview read no URL and are unaffected. `evidence/limits.md` §4 states it so.
- Step 1's precondition: one tracked change stood outside `.andromeda/` and the handoff, `escher-0.4.0/working-route.md`, the phase's own pending marker. It is bookkeeping and the merge does not touch it.
- Scope record: none — `gate.py scope` clean, 0 recorded.

## Surfaced / stuck
none.

## Process hygiene
| Process | Started by | Final state |
|---|---|---|
| cargo, rustc, escher-session, seven_guis_native (the legs' builds and test children) | a leg (entries 7–19) | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --comm seven_guis_nati`: `census: none running` |

## Decisions & corrections
- A guard refusal: the Bash guard refused a `cd` into the skill's references directory ("moves the session cwd"). It was re-issued once with absolute paths and ran.
- A guard refusal: the wake guard refused a `ScheduleWakeup` fallback while the gate block ran ("a scheduled wake … outlives /clear"). Nothing was re-issued. The block's completion notification was the wake, as the skill states.
- A sweep hazard: `git grep {token}` with no pathspec reads the `.andromeda/db/log/*.ndjson` journals and the docs, which name the token in prose (104 KB of hits for `element_removed_before_its_mounted_event_is_flushed`). An absence claim about code is read with `-- packages tests examples`.
- An inherited miscount: the standing left-out count «15» carried from the last sync's lock.md is 14 distinct commits (above).
