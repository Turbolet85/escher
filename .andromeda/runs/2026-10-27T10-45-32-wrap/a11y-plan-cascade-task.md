# Task — a11y-plan · 2026-10-27-upstream-sync-engine-debts

## The session
Took eleven upstream DioxusLabs/blitz commits by no-commit cherry-pick on the founder's word, upstream main read `f9cbb2756` → `ebacd68bd`, merge base `23b9b26a9` unmoved, sixteen left out; no dependency, config or escher-driver / escher-telemetry change.
Engine behaviour changed for every document, ratified for this take: `:open` now matches a `details` or `dialog` carrying `open`; a text input already focused when its editor is built enables IME; `::before`/`::after` vote in flow classification; the windowed shell routes redraw requests through its event loop.
blitz-vibey-script: its runtime split into `runtime/` modules with nine JS bootstrap scripts, its element bindings into `dom/element/` modules (moves, nothing removed); CSS properties become accessors on `CSSStyleDeclaration`; a `file:` script load now allowed only for a `file:` document (a narrowing).
Tests added: blitz-tests target `pseudo_element_flow_classification` (3), blitz-vibey-script `tests/dom.rs` 27 → 30, dioxus-native-dom lib 81 → 82 (`input_focused_on_mount_enables_ime`); the a11y leg (14 · 85 · 0 · 0), `run stand` (205 · 0 · 7 over 50 files) and stand-repeat unmoved.
Residual lines 35 and 36 (the left-out upstream delta; the merge surfaces of the engine-debts fixes, the clock and the select model) get a new owner, Epoch 3's «Upstream sync ahead of the UI graph»; F2's PROVISIONAL stand-row marks ratified by the founder.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- wpt/WPT_COMMIT · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/ · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface
- tests/blitz-tests/tests/all.rs · merge surface

## Touches
- §1 scope summary's upstream take reading (`a11y-plan-detail/a11y-scope-summary.md:18`): upstream main read `f9cbb2756` → `ebacd68bd`, merge base `23b9b26a9` unmoved, eleven taken, sixteen left out.
- §2's reading of the 2026-10-17 merge at `23b9b26a9` (`a11y-plan-detail/a11y-strategy.md:22`): re-read, unmoved.
- §7 Input method (IME) — the enable reading at `a11y-plan-detail/screen-reader-support.md:98` and `a11y-plan-detail/keyboard-navigation.md:23`: a text input focused before its editor exists now enables IME at the editor's construction (`create_text_editor`, `layout/construct.rs:313-317`); the IME event plumbing at `screen-reader-support.md:99`, `keyboard-navigation.md:48`, `:61` and the body's `a11y-plan.md:211` re-read, unmoved.
- §5 keyboard handling's `summary` Enter/Space activation: unchanged, but the `details` it toggles now carries `ElementState::OPEN`, so `:open` matches it.
- Disabled-ness and focusability rules whose code sits in `node/element.rs`: re-read unmoved (`is_disabled`, `flush_is_focussable` unchanged), only their line numbers shift.
- Line citations into `packages/blitz-dom/src/node/element.rs` past 487 (+2 at 488-489, +8 at 543-550, `flush_open_state` new at 543-549).
- Line citations into `packages/blitz-dom/src/mutator.rs` past 367 (+4 at 368-371, +4 at 520-523; the `open` arm after the range re-sanitize arm in `set_attribute`).
- Line citations into `packages/blitz-dom/src/stylo.rs` past 509 (+1 at 510, `NonTSPseudoClass::Open`) and `packages/blitz-dom/src/layout/construct.rs` past 312 (+5 at 313-317, +1 at 860, +6 at 867-872).
- Line citations into `packages/blitz-shell/src/lib.rs` (+10 at 90, 93-98, 104-106) and `packages/blitz-shell/src/window.rs` past 192 (+2 at 193-194) — the shell's accessibility refresh in `View::poll` re-read, unmoved; only the redraw route moved.
- Line citations into `packages/dioxus-native-dom/src/dioxus_document.rs` past 724 (+35 at 725-759, one test, 0 lines removed) — the `accessibility_tree` override and `author_id` carriage unchanged.
- Citations into blitz-vibey-script `runtime.rs` and `dom/element.rs`: the cited code moved to `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` and `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs` (`runtime.rs` keeps 151 lines) — e.g. script `disabled` · `hidden` · `autofocus` · `focus` · `blur` bindings now in `dom/element/attributes.rs` and `dom/element/form_controls.rs`, `scroll_into_view`'s script caller in `dom/element/scrolling.rs:230-268`.
- The select model's script-side note: escher's select-by-id comment (Blitz generates `change` for a `select` alone) now at `runtime/events.rs:171-173`, upstream's comment replaced.
- Test inventory: blitz-vibey-script `tests/dom.rs` 27 → 30 `#[test]`, the file gaining lines at 455-488 and 859-950 (citations into it past 454 shift).
- Test inventory: dioxus-native-dom lib 81 → 82 unit tests.
- Test inventory: blitz-tests directory 147 → 148 `.rs` files, 146 → 147 test targets; workspace tally 208 · 1117 · 0 · 12 → 209 · 1125 · 0 · 12; `run all` 630 → 633 passed.
- §9 CI integration: the `a11y` leg's fourteen files unchanged and its tally 14 · 85 · 0 · 0 unmoved; the new target runs in the workspace leg and `run all`, not in the `a11y` leg or `run stand`; stand-repeat 204 · 864 · 0 · 21 unmoved; fork CI green 17/17 on `e795a088c`.
- §5 keyboard harness OPEN on the load page's `load-fetch` Button and other rows naming `boot_load` · `LOAD_ANSWER_DELAY` · `type-select`: re-read by the stand-lines move, unmoved.
- `.andromeda/residuals.md` owner text for line 36 (merge surfaces of the engine-debts fixes, the clock and the select model) and line 35 (left-out upstream delta, +6 commits): owner now Epoch 3's «Upstream sync ahead of the UI graph», wherever a11y-plan names that owner.
- The founder's decision pattern «An engine behaviour change for every document», ratified for this take (`:open`, IME-on-construct, the pseudo vote, the redraw route, the CSS accessors, the `file:` refusal) — where a11y-plan records engine changes as ratified.
- a11y-plan's sidecar entry at `a11y-plan-amendments.md:500` (F2, PROVISIONAL): kept as written; the founder's ratification lands as a new sidecar entry.
- Indirect, through the CSS accessors: a script setting `element.style.display` or `visibility` now writes inline style, which reaches §4 / §6's hidden-content exclusion from the tree.

## New facts
- F1 · keyboard-navigation · A `details` or `dialog` carrying `open` holds `ElementState::OPEN`, set by `ElementData::flush_open_state` at element construction and on every `open` attribute write or removal through `DocumentMutator`, so `:open` matches it (it never did); `summary` activation toggling a `details` therefore flips `:open`; no test asserts `:open`.
- F2 · visual-design-verification · Author styles keyed on `:open` now apply to an open `details` or `dialog` in every document — an engine change ratified by the founder for this take; unasserted, `details_element` and the a11y leg green.
- F3 · screen-reader-support · A text input already focused when its text editor is constructed calls `shell_provider.set_ime_enabled(true)` at construction (upstream `7fc9ee3af`), pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime`.
- F4 · keyboard-navigation · Script exposure: CSS properties are accessors on `CSSStyleDeclaration` (names from Stylo's property list, `css_property_names` and `installStyleProperties`), pinned by `tests/dom.rs` `style_property_accessors` — a realization on the registered script-to-DOM crossing.

## Untouched
- The ARIA state reads (`aria-selected` · `aria-pressed` · `aria-checked` · `aria-invalid`), upstream's six name, description and state reads and the role derivation and HTML-AAM landmark mapping in blitz-dom's accessibility builder — no line of it changed.
- `stand_contrast`'s three tables (17 control rows, 14 `TEXT_ROWS`, 18 `STATE_ROWS`), the exempt rule, the visual lint's `contrast` reading and the SC 1.4.3 record — no stand figure moved.
- `stand_keyboard`'s Tab orders (`tab_order`, `ENABLED_TAB_ORDER`), the CRUD list keys and the unread `aria-activedescendant` — unchanged, `run stand` unmoved.
- The range input's keyboard and pointer model (`node/range.rs`) and the engine's select rules and typeahead (`node/select.rs`) — re-read, unchanged.
- Keyboard activation (`handle_key_up`, `dispatch_activation_click`) and a button click's focus (`click_button`) — unchanged.
- `inert` handling (skipped by Tab, passed by a hit, out of the tree) and the missing HTML focus fixup residual — unchanged.
- Hit testing (`hit_inner`), `visible_region` and `scroll_into_view` — re-read, unchanged.
- Password and file-input masking (`MASKED_VALUE`) and the actionable-key rule (`unkeyed_actionable`) — unchanged.
- The platform adapter (accesskit_xplat) and its windowed AT witness residual reading `re-carried:0.2.0` — not reached.
- §8 cognitive error recovery (the flight booker's `aria-invalid` dates) — unchanged.
- The double-click window and the document's clock — no time read added.
- Every NOT YET MEASURED block and observed-absent search record — no new measurement.
