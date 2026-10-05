# blitz-traits

_Crate notes. Primary source: `.andromeda/architecture.md` §Standard Contracts (Provider traits)._

## Responsibility
Types and traits shared by the other Blitz crates "without circular or unnecessary dependencies": modules devtools, events, navigation, net, node_id, shell. It owns no behaviour beyond `Dummy*` no-op implementations.

## Key integrations

### Consumes from
- keyboard-types, cursor-icon, http, url, bytes.

### Publishes to
- `NetProvider::fetch(doc_id, Request, Box<dyn NetHandler>)` + `is_noop()`; `NetHandler::bytes`; `NetWaker`; `Request` / `Body` / `FormData`; `AbortController` / `AbortSignal`.
- `NavigationProvider::navigate_to(NavigationOptions)`; `ShellProvider` (redraw, cursor, title, IME, window chrome, clipboard, file dialog — no-op defaults).
- `UiEvent`, `DomEvent`, `DomEventKind`, `EventState`, `BlitzPointerId`, per-event `cancelable()` / `bubbles()` tables, `BlitzImeEvent`.
- `NodeId` (32-bit slot + 32-bit version, `as_u64()` for AccessKit interop), `Viewport` (`color_scheme`, `window_size`, `hidpi_scale`, `zoom`), `DevtoolSettings`.

## Internal conventions
- `#[non_exhaustive]` on `NavigationOptions` and `Request`; no-op implementations prefixed `Dummy`.
- `BlitzImeEvent` copies winit's IME event so lower crates need not depend on winit.

## Crate-specific gotchas
- `ClipboardError` is a unit struct with a TODO; `NodeId` order is index-then-version (roughly creation order), not document order.

## Entry points for modification
- `src/{net,navigation,shell,events,node_id,devtools}.rs`

## Testing this crate
- No in-crate tests; exercised through blitz-dom and `tests/blitz-tests`.

## References
- `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
