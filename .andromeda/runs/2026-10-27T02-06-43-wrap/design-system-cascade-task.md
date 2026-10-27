# Task — design-system · 2026-10-26-codebase-health

## The session
Refactored three private Rust functions to bring their cognitive complexity under 15 (v030-01), with no behaviour change: the visual lint's findings walk in escher-driver and two table-asserting tests.
In `lint.rs`, `findings` keeps its signature and its one caller `Session::lint`, and now walks per entry through two new private helpers, `readings` and `placement`. Each finding's defect, order and reason is unchanged, and so is the `fonts-unavailable` refusal.
The MCP listing test and the 0.2.0 requirement sweep test each moved their per-row assertions into new private `#[track_caller]` helpers. Test names, targets, counts and messages are unchanged.
Measured: the complexity population over 15 fell from 88 to 85. escher-driver coverage reads 4930/5031 = 97.99 %. Line citations into the three files moved. Nothing was added to or retired from any public symbol, verb, field, carrier, dependency, token, colour, font or surface.

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md

## Touches
- Color palette, Border Progression group: the report's `complexity` sweep matched the colour row `hover 152`, because the pattern `over 15` also matches inside it. A re-read found it unmoved. It is a colour value, not a complexity reading, so it needs no change.
- Any palette or contrast line that rests on the lint's `contrast` defect or on `Harness::text_contrast` with its required ratios 4.5 and 3.0: a re-read finds it unmoved. The contrast arm's reader is untouched, and the planted row still reads 2.8490272 against 4.5.
- The seven_guis accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) and its contrast readings are unmoved. No task CSS changed, and the a11y leg reads 14 · 85 · 0 · 0 unchanged.
- Typography loading rule (a `screenshot` refused `fonts-unavailable` with nothing painted): unmoved. The lint's own `fonts-unavailable` refusal before anything is read is also unchanged.
- The CLI surface's output rule for `escher-session` and its `mcp` role (JSON-only stdout, no colour or ESC byte, exit statuses 0 · 1 · 2 · 3): unmoved. The `lint` verb's answer `{"findings":[…]}` and the MCP tool listing are byte-unchanged. `VERBS` still holds twelve verbs, and `listing.rs` was not edited.
- The desktop-native stand load page component: unmoved. The lint still reads 0 findings on the load page at boot, and `stand_load_page` is green.
- Lint readings at boot on the stand, `[0, 1, 0, 0]`: unmoved. This covers the flight booker's one `overlap` and the twelfth-Create `clipped` row inside `crud-list`. Any design-system line that cites these readings stays true.
- Any design-system citation into `packages/escher-driver/src/lint.rs` moves only if it falls at or after the first hunk at `:273`. The inline tests moved from `:526-604` to `:542-620`, `ordered` from `:299` to `:315`, and `overlaps` from `:334` to `:350`. Citations above `:273` are unmoved, and so is every `stand_lint.rs` citation, because that file was not edited.
- Any design-system citation into `packages/escher-driver/src/mcp_tests.rs` at or after base line `:480` moves by +17, and the file grew from 786 to 803 lines. Any citation into `tests/blitz-tests/tests/stand_requirement_sweep.rs` at or after `:1017` moves, and that file grew from 1090 to 1109 lines.
- The test counts any group may hold are unchanged: stand checks 205 · 0 · 7, workspace 208 · 1117 · 0 · 12, stand-repeat 204 · 864 · 0 · 21, `stand_lint` 7, and escher-driver unit tests 100.

## New facts
- (none)

## Untouched
- Color palette core, surface, text and semantic values, and the engine-painted defaults: no CSS, token or paint code changed.
- Typography mapping, the `system-fonts` feature rule and the WASM font-context rule: no font, text-layout or harness font code changed.
- Spacing values in the apps, the engine and the fixtures: no stylesheet or layout code changed.
- Depth strategy, including the hit-testing rule at a paint-clipped padding box: no blitz-dom code changed, and the guard confirmed nothing under `packages/blitz-dom/`.
- Border radius values: no stylesheet changed.
- Motion: the into-view scroll, harness settle, `VirtualClock`, `advance` and the load-wait rules. No clock, settle or scroll code changed.
- Iconography: the browser toolbar icons, cursors and favicons. No `apps/` or cursor code changed.
- The desktop-native surface's tokens, titlebar, custom widget, devtools overlay, window and stack groups: no shell, winit or example UI code changed.
- The mobile-native surface's tokens, URL bar, back button, capture and stack groups: no `apps/browser` or mobile build change.
- The web-spa surface's tokens and canvas: no WASM or `index.html` change.
- The CLI surface's tokens (owo-colors in the WPT runner): the WPT runner was not edited.
