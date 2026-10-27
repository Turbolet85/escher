# Task — a11y · 2026-10-27-upstream-sync-engine-debts

## The chunk
Takes in upstream blitz's new work at the measured sha `ebacd68bd`, by the take the founder's delegate chooses at P4 (cherry-picks onto the registry pins, those plus winit beta.4, a whole merge, or nothing), from 27 candidate commits.
Resolves the predicted conflicts in nine files keeping both sides' intent: the changed set, the clock invariant, the select and range models, the hit stops and the scrolled-box bounds stay as they are.
Carries upstream's new and edited test files as one target per file, and re-runs every escher proof after the take: the fast and doc legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit and fork CI.
Re-proves the a11y target SC 2.1.1 · 1.4.3 · 2.4.3 by its standing checks (`stand_keyboard`, `stand_contrast`, the `accessibility_*` files, `inert_subtree`) and the a11y leg; a number a take moves is re-measured and recorded, never loosened.
Re-reads per row the merge surfaces of the fork's engine changes, the per-OS test tallies and the telemetry sink allowlist against the take; it fixes no engine debt and adds no crate, port, env var or listener.

## Reached files
- Cargo.lock · conflict surface
- Cargo.toml · merge surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-vibey-script/src/runtime.rs · conflict surface
- wpt/WPT_COMMIT · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/events/pointer.rs · merge surface
- packages/blitz-dom/src/events/driver.rs · merge surface
- packages/blitz-dom/src/node/text.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-traits/src/events.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface
- tests/blitz-tests/tests/clipboard.rs · merge surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/inline_box_scrollable_overflow.rs · merge surface
- tests/blitz-tests/tests/display_contents.rs · merge surface
- tests/blitz-tests/tests/pre_overflow_scroll.rs · merge surface
- .github/scripts/test_blitz_tests_targets.py · edits the pinned blitz-tests target list
- .github/scripts/ci-leg.sh · re-reads the fast, doc, a11y and audit legs
- .github/workflows/ci.yml · re-reads the a11y job and every job on the pushed commit
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads stand_keyboard
- tests/blitz-tests/tests/stand_contrast.rs · re-reads stand_contrast
- tests/blitz-tests/tests/inert_subtree.rs · re-reads inert_subtree
- tests/blitz-tests/tests/accessibility_states.rs · re-reads accessibility_states
- tests/blitz-tests/tests/accessibility_label.rs · re-reads accessibility_label
- tests/blitz-tests/tests/accessibility_description.rs · re-reads accessibility_description
- tests/blitz-tests/tests/accessibility_merged_states.rs · re-reads accessibility_merged_states
- tests/blitz-tests/tests/accessibility_active_descendant_unread.rs · re-reads accessibility_active_descendant_unread
- tests/blitz-tests/tests/accessibility_hidden.rs · re-reads accessibility_hidden
- tests/blitz-tests/tests/accessibility_roles.rs · re-reads accessibility_roles
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist

## Needs
- The a11y tier-0 target SC 2.1.1 · 1.4.3 · 2.4.3: after the take it is re-proven by the standing checks and by the a11y leg's fourteen files reading green, with the a11y job settled on the pushed commit.
- Focusability and focus order: conflict resolutions in `node/node.rs` and `mutator.rs` keep an inert node and a disabled form control unfocusable and the tabindex and native-focusable element rules unchanged, so `stand_keyboard`'s Tab order tables (`tab_order`, `ENABLED_TAB_ORDER`) read as recorded.
- Keyboard handling: Enter and Space activation, the range input's keys and the single-choice select's keys and typeahead survive the merges into `events/driver.rs`, `events/pointer.rs` and `mutator.rs`'s select and range arms (reached by `0980cce5f`, `527394724`, `ebacd68bd`).
- Focus restoration on click: a button or select click focusing it, a range click moving no value, and the double-click window read on the document's clock, through `events/pointer.rs`, which `0980cce5f` edits.
- Hit testing and visibility: `ebacd68bd` hits an element with no computed styles yet in `node/node.rs`; the resolution keeps the hit passing through an inert element, skipping a dropped node and stopping at a clipping or paint-contained box.
- Hidden content and tree exclusion: `display: none`, `visibility: hidden` and `inert` exclude a subtree and a select's option stays; `53e9d81c6`'s `<details>` `:open` pseudo-class (`stylo.rs`, `node/element.rs`, `mutator.rs`) changes which details content computes hidden, so what the tree keeps there.
- The accessibility tree lifecycle: the changed set (what marks it and what does not, drained by `take_changed_nodes`) survives the `mutator.rs` conflict, and the shell poll that drains it and rebuilds the platform tree survives `6587e7e79`'s redraw routing in `blitz-shell/src/{lib,window}.rs`.
- The builder's role, state, name and description reads (`aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid`, the merged upstream reads, the label rules) are re-asserted by the `accessibility_*` checks after the take.
- Stable ids as AccessKit `author_id` through `DioxusDocument::accessibility_tree`'s override: `7fc9ee3af` edits `dioxus_document.rs` and `286874cc1` (if taken) replaces upstream's side of the bridge; the tree the shell hands the adapter still equals the harness's tree (`stand_accessibility_ids`).
- The snapshot model consumes the tree for role, name, focus and the three states: a bridge or builder change must not open a second role or name mapping.
- Input method: `7fc9ee3af` enables IME when a focused text input is constructed (`layout/construct.rs`, `dioxus_document.rs`), reaching the plan's IME surface and focus on text inputs.
- Clipboard keys in text inputs: `527394724` moves the shell to winit's clipboard API and drops the `clipboard` feature from blitz-shell's and dioxus-native's defaults, so what keyboard copy and paste do in a focused field under the default features.
- The platform adapter wiring in blitz-shell: winit beta.4 (`a09f46792`, if taken) and the redraw routing must keep the adapter created before the window is first shown.
- The Dioxus framework exposure: a falsy value of one of the bridge's boolean attributes (`disabled`, `hidden`, `inert` among them) removes the attribute; a Dioxus 0.8 take (`286874cc1`) reaches that bridge and retires `keyboard_compat.rs`, its coverage-leg ignore lines dropping with it.
- The `accessibility` feature's reach: it reaches seven_guis through escher-driver with the Dioxus crates taken `default-features = false`; a manifest change under the take must keep it on.
- Contrast: `stand_contrast`'s three tables (17 control rows, 14 `TEXT_ROWS`, 18 `STATE_ROWS`) and its painted-pixel witness re-assert in both layout modes; a Stylo, Taffy or paint move that shifts a value is re-measured and its record moved, never loosened.
- The visual lint's `contrast` reading stays on `Harness::text_contrast` with `stand_contrast`'s exempt rule, re-proven by the stand checks.
- The CRUD list's keyboard model and the unread `aria-activedescendant`, re-asserted by `stand_keyboard` and `accessibility_active_descendant_unread`.
- The flight booker's invalid dates and CRUD's selected row stated by `aria-*` alone, re-asserted through the tree and the snapshot by the standing checks.
- Scroll origins: `0980cce5f` reverses scroll origins through Taffy's `ScrollableOverflow` in `scrolling.rs`, `writing_mode.rs` and `node/node.rs`; the scrolled-box bounds and visible-region readings the keyboard and snapshot checks read stay as recorded or are re-measured.
- The a11y leg's file list and the stand-repeat leg's eight a11y files in §9 stay as recorded unless the take carries a new accessibility test file, which would move the list and its count.

## Standing conventions
- S1 · an accessibility check reads the built tree after resolve, one stated table per attribute with its row count asserted, in both layout modes · §3 A11y Assertion Harness Contract
- S2 · a test file that asserts accessibility joins the a11y leg's list in `ci-leg.sh`, and a stand or repeat-bound one the stand-repeat list, the counts in §9 moving with it · .andromeda/a11y-plan-detail/ci-integration.md
- S3 · the roles and names a check expects are stated once, in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S4 · a keyboard check drives keys through the harness's `press` / `press_with` and reads the focused node back per step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · a stand colour or contrast change moves its row in the recorded contrast tables · .andromeda/a11y-plan-detail/visual-design-verification.md

## Not needed
- Focus-ring, target-size and typography tokens: not yet measured, and the take adds no stand style.
- Motion, `prefers-reduced-motion`, `prefers-color-scheme`, zoom and user-preference handling: the take adds no media rule or animation to the stand.
- Language and direction (`html lang` in fixtures) and text alternatives in `examples/assets` fixtures: no fixture is edited.
- Live regions, heading hierarchy and a screen-reader test spec: not built, and nothing in the delta builds them.
- Cognitive orientation and status cues, timeout extensions, plain language and accessible authentication: not recorded and not reached.
- The a11y violation JSON schema: not defined; the sink allowlist re-read is the observability plan's.
- SLO budgets, the anti-patterns list and the decisions log: no recorded intent.
- The windowed assistive-technology witness at the platform adapter, carried past 0.2.0: an active assistive technology this version does not reach.
- HTML's focus fixup for a node made inert while focused, and the native checkbox or radio toggled reading: engine debts owned by residual lines for «Engine debts» (v030-04); this chunk fixes no debt.
- The load page's `load-fetch` keyboard and contrast gap: owned by its residual line, untouched by the take.
- The observed-absent slice searches of the scope summary and assertion harness: historical survey records, not rules the take acts on.
