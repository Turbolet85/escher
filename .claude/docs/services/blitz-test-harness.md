# blitz-test-harness

_Crate notes. Primary source: `.andromeda/test-plan.md` §3 and `.andromeda/architecture.md` §Standard Contracts Test harness._

## Responsibility
The headless harness for Blitz documents (unpublished): wraps any blitz-dom `Document` (`HtmlDocument`, `DioxusDocument`) with deterministic construction defaults, a pump/tick loop, input synthesis through the real event-dispatch pipeline, and inspection helpers. No window, GPU or compositor. It is the measured starting point for escher's headless stand and driver.

## Key integrations

### Consumes from
- blitz-dom (built with `accessibility`), blitz-html (`HtmlProvider`), dioxus-native-dom, keyboard-types.

### Publishes to
- `Harness`, `HarnessOptions { width, height, scale, color_scheme, base_url, net_provider }`, `key_event`, `mouse_pointer_event`, `pointer_event`, `touch_pointer_event`, `Rect`.

## Internal conventions
- Constructors pump once; `wrap` does not. `pump` polls with no waker and resolves at harness time; `dispatch` / `dispatch_recorded` do not pump; input helpers pump after dispatch.
- `dispatch_recorded` installs a `RecordingHandler` on the underlying `BaseDocument` — it bypasses Dioxus VirtualDom forwarding.
- `dom_string()` is a stable one-node-per-line tree with geometry.

## Crate-specific gotchas
- The `query` helper panics with "invalid selector" on a selector that fails to parse.
- Synthesized pointer events set page = screen = client coordinates.

## Entry points for modification
- `src/harness.rs` (construction, pump, dispatch) · `src/input.rs` (input synthesis) · `src/inspect.rs` (queries, `dom_string`)

## Testing this crate
- `cargo test -p blitz-tests --test harness_smoke` — end-to-end smoke for HTML and Dioxus documents.

## References
- `.claude/rules/verification-harness.md` · `.claude/docs/tests-summary.md`
