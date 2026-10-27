# Readings — residual line 36's merge surfaces against the take

Read 2026-10-27 on the take's working tree against the base `de85792f5` (`git diff --stat de85792f5 -- {files}`),
run `.andromeda/runs/2026-10-27T09-58-03-implement/`. Every row's owner is `.andromeda/residuals.md:36`, which reads:

> 2026-10-24-upstream-sync-polish-and-ship · absorbed:v030-03 · the merge surfaces of 2026-10-18-engine-debts' seven
> engine fixes, 2026-10-18-one-virtual-clock's document clock and 2026-10-20-select-by-id's select model — each flagged
> upstreamable with no upstream pull request, re-read per row at the take read at `f9cbb2756` with nothing sent upstream
> (escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/readings.md; the surfaces by file in
> escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md and in architecture's established-decisions
> detail) — re-read at the next upstream sync; placed here by the operator's word on that chunk's P5 card, row 2
> (inputs#I5) (target: next)

No upstream pull request is opened (the founder's standing hold, 2026-10-07/2026-10-10).

## The engine-debts fixes (2026-10-18-engine-debts) — owner `.andromeda/residuals.md:36`

- `packages/blitz-dom/src/{node/node.rs, accessibility.rs, document.rs, events/pointer.rs, events/keyboard.rs,
  scrolling.rs, layout/writing_mode.rs}`: `git diff --stat` empty — not reached by the take.
- `packages/blitz-dom/src/node/element.rs`: reached by `53e9d81c6` alone — `ElementData::new` calls
  `data.flush_open_state()` before returning (hunk `@@ -485,6 +485,8 @@`) and the new `pub fn flush_open_state`
  (`@@ -538,6 +540,14 @@`) sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`. `is_disabled` and
  `flush_is_focussable` are untouched (no hunk names them).
- Their checks green in both layout modes (gate entry 14, 23 targets, 125 passed, 0 failed): `hit_clipped_at_scrolling_box`,
  `inert_subtree`, `transformed_client_rect`, `scrolled_box_absolute_position`, `scrolled_box_client_rect`,
  `password_caret_click`, `scroll_into_view_nested`, `hit_test_dropped_nodes`, `accessibility_roles`,
  `accessibility_hidden`, `focusability_updates`.

## The document's clock (2026-10-18-one-virtual-clock) — owner `.andromeda/residuals.md:36`

- `packages/blitz-dom/src/{config.rs, clock.rs, document.rs, resolve.rs, iframe.rs, scrolling.rs, events/pointer.rs}`:
  `git diff --stat` empty — not reached.
- Wall-clock reads in blitz-dom outside `clock.rs`: 0 (gate entry 8, exit 1, last line 0). The fixed-sha census of the
  take's added engine lines holds no `Instant::now` or `SystemTime::now` (entry 7, its two hits named in `sink.md`).
- `virtual_clock_engine` green (entry 14); blitz-dom's lib tests 106 passed (entry 13), the clock and iframe clock
  tests among them.

## The select model (2026-10-20-select-by-id) — owner `.andromeda/residuals.md:36`

- `packages/blitz-dom/src/mutator.rs` (+8): `set_attribute` holds our range re-sanitize arm (`min` · `max` · `step` →
  `resanitize_range_input`) and, after it, upstream's `open` arm (`flush_open_state`) as two independent `if`s (the
  one-hunk resolution, `take.md` §3); `remove_attribute` gains the `open` arm after the link flush (picked clean). The
  changed-set inserts, the range re-seed on `value` and the select arms are untouched (no other hunk).
- `packages/blitz-dom/src/node/element.rs` (+10): `flush_open_state`, as above.
- `packages/blitz-dom/src/stylo.rs` (+1 −1): `NonTSPseudoClass::Open` reads `ElementState::OPEN` in place of `false`.
- `packages/blitz-vibey-script/src/runtime.rs`: split by `e6f4615ec` (151 lines, upstream's side whole); our
  select-by-id comment carried to `packages/blitz-vibey-script/src/runtime/events.rs:171-173` (entries 2, 3, 4: 0
  files differing from upstream but `events.rs`, `events.rs` +2 −2 against `efdc5472a`, the comment found once).
- `packages/blitz-dom/src/layout/construct.rs` (+12 −15): `7fc9ee3af`'s IME enable in `create_text_editor` (reads
  `is_focussed()` before the editor is created, then `shell_provider.set_ime_enabled(true)`) and `2963b65a2`'s pseudo
  vote in `classify_flow_children` (every `::before`/`::after` votes; the `display: contents` filter dropped).
  `RangeInputData`'s seeding untouched (0 diff lines name it).
- `select_interaction` and `stand_act_select` green (entry 14, and the workspace leg inside `fast`);
  `range_input_interaction` green (entry 14).

## The transformed scrolling box (for «Engine debts», v030-04)

The recorded `visible_region` limit stands unmoved: the take leaves `scrolling.rs` and `layout/writing_mode.rs`
untouched (both `git diff --stat` empty). None of `hit_inner`, `physical_unrounded_geometry`, `absolute_position`,
`scroll_into_view` or `visible_region` is edited; blitz-vibey-script's `scroll_into_view` caller moves from
`dom/element.rs` into `dom/element/scrolling.rs` with the split, calling the same engine method.
