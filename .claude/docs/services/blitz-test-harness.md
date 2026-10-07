# blitz-test-harness

_Crate notes. Primary source: `.andromeda/test-plan.md` §3 and `.andromeda/architecture.md` §Standard Contracts Test harness._

## Responsibility
The headless harness for Blitz documents (unpublished): wraps any blitz-dom `Document` (`HtmlDocument`, `DioxusDocument`) with deterministic construction defaults, a pump/tick loop, `settle` (that pass repeated until no work is due), input synthesis through the real event-dispatch pipeline, and inspection helpers. No window, GPU or compositor. It is the measured starting point for escher's headless stand and driver.

## Key integrations

### Consumes from
- blitz-dom (built with `accessibility`), blitz-html (`HtmlProvider`), dioxus-native-dom, keyboard-types.

### Publishes to
- `Harness`, `HarnessOptions { width, height, scale, color_scheme, base_url, net_provider, font_ctx, incremental }` (`font_ctx` → `DocumentConfig.font_ctx`, `incremental` → `DocumentConfig.incremental`; `Default` leaves both `None`), `key_event`, `mouse_pointer_event`, `pointer_event`, `touch_pointer_event`, `Rect`, `Key` and `Modifiers` (re-exports of the `keyboard_types` types `press_with` takes, so a dependent crate names a key with no dependency of its own — escher-driver does), `Busy`, `NotSettled`, `SETTLE_PASS_LIMIT`, `Settled`.
- `Harness::settle() -> Result<Settled, NotSettled>` — runs every pass that is due, then says whether anything is outstanding: `Settled { passes, animating }`, or `NotSettled { busy }` with `Busy` one of `Render` · `Layout` (still busy after `SETTLE_PASS_LIMIT` = 64 passes) · `Loads` (a load in flight, reported at once). The outcome names a class only — no node, id, name, URL or value.
- Consumed by seven_guis' headless stand (`seven_guis::stand::{boot, boot_timer, options}`), which builds its pinned options as a full `HarnessOptions` literal.

## Internal conventions
- Constructors pump once; `wrap` does not. `pump` polls with no waker and resolves at harness time; `dispatch` / `dispatch_recorded` do not pump; input helpers pump after dispatch — `apple_keybinding(command)` among them, which dispatches `UiEvent::AppleStandardKeybinding` to the focused element as a macOS window does for a standard key binding (`deleteBackward:`); its one standing check is the driver's `press backspace` on the macOS CI leg.
- One settle pass is `pump`'s two calls with poll's answer kept; a pass is quiet when poll answered `false`, the hover node did not change and no load finished in it. Settle reads no clock but the harness's, advances no time (a timer or an animation is the caller's to move; a running animation is reported as `animating`), never waits on a load, neither reads nor drains the changed set, logs nothing. No input helper settles — they still pump once.
- Loads are counted on the `net_provider` the harness was constructed with: a provider that fetches is wrapped in a crate-private counter (counts only); an offline harness and a document passed to `wrap` carry none and are read through `has_pending_critical_resources()` alone.
- `dispatch_recorded` installs a `RecordingHandler` on the underlying `BaseDocument` — it bypasses Dioxus VirtualDom forwarding.
- `dom_string()` is a stable one-node-per-line tree with geometry.

## Crate-specific gotchas
- The `query` helper panics with "invalid selector" on a selector that fails to parse.
- Synthesized pointer events set page = screen = client coordinates.

## Entry points for modification
- `src/harness.rs` (construction, pump, dispatch) · `src/settle.rs` (the settle loop, its outcome types, the load counter) · `src/input.rs` (input synthesis) · `src/inspect.rs` (queries, `dom_string`)

## Testing this crate
- `cargo test -p blitz-tests --test harness_smoke` — end-to-end smoke for HTML and Dioxus documents.
- `cargo test -p blitz-test-harness --lib settle` — the crate's 11 unit tests (the load counter, the loop over a scripted document, the messages); `cargo test -p blitz-tests --test stand_settle` — settle on the stand and on fixtures, both layout modes.

## References
- `.claude/rules/verification-harness.md` · `.claude/docs/tests-summary.md`
