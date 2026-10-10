
## 2026-10-10-upstream-sync-agent-surfaces — input rows re-read on the merged engine: the `file:` read, `autofocus`, the script Selection API; one compile-time env read
**Section:** §Input Validation → `file:` URLs (net provider), Markup attributes, JS API arguments · §Secret Management → Environment values read
**Change:**
- `file:` URLs: was "Read with `std::fs::read(request.url.path())`"; now read through `Url::to_file_path` off wasm — a `file:` URL that is no file path is a typed `InvalidInput` error, no panic — and through `request.url.path()` on wasm only. No path restriction, as before.
- Markup attributes, new row `autofocus`: on a focusable element, under blitz-dom's `autofocus` feature, the attribute enables autofocus when present with any value but `"false"` — bare, empty and `"true"` each focus the element.
- JS API arguments, new row Selection: `setBaseAndExtent` checks its argument count and offsets and throws a JS error on a bad call, leaving the selection unchanged; the API writes the document's text selection; the `innerText` / `outerText` getters only read; escher uses none.
- Environment values: the `env!("CARGO_MANIFEST_DIR")` bullet also names upstream's `tests/all.rs` of blitz-tests, a target held at `test = false` and never built. No runtime env read was added.
**Why:** each row states a reading of code the merge changed. The `file:` change narrows — an error where there was none — and leaves the standing gap, no path restriction, as it was. The Selection API is a realization on the script-to-DOM crossing scripts already write through, so it is an inventory row, not a widening.
**Kept:** the Dioxus boolean-attributes row stands: the bridge still removes a falsy `autofocus`, so a Dioxus `autofocus: false` never reaches the engine's presence reading.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
