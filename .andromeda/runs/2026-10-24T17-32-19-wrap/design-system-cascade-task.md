# Task — design-system · 2026-10-24-stand-requirement-sweep

## The session
A test-and-CI-only chunk: no engine, driver, telemetry, stand-source, manifest or lock change, no colour, font, spacing, radius, depth or motion value moved, no new target, crate, env var or listener.
It added a second requirement record to `stand_requirement_sweep` (`WITNESSES_020`, nine 0.2.0 rows, a `Subject::Unit` kind, a check that every witness runs inside `stand-repeat`; 2 → 3 tests).
It asserted three readings that were stated but unchecked: the lint's co-reading (a clipped or off-screen node holding text also reads `truncated`; `stand_lint` 6 → 7) and the text extent reader's three limits, RTL, vertical and transformed text (`text_extent` 3 → 6), all in both layout modes.
It widened the sink captures: `stand_act_spans` reads a paged `snapshot` part 2 (35 → 36 calls per mode), and `host_log` reads the `start --record` clients' own stderr at `trace` with 0 needles (2 → 3 tests). A paged part read through the `escher-session` host's own sink stays unmeasured, a stated limit with a residuals owner.
The `stand-repeat` CI leg now names ten more blitz-tests targets by a literal `--test` list after the `stand_*.rs` glob (the founder's word, 2026-10-24), and its pin in `test_ci_workflows.py` gains the ten words. The shared fixture pages gain `ITEMS` and `many_items`, moved unchanged out of `stand_snapshot_parts.rs`.
Measured: workspace 206 · 1105, stand repeat 198 · 837 · 0 · 21, `run stand` 197 · 0 · 7 over 48 files, a11y 6 · 43 unmoved, coverage floors unmoved; CI green on `18b50a2a2`.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/tests/stand_act_spans.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- examples/seven_guis/tests/host_log.rs
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- Typography, engine text mapping: a group stating how the text extent reader handles RTL text (read as a left-to-right line from the content box), vertical text (read as a horizontal line) or a transformed box (lines read at its client rect, unscaled) keeps that claim, but any clause calling those limits unasserted moves. `text_extent` now asserts all three (3 → 6 tests, both layout modes).
- Typography or depth, transform reading: a group stating that an element's client rect is read where its transforms move it gains a second witness. In `text_extent`'s transformed-box test, the content box equals `get_client_bounding_rect`, which is twice `offset_rect` about the same centre.
- Depth strategy, clipping by `overflow`: a group tying an `overflow: hidden` clip to what the visual lint reads moves only where it says the clipped-text co-reading is asserted by no check. `stand_lint` now asserts that a clipped or off-screen node holding text also reads `truncated` (6 → 7 tests).
- Example fixtures (any category's group), test pages: `stand_lint`'s in-file `reading_page()` is new. It holds `fx-cut-text` (a nowrap line past a 100 × 40 `overflow: hidden` box), `fx-padded-text` (a 60 px paragraph whose `padding-right: 100px` alone passes its box) and `fx-off-text` (wholly right of the 800 px viewport). This touches only a group that inventories test-page values.
- Example fixtures, test pages: `text_extent`'s page gains the paragraphs `#ltr`, `#rtl` (`dir="rtl"`), `#vertical` (`writing-mode: vertical-lr`) and `#scaled` (`transform: scale(2)`), which matters only where a group inventories test-page values.
- Example fixtures, citations: `ITEMS` (400) and `many_items` moved unchanged from `stand_snapshot_parts.rs` into `tests/blitz-tests/tests/fixtures/mod.rs` (64-86). A group citing either at `stand_snapshot_parts.rs`, or listing the shared fixture pages as `password_labelled`, `password_named` and `covered` alone, moves.
- Spacing, example fixtures: the `padding-right: 100px` and 60 px width of `fx-padded-text` are test-page values. A group that inventories untokenized spacing in test pages would gain them, and none in the apps or examples moved.
- CLI surface, output: a group stating what the `escher-session` client writes to its own stderr, or citing the 2026-10-14 one-off reading of a client's stderr, gains a standing check. `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` runs `start crud --record`, `type`, `snapshot`, `snapshot --part 1`, `lint` and `stop` at `RUST_LOG=trace` and reads 0 id, name, sentinel, recording-path and recorded needles. The test count moves 2 → 3.
- CLI surface, output: a group naming the paged `snapshot` answer (`{"text","part","parts"}`) keeps its shape. A clause saying the paged read is unmeasured in a sink capture moves for the in-process capture, where `stand_act_spans` reads part 2 of at least 2, 35 → 36 calls per mode. Through the host's own sink it stays unmeasured.
- Motion, the clock: a group naming the runners of `virtual_clock_engine` moves, because it now also runs in `stand-repeat`. The clock rules themselves are unchanged.
- Depth strategy, hit and clip witnesses: a group naming the runners of `hit_clipped_at_scrolling_box`, `scrolled_box_absolute_position` or `transformed_client_rect` moves, because each now also runs in `stand-repeat`. Its hit-testing rule is unchanged.
- Typography, password mask: a group naming the runners of `password_caret_click` moves, because it now runs in `stand-repeat`.
- Status states and accessibility witnesses: a group naming the runners of `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` or `select_interaction` moves, because each now also runs in `stand-repeat`. The a11y leg's six files and 43 tests are unchanged.
- Gate: any group stating that `stand-repeat` runs only the `stand_*.rs` files and seven_guis' eight moves. The leg now also names ten targets by literal `--test` words (`.github/scripts/ci-leg.sh` 54-58), pinned by `test_stand_repeat_leg_runs_three_rounds` (`test_ci_workflows.py` 359-362).
- Test inventory counts: workspace 206 · 1099 → 206 · 1105; CI macOS and coverage 187 · 1096 → 187 · 1102; windows 187 · 1052 → 187 · 1057; stand repeat 168 · 684 · 0 · 21 → 198 · 837 · 0 · 21; `run stand` 195 → 197 passed over 48 files. A group quoting any of these moves.
- Requirement record: a group citing `stand_requirement_sweep`'s test count (2) or its `WITNESSES` table as the only record moves. A second table, `WITNESSES_020`, holds rows v020-01…09 with 46 witnesses, and `WITNESSES` is byte-unchanged.
- Line citations: any citation into the nine reached files past an added range shifts:
  - `stand_lint.rs`: added 4-7, 110-142 and 226-275;
  - `text_extent.rs`: added 5-8, 10, 25-28 and 119-205;
  - `stand_act_spans.rs`: added 6-8, 243, 685-693 and 727;
  - `stand_requirement_sweep.rs`: added 10-15, 227-229, 232, 538-847, 892-952 and 993-1069;
  - `host_log.rs`: added 11-15 and 40-131;
  - `fixtures/mod.rs`: added 64-86;
  - `stand_snapshot_parts.rs`: lost the moved block and added line 23;
  - `ci-leg.sh`: added 54-58;
  - `test_ci_workflows.py`: added 359-362.
- Session state directories: a group listing the session checks' state directories gains `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`.

## New facts
- (none)

## Untouched
- Brand Identity, Anti-Patterns, Self-Validation Protocol and Design Decisions Log: no recorded intent, and the session records none.
- Color palette: the stand accent `#3a5ce5`, its hover and active steps, the surface, text, semantic and border values and the stand contrast rows. No CSS or colour moved, `stand_contrast` was not reached, and the lint's `contrast` defect is unchanged.
- Typography loading: `fonts-unavailable` for a screenshot or lint with no `sans-serif` family, blitz-dom's `system-fonts` feature for blitz-tests and the WASM bundled-font rule. No font, feature or loading path changed.
- Depth strategy's hit-testing rule (a clip by `overflow` or `contain: paint` stops a hit at the padding box, the root excepted). It is an engine rule, and no engine file was reached.
- Motion: the instant into-view scroll in nested boxes, settle's handling of the animating set, `advance` and the `VirtualClock`, and the load wait moving no clock. Only test runners moved, not those rules.
- Border radius and iconography (browser toolbar icons, favicons, cursor mapping): nothing in the session touches radius, icons or cursors.
- Surfaces desktop-native, mobile-native and web-spa: platforms, toolkits, windows, the urlbar, the back button, capture and the WASM canvas. No app, shell or packaging file changed.
- CLI surface's stdout rule (JSON only, no colour or ESC byte, no terminal test) and its exit statuses `0` to `3`. The command line's code was not reached, and the new `host_log` test reads stderr alone.
- The NOT YET MEASURED absences (project-wide colour token set, type scale, spacing scale, elevation scale, radius scale, CLI navigation and terminal width, web-spa routing): the session measures none of them.
