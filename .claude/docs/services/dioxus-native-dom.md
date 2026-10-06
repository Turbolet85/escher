# dioxus-native-dom

_Crate notes. Primary source: `.andromeda/architecture.md` §Standard Contracts Dioxus DOM bridge._

## Responsibility
The headless Dioxus renderer on blitz: `DioxusDocument` integrates `BaseDocument` with a `VirtualDom`, writes vdom mutations into the DOM and routes DOM events back to vdom handlers. The windowed `dioxus-native` crate builds on it; the seven_guis stand runs through it.

## Key integrations

### Consumes from
- blitz-dom (shared as `Rc<RefCell<BaseDocument>>`), dioxus-core / dioxus-html 0.7.3.

### Publishes to
- `DioxusDocument` (`vdom`, `inner`, `initial_build`, `poll`, `handle_ui_event`, `vdom_state.try_element_to_node_id(ElementId)`), `NodeHandle` backing `MountedData`.
- Stable element ids: `element_id(NodeId) -> Option<String>` and `element_ids() -> Vec<(NodeId, String)>` (document pre-order, pairwise distinct), computed on demand and written nowhere — author key (the HTML `id`: non-empty, `/`-free, first in document order) → component path (`TaskShell/Counter/div:0`; `{tag}[{key}]` for a Dioxus-keyed root) → document path (`/html:0/body:0`). Proven persistent on the stand: the same id across a re-render, a remount (fresh `NodeId`s for every element bar the `html`/`head`/`body`/`#main` skeleton, which lives outside the VirtualDom) and a second process (`stand_id_persistence`).

## Internal conventions
- DOM events route to the vdom via the nearest `data-dioxus-id` attribute (parsed as `usize`); listener registration sets a `"<rust func>"` placeholder.
- `style`-namespace attributes become style properties; a falsy `checked` or `disabled` (Bool false, `"false"`, 0, None) clears the attribute; `dangerous_inner_html` sets inner HTML.
- Every document starts as `<html><head></head><body><main id="main"></main></body></html>` with `DEFAULT_CSS`; base URL `dioxus://index.html`.
- `mounted` listeners fire after `initial_build` and each `poll`; event kinds with zero handlers are skipped.
- A crate-local `trace!` macro expands to `tracing::debug!` under `tracing` (its 4-argument arm passes only the first two items).

## Crate-specific gotchas
- `mutation_writer.rs` has a "WARNING: DO NOT REORDER" block.
- Twelve event-data conversions call `unimplemented!()`; IME events are not handled; `NativeFormData::valid` always returns true.
- `element_to_node_id` unwraps; `NodeHandle::node` panics if the node is gone.
- `VComponent.name` is the component's full type path (`crate::module::Name`, generics included), and the user root sits under dioxus-core's RootScopeWrapper → SuspenseBoundary → ErrorBoundary scopes — the id walk strips the path and skips those four scopes.
- A node a re-render removes is only DETACHED (parent `None`) and keeps resolving until its ElementId is reassigned; `element_id` reads `None` for it, but `get_node` still returns it.
- A `key:` on an element inside an `if` within a `for` body does not key the list: the `for` items are the unkeyed `if` wrappers, diffed by position, and a changed inner key replaces the node — put the filter in the iterator so each `for` item is the keyed element.
- Every other boolean attribute (`readonly`, `required`, `hidden`, `multiple`, `selected`, `open`, `autofocus`) is still written with the literal value `"false"` when falsy — blitz-dom keys element state and click targeting on presence, so a presence read takes it as set.

## Entry points for modification
- `src/{dioxus_document,element_id,mutation_writer,events,write_once_attr}.rs`

## Testing this crate
- `cargo test -p dioxus-native-dom` (`keyed_nodes_do_not_crash`, touch tests, six `element_id` unit tests); Dioxus integration in `tests/blitz-tests` via `Harness::from_component` / `from_vdom`; `dioxus_falsy_disabled.rs` pins the falsy-`disabled` clearing.

## References
- `.andromeda/architecture.md` · `.claude/docs/services/blitz-test-harness.md`
