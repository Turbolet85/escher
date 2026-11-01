# Implement outcome — 2026-11-01-upstream-sync-ui-graph

**Outcome: green + smoke.** 24 of 28 gate entries green, 0 red; the other four are `operator` entries this skill never fires. Nothing is committed. Next: the **operator pass**, entries 25 (hygiene), 26 (the local gate and the push, after the pre-CI commit), 27 (the CI read) and 28 (the per-OS tally), and only then `/andromeda-wrap-session`.

## What landed in the tree
- **The take:** twelve upstream commits by no-commit cherry-pick. Five took a conflict and two a port, each read byte-equal to the trial. With the twelve alone, `git diff 7c2143e4a -- packages/ tests/ wpt/` read empty. The trailers the pre-CI commit owes are in `evidence/take.md`.
- **The panic:** `deallocate_anonymous_block` clears `layout_parent` on a child still naming the block. `key_press_after_container_lost_its_box` is new, and `select_popup`'s row 3 key press is back. Both read red first, at `node/node.rs:1788:34` (the base's `:1775`, moved by the take), and green after. Focus stays on the control, `DocumentMutator::flush` is unedited, and `inert_subtree` is unedited and green.
- **`text_contrast`:** `None` past a `mix-blend-mode` other than `normal`, red before the arm. No stand row moved.
- **Freight:** `matrix_test`'s free-disk step is gone, `Stand checks repeated`'s stands. `embolden` is a leg and the last step of `fast`, with two new pins (Ran 110, as predicted). The orphan comment is gone.
- **Evidence:** `take.md` · `lock.md` · `readings.md` · `sink.md` · `counts.md`. `ci.md` is the operator pass's.

## Files changed
33 by `gate.py scope`: `scope: clean — changed 33 · listed 32 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 754`.

New: `packages/blitz-paint/src/blend.rs`, `packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js`, `packages/blitz-vibey-script/tests/url_search_params.rs`, `tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs`.

Modified: every file of research's list but `docs/capabilities.md`, plus `packages/blitz-test-harness/src/settle.rs`.

## Deviations from the plan, with why
1. **`packages/blitz-test-harness/src/settle.rs`, outside research's lists.** Scope record: `` - `packages/blitz-test-harness/src/settle.rs` · companion · serves `packages/blitz-traits/src/net.rs` · self ``. Step 9's read across the load found `CountedHandler` forwarding `bytes` alone, so `f6edb9701`'s new `NetHandler::error` stopped at the wrapper and a failed stylesheet left a harness document reading `NotSettled(Loads)` for good, on the driver too. The forward has its own unit test, red before and green after. **This is a defect the trial's green gates could not see, and the wrap's harness and driver contracts should say a failed load now ends the wait.**
2. **`docs/capabilities.md`, listed and not edited.** Step 8 asks for a hand read: it states no such limit, no `URLSearchParams`, no `text_contrast` condition, so no claim moves.
3. **The panic's check asserts one thing more than step 5 lists:** a key still moves a select's choice past its dropped block, the behaviour `evidence/panic-reading.md` measured.
4. **The first gate block was stopped mid-run** to make deviation 1, its readings about to go stale. The census read nothing left running, and the whole block ran again. The block above is the second.
5. **`target/tmp/trial-wt-ui` still stands.** The plan removes it "once the take stands", and I read that as after CI's green, since it is the only tree holding the trial's resolutions until then. Its removal falls to the operator pass or the wrap. `target/tmp/trial-wt` is the founder's and is untouched.

## Surfaced for the wrap
- **The plan's Cells figure is stale.** It lists 47,949 bytes; the base and the take both read 47,089, 740 lines, 5 parts, measured through the binary on a scratch clone of the base and on this tree. The take moves nothing. The test plan already says 47,949 predates `cells-grid`'s `id`, with no later reading claimed.
- **A stale claim the take did not move:** `docs/capabilities.md:337` says `stand-repeat` runs fifteen named targets, and `ci-leg.sh` holds seventeen. Its pin does not read that number. Not edited here: it is outside this chunk's moves and has no owner yet.
- **The index holds the picks and the work tree the later edits**, so the pre-CI commit needs `git add -A`. `key_press_after_container_lost_its_box.rs` is intent-to-add.

## Gate readings
- Workspace 219 · 1299 · 0 · 12, against the trial's 218 · 1294 · 0 · 12: +1 line, +3 new check, +1 `contrast`, +1 `settle`.
- `run stand` 237 · 0 · 7 over 51 files, unmoved. `run all` 733 · 0 · 12, +3 the new check and +3 `render_blocking_stylesheet`.
- a11y 14 · 114 · 0 · 0 and stand-repeat 228 · 1155 · 0 · 21, both unmoved, 3 of 3 rounds.
- Audit `advisories ok, licenses ok`, `deny.toml` unmoved. Manifests and lock: no line moved.
- `gate.py guard` entries: none in this plan.
- No source-delta deferral was taken. No capability is claimed: the coverage view holds no row for this chunk, so no matrix write.

## Smoke
Ran as P2 gates, not repeated: entry 24, the windowed `seven_guis_native` under `timeout 10`, green, its log holding `service.name=seven_guis`; entries 18, 19 and 21 through the agent-run contract, whose `status` reads booted, `passed`, 733. The four lean screens and Cells also started, read and stopped through `escher-session`.

## Process hygiene
`gate.py census --comm seven_guis_native --comm escher-session --comm cargo --comm rustc --args ci-leg.sh --args sizes-`: `census: none running`.

| Process | Started by | Final state |
|---|---|---|
| cargo and test binaries, the gate block's entries | this run | terminated |
| cargo, the stopped first gate block | this run | terminated; census read none right after the stop |
| `seven_guis_native`, entry 24 | this run | terminated, by its `timeout 10` |
| `escher-session` hosts, the size readings on both builds | this run | terminated, each by `stop`, exit 0 |

## Decisions & corrections
- A trait default method can hide a take's fix: `NetHandler::error`'s do-nothing default compiled silently through `CountedHandler`, a wrapper no conflict or gate touched (`evidence/sink.md`). Sweep the implementers of a trait a take extends.
- No operator correction, guard or hook refusal, or recurrence of a standing learning this run.
