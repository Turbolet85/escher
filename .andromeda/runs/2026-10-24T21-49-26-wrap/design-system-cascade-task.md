# Task — design-system · 2026-10-24-a11y-re-read

## The session
The `a11y` CI leg (`.github/scripts/ci-leg.sh:39`) widened from six to fourteen `--test` targets, still one `cargo test --workspace --locked` line: the six it ran (accessibility_hidden, accessibility_roles, focusability_updates, stand_keyboard, stand_contrast, stand_accessibility_ids) kept in order, then eight joining (accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction, stand_act_select), on the founder's answer to the P4 fork.
Its tally moved 6 result lines · 43 passed → 14 · 84 · 0 failed · 0 ignored, locally and on the CI "Accessibility (a11y) tests" job (CI#40125511019 → CI#40134371519); every other job's tally unmoved.
The pin `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`'s a11y row now names the fourteen, one `assertIn` each, in no order (`test_ci_workflows.py:298-299` → `:298-302`); lines after 299 in that file moved +3.
The stand's Tab order, Enter/Space activation and contrast were re-read after 0.2.0's engine changes and found unmoved: stand_keyboard 14, stand_contrast 5, stand_accessibility_ids 8 green; `tab_order`, `ENABLED_TAB_ORDER`, `controls`, `TEXT_ROWS`, `STATE_ROWS` unedited, nothing re-baselined.
No engine, stand, colour, CSS, table, `ci.yml`, manifest, lock or Rust source file changed; nothing retired.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/

## Touches
- Any statement of the `a11y` leg's selection as six files, or of its tally as 43 tests over 6 targets (6 · 43), is now stale: fourteen files, 84 tests.
- Any statement that stand_contrast runs in the `a11y` leg stays true (still among the fourteen, its count 5 unmoved).
- Any statement that a file among the eight joiners (accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction, stand_act_select) runs outside the `a11y` leg, or not in it, is now stale; each still runs in every leg it ran in before.
- Any statement that focus_redraw, focus_within, accessibility_names or hit_test_dropped_nodes is outside the `a11y` leg stays true, while a phrase counting the leg as six is stale.
- The stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) and its SC 1.4.3 readings (white on the accent 5.47) re-asserted unmoved by stand_contrast; no colour value moved.
- The stand_contrast record rows (`TEXT_ROWS`, `STATE_ROWS`) re-read unedited — any count or reading of those rows stays as it stands.
- The stand's Tab order tables (`tab_order`, `ENABLED_TAB_ORDER`) and Enter/Space activation on each stand button re-read unmoved through stand_keyboard.
- The select's interaction tests (select_interaction, stand_act_select) and the inert subtree check now read inside the `a11y` leg — any status-state or focus text naming their legs is touched.
- Any line citation into `.github/scripts/test_ci_workflows.py` past line 299 moved +3; the a11y tuple's range is now `:298-302`; `ci-leg.sh` line count unchanged.
- Any statement that the pin asserts the leg's files in order, or names six, is stale.
- The CI a11y job's per-job tally, if cited with a run id, is owed CI#40134371519's 14 · 84 · 0 · 0.
- Three rows in this doc matched the sweep's `a11y-tally` pattern by accident ("the 17 listed s06 files": `design-system-detail/border-radius.md:40`, `depth-strategy.md:49`, `motion.md:46`) — no-change, none about the a11y leg.
- Workspace tally 206 · 1105 · 0 · 12, `stand-repeat` 198 · 837 · 0 · 21 and `run stand` 197 passed · 7 ignored re-read unmoved — any such count stays.

## New facts
- (none)

## Untouched
- Typography: font loading, the `fonts-unavailable` refusal, the `system-fonts` feature and the WASM font context — no font or text path changed.
- Spacing values and the absent spacing scale — no CSS or layout value changed.
- Depth strategy: the hit-testing stop at a paint-clipping box's padding box — no engine hit code changed.
- Border radius values and the absent radius scale — no CSS changed.
- Motion: the into-view scroll, settle's animating set, the `VirtualClock`, `advance` and the load wait — no clock or settle code changed.
- Iconography and the engine cursor — no icon, asset or cursor code changed.
- Surface desktop-native, mobile-native and web-spa tokens, components and platform notes — no shell, app or packaging file changed.
- Surface cli: `escher-session`'s stdout JSON, its exit statuses and its `mcp` role — no driver or binary code changed.
- The project-wide colour token set's absence and the engine-painted defaults — no stylesheet or paint code changed.
- The `ci.yml` "Accessibility (a11y) tests" job's tier, `needs`, restore-only cache and failure-only upload — byte-identical.
