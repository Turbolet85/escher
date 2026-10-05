# seven_guis (the stand)

_Crate notes. Primary sources: `.andromeda/architecture.md` §Existing Scopes, `.andromeda/layout-templates.md` desktop-native, the active `escher-X.Y.Z/intent.md`._

## Responsibility
The 7GUIs benchmark app ("Seven benchmark tasks for GUI frameworks") in Dioxus — tasks counter, temp_converter, flight_booker, timer, crud, circle_drawer, cells. escher's internal stand: every 0.1.0 capability (stable ids, snapshot, driver, screenshots, a11y) is proven on it headlessly.

## Key integrations

### Consumes from
- dioxus-native (native binary `seven_guis_native`) or a wasm32 cdylib (`--no-default-features --features hybrid`, `console_error_panic_hook`).

### Publishes to
- Nothing — an app. Screens: Home (centered 640px column of task cards with description and tag) and TaskShell (header with back button, title, spacer over a scrolling body).

## Internal conventions
- Example CSS in `const CSS: &str` raw strings; accent `#4a6cf7` on `#f5f5f5`, text `#1a1a1a`; invalid `#e53e3e` family, success `#ebf8ee` family.
- Inputs replace the focus outline with a border colour or box-shadow on focus.
- `idna_adapter = "=1.0.0"` pin disables unicode URL support.

## Stand-relevant behaviour
- Flight booker validates `dd.mm.yyyy` with month range and leap years; return not before start; invalid dates get an `invalid` class and disable Book.
- Cells: tokenizer rejects unknown characters, reference cycles read `#CYCLE`, division by zero is an error; commits on Enter or blur.
- Circle drawer clamps diameter 5–100.

## Entry points for modification
- `examples/seven_guis/src/app.rs` (Home, TaskShell) · `src/tasks/*.rs` · `src/main.rs` · `src/lib.rs` (wasm)

## Testing this crate
- No tests yet; the headless stand (no display, fixed viewport, bundled fonts, no live network, fresh per check) is the working route's "Headless stand" chunk.
- Run windowed: `just seven_guis`.

## References
- `.claude/rules/a11y.md` (the stand is in its paths) · `.claude/docs/design-summary.md`
