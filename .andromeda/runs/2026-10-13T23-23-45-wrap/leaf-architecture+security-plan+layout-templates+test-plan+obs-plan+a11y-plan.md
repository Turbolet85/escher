group: architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «ratified by the founder, 2026-10-13), and the changed set»
  source: .andromeda/architecture.md:87 «three more direct engine changes for every document stand beside those since 2026-10-13-stand-keyboard-harness, the sixth to the eighth: keyboard activation — a pressed `Enter` whose target is the focused node»
  replace-with: «ratified by the founder, 2026-10-13), keyboard activation and a button click's focus (a pressed `Enter` on the focused `button`, `summary`, `input` of type `submit` · `reset` · `button` or `a` with `href`, and a released Space — `handle_key_up` — on the focused `button`, `summary` or `input` of type `submit` · `reset` · `button` · `checkbox` · `radio`, each with no `disabled` attribute, dispatch a Click the app's handlers see before `handle_click`; a click on a `button` or an `input` of type `submit` · `reset` · `button` focuses it; focusability reads `disabled` by its presence on any element — three direct engine changes for every document, ratified by the founder, 2026-10-13), and the changed set»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «its flight booker and CRUD state which control is pressed, invalid or selected by `aria-*` attributes alone;»
  source: .andromeda/layout-templates.md:10 «since 2026-10-13-stand-keyboard-harness the CRUD list is a keyboard region — `crud-list` takes `tabindex="0"` and an `onkeydown`: ArrowDown selects the next visible row»
  replace-with: «its flight booker and CRUD state which control is pressed, invalid or selected by `aria-*` attributes alone; its CRUD list `crud-list` is a keyboard region — the third CRUD Tab stop, ArrowDown · ArrowUp · Home · End moving the selection and filling the fields as a row's click does;»

- leaf: CLAUDE.md:45   class: omission
  leaf-text: «focusability, focus order and painted colours are a11y surface — `stand_contrast` asserts every stand control's text/background pair row for row, so a stand colour change moves its record;»
  source: .andromeda/a11y-plan.md:213 «`stand_keyboard` walks each lean task's stated sequence (`tab_order`) forward and back with Tab and Shift+Tab, with wrap»
  replace-with: «focusability, focus order and painted colours are a11y surface — `stand_keyboard` asserts each lean task's Tab sequence (`common::tab_order`) and Enter and Space on each stand button, so a focus-order change moves that table, and `stand_contrast` asserts every stand control's text/background pair row for row, so a stand colour change moves its record; the engine honours no `inert`, owned by a CARRY on the route entry "Stand a11y assertions";»

- leaf: CLAUDE.md:75   class: omission
  leaf-text: «| DOM entry point · config · range input (current value · rules · key and pointer default actions) | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` · range input `packages/blitz-dom/src/node/range.rs` (unit tests inline) and `packages/blitz-dom/src/events/{keyboard,pointer}.rs` · check `tests/blitz-tests/tests/range_input_interaction.rs` |»
  source: .andromeda/architecture.md:263 «since 2026-10-13-stand-keyboard-harness the stand keyboard check `stand_keyboard`, 11 tests in both layout modes — Tab and Shift+Tab walking each lean task's stated sequence with wrap»
  replace-with: «| DOM entry point · config · range input (current value · rules · key and pointer default actions) · keyboard activation (Enter · Space → a click) and a button click's focus | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` · range input `packages/blitz-dom/src/node/range.rs` (unit tests inline) and `packages/blitz-dom/src/events/{keyboard,pointer}.rs` · check `tests/blitz-tests/tests/range_input_interaction.rs` · keyboard activation `handle_key_up` and `dispatch_activation_click` in `packages/blitz-dom/src/events/keyboard.rs`, the button arm of `handle_click` in `packages/blitz-dom/src/events/pointer.rs` · check `tests/blitz-tests/tests/stand_keyboard.rs` |»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «the diff naming the slider's `value` and its display's `name`)»
  source: .andromeda/test-plan.md:26 «since 2026-10-13-stand-keyboard-harness `press enter` and `press space` on a Tab-focused `counter-increment` each count as its click, the returned diff naming `counter-value` with `differs: [name]`»
  replace-with: «the diff naming the slider's `value` and its display's `name`, and, since the keyboard harness, `press enter` and `press space` on a Tab-focused button each counting as its click — keyboard activation, a direct engine change for every document ratified by the founder (2026-10-13))»
