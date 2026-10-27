proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 27
    section: "## 5. Keyboard Navigation"
    change: "Add after line 27: - A `details` or `dialog` carrying `open` holds `ElementState::OPEN`, set by `ElementData::flush_open_state` at element construction and on every `open` attribute write or removal through `DocumentMutator`, so `:open` matches it (it never did); activating a `details`'s first `summary` (above) toggles `open` and so flips `:open`; no test asserts `:open` (packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510) — upstream's, an engine change for every document ratified for this take by the founder's «Одиннадцать коммитов (Рекомендую)» (2026-10-27) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md"
    sidecar: "keyboard-navigation: `:open` matches a `details` or `dialog` carrying `open` (`flush_open_state` at construction and on every `open` write), so summary activation flips it — 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs add `ElementData::flush_open_state` (node/element.rs:543-549), called from `ElementData::new` (:488-489) and the mutator's `open` arms (mutator.rs:368-371, :520-523), and `NonTSPseudoClass::Open` now reads that state (stylo.rs:510); Counts `open`: no master states `:open`; Coverage: tests ✗, summary activation unchanged; the task's F1"
    basis: packages/blitz-dom/src/node/element.rs:543
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 55
    section: "## 5. Keyboard Navigation"
    change: "Add after line 55: - CSS properties are accessors on `CSSStyleDeclaration` since 2026-10-27-upstream-sync-engine-debts (upstream `efdc5472a`): their names come from Stylo's property list (`css_property_names`) and `installStyleProperties` installs them, so a script's `element.style` read or write of a property — `display` and `visibility` among them, which reach the tree's hidden-content exclusion (§6 Visibility and hit-testing) — goes through the inline style; pinned by `tests/dom.rs` `style_property_accessors`; a realization on the registered script-to-DOM crossing (packages/blitz-vibey-script/src/runtime/css.rs:8-18; packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48; packages/blitz-vibey-script/tests/dom.rs:455-487) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md"
    sidecar: "keyboard-navigation: script exposure gains CSS property accessors on `CSSStyleDeclaration` (`css_property_names`, `installStyleProperties`, pinned by `style_property_accessors`) — 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs: CSS properties as accessors on `CSSStyleDeclaration` — `runtime/css.rs` `css_property_names` (8-18) and `bootstrap/style.js` `installStyleProperties` (26-48) (`efdc5472a`); Coverage: tests unit `style_property_accessors`, a realization on the registered script-to-DOM crossing (Decisions & corrections); Counts `cssdecl`: control unfired, no master states it; the task's F4"
    basis: packages/blitz-vibey-script/tests/dom.rs:456
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 55
    section: "## 6. Visual Design Verification"
    change: "Add after line 55: - Author styles keyed on `:open` apply to an open `details` or `dialog` in every document since 2026-10-27-upstream-sync-engine-debts — `NonTSPseudoClass::Open` reads the element's `ElementState::OPEN` where it read false (§5 Focus restoration) — an engine change ratified for this take by the founder's «Одиннадцать коммитов (Рекомендую)» (2026-10-27); unasserted: no test asserts `:open`, `details_element` and the a11y leg green (packages/blitz-dom/src/stylo.rs:510) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md"
    sidecar: "visual-design-verification: author styles keyed on `:open` now apply to an open `details` or `dialog`, unasserted — 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs: `NonTSPseudoClass::Open` now reads `ElementState::OPEN` in place of `false` (stylo.rs:510) — `:open` matches; Decisions & corrections: the take ratifies the engine change for every document for `:open`; Coverage: `:open` tests ✗, `details_element` and the a11y leg green; the task's F2"
    basis: packages/blitz-dom/src/stylo.rs:510
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 98
    section: "## 7. Screen Reader Support"
    change: "Add after line 98: - A text input already focused when its text editor is constructed enables IME at construction — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` (upstream `7fc9ee3af`), so an input focused before its editor exists has IME on with no fresh focus; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/blitz-dom/src/layout/construct.rs:313-317; packages/dioxus-native-dom/src/dioxus_document.rs:725-759) — an engine change for every document ratified for this take by the founder's «Одиннадцать коммитов (Рекомендую)» (2026-10-27) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md"
    sidecar: "screen-reader-support: a text input already focused when its editor is constructed enables IME there (`create_text_editor`, pinned by `input_focused_on_mount_enables_ime`) — 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs: `create_text_editor` (layout/construct.rs:313-317) calls `shell_provider.set_ime_enabled(true)` for a text input already focused (`7fc9ee3af`), and dioxus-native-dom adds `input_focused_on_mount_enables_ime` (dioxus_document.rs:725-759); Counts `ime`: the enable reading `screen-reader-support.md:98` read at P2; Coverage: a11y IME (§7) ✓; the task's F3"
    basis: packages/blitz-dom/src/layout/construct.rs:313
    fact: F3
note: No scoped detector fires — the chunk adds no interactive UI element (D-a11y-surface) and changes no violation or log schema (D-a11y-obs-schema); the IME enable reading's second site (keyboard-navigation.md:23, removal's teardown) and the `:open` sites in #1.9 / #3.4 / #4.9 were left to the single F1 and F3 placements to avoid duplicate detail.
