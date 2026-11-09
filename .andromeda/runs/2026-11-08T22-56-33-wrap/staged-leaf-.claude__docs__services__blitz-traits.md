# blitz-traits

_Crate notes. Primary source: the architecture master `doc:architecture` §Standard Contracts (Provider traits)._

## Responsibility
Types and traits shared by the other Blitz crates "without circular or unnecessary dependencies": modules devtools, events, navigation, net, node_id, shell. It owns no behaviour beyond `Dummy*` no-op implementations.

## Key integrations

### Consumes from
- keyboard-types (0.8 since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`), cursor-icon, http, url, bytes.

### Publishes to
- `NetProvider::fetch(doc_id, Request, Box<dyn NetHandler>)` + `is_noop()`; `NetHandler::bytes` and, beside it, `NetHandler::error(self: Box<Self>, url: String, error: String)`, a default that does nothing, which a wrapping handler must forward or the failure stops at the wrapper; `NetWaker`; `Request` / `Body` / `FormData`; `AbortController` / `AbortSignal`.
- `NavigationProvider::navigate_to(NavigationOptions)`; `ShellProvider` (redraw, cursor, title, IME, window chrome, clipboard, file dialog — no-op defaults).
- `UiEvent`, `DomEvent`, `DomEventKind` (`from_str` strips `on`, maps `composition` to `Ime` and `change` to `Change`; `DomEventKind::Change`, after `Input`, and `DomEventData::Change(BlitzInputEvent)` — named `change`, not cancelable, bubbling, with no UI event mapped and no default action — dispatched by blitz-dom after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, a text or range input firing no engine `change`), `EventState`, `BlitzPointerId`, per-event `cancelable()` / `bubbles()` tables, `BlitzImeEvent`.
- `NodeId` (32-bit slot + 32-bit version, `as_u64()` for AccessKit interop), `Viewport` (`color_scheme`, `window_size`, `hidpi_scale`, `zoom`; its `scale()` hidpi_scale × zoom), `DevtoolSettings`.

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
- `doc:architecture` (the records store) · `.claude/docs/services/blitz-dom.md`
