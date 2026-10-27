## 2026-10-27-upstream-sync-engine-debts — the stand graph re-read on the take at `ebacd68bd`
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: Dioxus crates)
**Change:**
- The Dioxus crates entity's last re-read: was the upstream take read at `f9cbb2756` and `tokio` 1.54.1, no package entering or leaving `Cargo.lock`, per «2026-10-24-upstream-sync-polish-and-ship — the dropped-node hit skip, and the stand graph re-read», and the same reading again with no dependency moved and the `escher-session` host serving six tasks, per «2026-10-25-cold-agent-bar — the served pages, their open gap, the lint's option overlap and the scrollbar's band»; now the upstream take read at `ebacd68bd` (merge base `23b9b26a9`): `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` unchanged by the take, no package entering, leaving or moving, and the stand's graph still holding neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still.
- The rest of the entity — the feature's reach through escher-driver, the adapter kept off the stand binary and its AT witness's residuals owner — reads as before.
**Why:** the take moved the upstream read point; the manifests, the lock and the adapter probe were measured at the chunk, the probe green with 0 rows.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — citations into code the take moved, re-pointed
**Section:** §2 A11y Strategy (a11y-strategy.md: the tree lifecycle, the window-event forwarding) · §4 ARIA Patterns & Roles (aria-patterns-roles.md: browser images) · §5 Keyboard Navigation (keyboard-navigation.md: focusability, the cached-focusability recompute, the disabled states, disabled-ness keyed one way, the file input's inner button, autofocus, the key dispatch, the macOS key bindings, IME forwarding, script `focus()` / `blur()` and `autofocus` reflection) · §6 Visual Design Verification (visual-design-verification.md: page zoom)
**Change:**
- Citations into blitz-shell's `window.rs`, blitz-dom's `node/element.rs` and `mutator.rs` and the browser's `tab.rs` follow the lines the take moved; each claim's text is unchanged.
- The script `focus()` / `blur()` bodies and the `autofocus` reflection (`get_autofocus`, `set_autofocus` writing "true") are cited in blitz-vibey-script's `dom/element/form_controls.rs` (was `dom/element.rs`, which now holds only the `focus` and `blur` method registrations); the claims are unchanged.
**Why:** the take's commits shift code in those files, and blitz-vibey-script's element bindings split into modules, so the cited script bodies left `dom/element.rs`. The citation sweep re-points each row by the take's line map. The script rows' old path no longer held their code, so they were re-pointed by hand at this wrap.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `:open` matches an open `details` or `dialog`
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: a new bullet after the `summary` activation bullet) · §6 Visual Design Verification (visual-design-verification.md: a new visual-state bullet)
**Change:**
- New (§5): a `details` or `dialog` carrying `open` holds the `OPEN` element state, set by `ElementData::flush_open_state` at element construction and on every `open` write or removal through the mutator, so `:open` matches it, in every document; activating a `details`'s first `summary` toggles `open` through the mutator and so flips `:open`; no test asserts `:open`.
- New (§6): author styles keyed on `:open` apply to an open `details` or `dialog` in every document, `:open` reading the element's `OPEN` state; unasserted, no test asserting `:open`.
- `summary` activation, focus order and the keys are unchanged.
**Why:** upstream's engine change for every document, taken in this sync on the founder's own word on the take (2026-10-27), relayed verbatim by the overseer, which ratifies for this take an engine behaviour change for every document. The take adds no `:open` test; `details_element` and the a11y leg read green at the chunk.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — CSS properties as `CSSStyleDeclaration` accessors
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: a new keyboard-framework bullet after the `autofocus` reflection)
**Change:**
- New: `CSSStyleDeclaration` exposes each CSS property as an accessor, its names from Stylo's property list (`css_property_names`), installed by `installStyleProperties`. A script's `element.style` read or write of a property therefore goes through the inline style, including `display` and `visibility`, which reach the tree's hidden-content exclusion (§6 Visibility and hit-testing). It is pinned by `tests/dom.rs` `style_property_accessors` and is a realization on the registered script-to-DOM crossing.
- The hidden-content rule itself is unchanged.
**Why:** upstream's change, taken in this sync on the founder's own word on the take (2026-10-27), relayed verbatim by the overseer. No master stated it before, so it is new detail measured at the chunk.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — IME enabled at a focused input's editor construction
**Section:** §7 Screen Reader Support (screen-reader-support.md: sr-ime, a new bullet after the focus-enables-IME bullet)
**Change:**
- New: a text input already focused when its text editor is constructed enables IME there (`create_text_editor` calls `shell_provider.set_ime_enabled(true)`), so an input focused before its editor exists has IME on with no fresh focus, in every document. It is pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime`.
- Focus enabling IME and blur disabling it read as before.
**Why:** upstream's engine change for every document, taken in this sync on the founder's own word on the take (2026-10-27), relayed verbatim by the overseer, which ratifies for this take an engine behaviour change for every document. The one new dioxus-native-dom unit test was measured at the chunk.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the stand accent's home at layout-templates, ratified (F2)
**Section:** §6 Visual Design Verification (visual-design-verification.md: the 2026-10-14-stand-a11y-assertions accent and contrast-record entry; no line moves)
**Change:**
- The stand accent's cross-master home is `layout-templates §Surface: desktop-native → Primary screens`, its `values · desktop-native-screens` block (moved from `design-system §Color Palette`). The move was PROVISIONAL and is now ratified.
- The entry still reads as before: the shades `#3a5ce5` · `#2a4cd3` · `#1f3fc1`, white on each 5.47 · 6.83 · 8.27, the old `#4a6cf7` at 4.39, the record's pass 13 · fail 0 · exempt-inactive 3 · no-text 1, `TEXT_ROWS` 14 and `STATE_ROWS` 18, and every quoted pair (the invalid `#c53030` on `#fff5f5`, the select's `#1a1a1a` on `#ffffff`). The a11y-plan body carried no PROVISIONAL mark to drop.
**Why:** the founder's own word by phone (2026-10-27), relayed for this wrap by the operator, confirms F2, his delegate's answer, so the nine rows' move stands. The discharge follows his word, on the operator's confirm at this wrap's P2 halt by question dialog. That confirm also directs one new entry per touched doc that names F2 ratified, while the earlier entries keep their words.
**Kept:** the 2026-10-27-readme-and-records PROVISIONAL entry «2026-10-27-readme-and-records — the stand accent's home re-pointed to layout-templates (PROVISIONAL)» keeps its words and is not superseded: its re-point of the stand accent's home to layout-templates still stands in the body.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
