# Task — a11y-plan · 2026-10-24-stand-requirement-sweep

## The session
A test-and-CI-only chunk: no engine, driver, telemetry, stand-source, manifest or lock change, no new target, env var, listener or crate; nothing retired.
The `stand-repeat` CI leg now runs, after its `tests/blitz-tests/tests/stand_*.rs` glob and seven_guis' eight targets, ten named blitz-tests targets by a fixed `--test` list in `.github/scripts/ci-leg.sh` (the founder's word «Список в скрипте (Рекомендую)», 2026-10-24), three of them a11y-plan's own witnesses: `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread`; the other seven are `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `virtual_clock_engine`, `select_interaction` and `text_extent`.
`stand_requirement_sweep` gains a 0.2.0 witness record (`WITNESSES_020`, nine rows v020-01…09) and a check that every row's witness runs inside `stand-repeat` (2 → 3 tests); `stand_lint` asserts that a clipped or off-screen node holding text also reads `truncated` (6 → 7); `text_extent` asserts its three stated limits — RTL and vertical text read as an LTR horizontal line, a transformed box read at its client rect (3 → 6).
`stand_act_spans` reads a paged snapshot part in its sink capture (35 → 36 calls per mode), the shared fixture pages gain `many_items` and `ITEMS`, and `host_log` reads the `start --record` clients' own stderr at `trace` with 0 needles (2 → 3).
Measured: the a11y leg unmoved at 6 files · 43 tests; `stand-repeat` 198 · 837 · 0 · 21 (was 168 · 684 · 0 · 21); workspace 206 · 1105; `run stand` 197 · 0 · 7 over 48 files; coverage floors unmoved; CI#40114977936 green, 17/17.

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
- §3 row on `accessibility_states` (`a11y-plan.md:53`): its runner clause "runs in the workspace test leg and per package, not in the `a11y` leg" now also gains `stand-repeat` (three rounds, zero-failure budget); still outside the a11y leg's six files.
- §3 row on `accessibility_merged_states`, `inert_subtree` and `accessibility_active_descendant_unread` (`a11y-plan.md:55`): `inert_subtree` and `accessibility_active_descendant_unread` now also run in `stand-repeat`; `accessibility_merged_states` is not among the ten and keeps its workspace-and-per-package runners alone — the row's shared runner clause splits.
- Any category-file row that places the runners of `accessibility_states`, `inert_subtree` or `accessibility_active_descendant_unread` (a test inventory, a harness or keyboard-harness row, a visibility row citing `inert_subtree`) — the same `stand-repeat` addition.
- §9 CI Integration (`a11y-plan.md:235`, `a11y-plan-detail/ci-integration.md:8`): re-read; the a11y leg's six files and its job are unmoved, and its three `stand_*` files still run in `stand-repeat` by prefix — but that leg now also runs three more a11y files by name, so a statement of which a11y files `stand-repeat` runs is now incomplete.
- The a11y leg's tally, 6 files · 43 tests, measured unmoved at this chunk — a reading row citing it holds.
- A count or reading of `stand-repeat` cited anywhere in a11y-plan (168 · 684 · 0 · 21 → 198 · 837 · 0 · 21; `run stand` 195 → 197 · 0 · 7 over 48 files; the `stand_*.rs` glob still 48 files).
- A row naming `stand-repeat`'s selection as the `stand_*` files (and seven_guis' process checks) alone — now the glob, the ten named targets and seven_guis' eight.
- A row stating that a non-`stand_*` a11y test is reached only by the workspace test leg, per package or `run all` / `run {name}`: `run stand` still keeps its prefix rule, so the ten reach the agent harness through `run all` / `run {name}` alone — only the CI `stand-repeat` half moves.
- A row stating the `test_ci_workflows.py` pin of `stand-repeat` (`test_stand_repeat_leg_runs_three_rounds`): it now asserts the ten named words too.
- A row on escher-driver's visual lint (§6, `Session::lint`, the `lint` verb): `stand_lint` 6 → 7 tests, and the co-reading "a clipped or off-screen node holding text also reads `truncated`" is now asserted by `stand_lint` (fixture page `reading_page()`), no longer asserted by no check; its `contrast` reading and exempt rule unmoved.
- A row on `BaseDocument::text_extent` or its stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box at its client rect), including any §7 Language and direction row that names how escher's readers treat `dir="rtl"` or `writing-mode`: the limits stand as limits and are now asserted by `text_extent` (3 → 6 tests, both layout modes).
- A row on the stand's or `stand_requirement_sweep`'s requirement-witness record: a second table `WITNESSES_020` (nine 0.2.0 rows, 46 witnesses, `Subject::Unit` for crate unit tests) beside the byte-unchanged `WITNESSES`.
- A row on `password_caret_click` (a click places a password caret by its bullets, §7): its test now also runs in `stand-repeat`; the behaviour is unmoved.
- A row on `select_interaction` (§5 single-choice select keys): its test now also runs in `stand-repeat`; the behaviour is unmoved.
- A row on `hit_clipped_at_scrolling_box`, `scrolled_box_absolute_position` or `transformed_client_rect` (§6 Visibility and hit-testing): those tests now also run in `stand-repeat`; the behaviour is unmoved.
- A row on `virtual_clock_engine` (§5's double click read on the document's clock): that test now also runs in `stand-repeat`; the behaviour is unmoved.
- Any line citation into a reached file: `stand_lint.rs` (lines added at 4-7, 110-142, 226-275), `text_extent.rs` (5-8, 10, 25-28, 119-205), `stand_requirement_sweep.rs` (10-15, 227-229, 232, 538-847, 892-952, 993-1069), `fixtures/mod.rs` (64-86, `ITEMS` and `many_items` moved in), `stand_snapshot_parts.rs` (both moved out, 23 added), `stand_act_spans.rs` (6-8, 243, 685-693, 727), `host_log.rs` (11-15, 40-131), `ci-leg.sh` (54-58) and `test_ci_workflows.py` (359-362) — later lines shift.
- A row listing the shared fixture pages (`password_labelled`, `password_named`, `covered`): `many_items` now joins them.
- The decision record: the founder's word of 2026-10-24 ratifies `stand-repeat`'s named `--test` list, the files keeping their names (passed over: renaming them `stand_*`).

## New facts
- F1 · ci-integration · `stand-repeat` runs three a11y witnesses outside the `stand_*` prefix by a fixed `--test` list in `ci-leg.sh` — `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` — three rounds under the zero-failure budget, on the founder's word «Список в скрипте (Рекомендую)» (2026-10-24), while the a11y leg's six files stay unchanged.
- F2 · ci-integration · `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` reads `ci-leg.sh` and asserts every 0.2.0 integration witness (the a11y ones among them) runs inside `stand-repeat`, by prefix or by the named list, so dropping one of the three from the list turns that check red.

## Untouched
- §2 engine rules — the accessibility tree's build and refresh, the changed set, role derivation and the ARIA state reads in `packages/blitz-dom/src/accessibility.rs` — no engine file changed.
- §4 ARIA Patterns & Roles — the HTML-to-AccessKit mapping, the four state attributes, `role="switch"` and hidden-content rules; no mapping changed.
- §5 engine keyboard and focus rules — focusability, disabled-ness, button-click focus, Enter and Space activation, range and select keys, the bridge's falsy boolean attributes; behaviour unmoved, only some tests' CI runner changed.
- §5's `stand_keyboard` Tab-order tables (`tab_order`, `ENABLED_TAB_ORDER`) and the CRUD list's keys — no stand source or table changed.
- §6 `Harness::text_contrast`, `contrast_ratio` and `stand_contrast`'s three tables (18 control rows, 14 `TEXT_ROWS`, 19 `STATE_ROWS`) and the SC 1.4.3 reading — no harness, stand colour or contrast table changed.
- §7 snapshot model, `author_id`, the actionable-key rule, password and file-input masking, and the stand's `aria-*` state carriers — no dioxus-native-dom, driver or stand-task change.
- §7 Platform adapter (`accesskit_xplat`) and the windowed AT witness residual — untouched; no platform adapter enters the stand graph (measured 0).
- §3 harness helpers `assert_role` and `unknown_tags`, and the `accessibility_label` (13) and `accessibility_description` (3) test counts — their files not reached, counts unmoved.
- §8 Cognitive Accessibility — the flight booker's `aria-invalid` date cue, unmoved.
- §3's NOT YET MEASURED block on the a11y violation schema and the obs log format — the `host_log` and `stand_act_spans` additions are obs and security readings of id-free stderr, not an a11y violation schema.
- §10, §11 and §12 — no recorded intent; the session adds none.
