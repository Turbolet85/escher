# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan--cascade-g1.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:18#1
- fanout-a11y-plan--cascade-g2.md 1 · applied · .andromeda/a11y-plan-detail/keyboard-navigation.md:27+#1
- fanout-a11y-plan--cascade-g2.md 2 · applied · .andromeda/a11y-plan-detail/keyboard-navigation.md:55+#1
- fanout-a11y-plan.md 1 · applied · .andromeda/a11y-plan-detail/keyboard-navigation.md:54#1, .andromeda/a11y-plan-detail/keyboard-navigation.md:55#1
- fanout-a11y-plan--cascade-g2.md 3 · applied · .andromeda/a11y-plan-detail/visual-design-verification.md:55+#1
- fanout-a11y-plan--cascade-g2.md 4 · applied · .andromeda/a11y-plan-detail/screen-reader-support.md:98+#1

## Edits
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:18
  old: «re-read on the upstream take read at `f9cbb2756` and `tokio` 1.54.1: no package enters or leaves `Cargo.lock`, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md, and again, with no dependency moved and the `escher-session` host serving six tasks, at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»
  new: «re-read on the upstream take read at `ebacd68bd` (merge base `23b9b26a9`): `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` unchanged by the take, no package entering, leaving or moving, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/keyboard-navigation.md:27
  anchor: «- Activating the first `summary` of a `details` toggles it open and focuses the summary (packages/blitz-dom/src/events/pointer.rs:830-856)»
  add: «- A `details` or `dialog` carrying `open` holds the `OPEN` element state — `ElementData::flush_open_state`, run at element construction and on every `open` write or removal through the mutator — so `:open` matches it, in every document; activating a `details`'s first `summary` (above) toggles `open` through the mutator and so flips `:open`; no test asserts `:open` (packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/keyboard-navigation.md:54
  old: «packages/blitz-vibey-script/src/dom/element.rs:1007-1019»
  new: «packages/blitz-vibey-script/src/dom/element/form_controls.rs:286-298»
- line: .andromeda/a11y-plan-detail/keyboard-navigation.md:55
  old: «packages/blitz-vibey-script/src/dom/element.rs:530-547»
  new: «packages/blitz-vibey-script/src/dom/element/form_controls.rs:41-66»
- after: .andromeda/a11y-plan-detail/keyboard-navigation.md:55
  anchor: «- `autofocus` reflection writes the value "true", one of the values blitz-dom's autofocus handling takes — any present value but "false" (Focus order above); blitz-dom is used with feature `autofocus` (packages/blitz-vibey-script/src/dom/element.rs:530-547; packages/blitz-vibey-script/Cargo.toml:20)»
  add: «- `CSSStyleDeclaration` exposes each CSS property as an accessor, its names from Stylo's property list (`css_property_names`), installed by `installStyleProperties`, so a script's `element.style` read or write of a property — `display` and `visibility` among them, which reach the tree's hidden-content exclusion (§6 Visibility and hit-testing) — goes through the inline style; pinned by `tests/dom.rs` `style_property_accessors`; a realization on the registered script-to-DOM crossing (packages/blitz-vibey-script/src/runtime/css.rs:8-18; packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48; packages/blitz-vibey-script/tests/dom.rs:455-487) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/visual-design-verification.md:55
  anchor: «- Disabled icon buttons render at 0.35 opacity (apps/browser/assets/browser.css:191-198)»
  add: «- Author styles keyed on `:open` apply to an open `details` or `dialog` in every document, `:open` reading the element's `OPEN` state (§5 Focus restoration); unasserted — no test asserts `:open` (packages/blitz-dom/src/stylo.rs:510) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/screen-reader-support.md:98
  anchor: «- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:771-784; packages/blitz-dom/src/node/node.rs:794-801)»
  add: «- A text input already focused when its text editor is constructed enables IME there — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` — so an input focused before its editor exists has IME on with no fresh focus, in every document; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872; packages/dioxus-native-dom/src/dioxus_document.rs:725-759) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»

## Patterns
[[pattern]]
id = "a11y-01"
fixed = "re-read on the upstream take read at `f9cbb2756`"
