
## 2026-10-10-upstream-sync-agent-surfaces — autofocus is read by the attribute's presence; the stand's adapter absence re-read on the merged lock
**Section:** §5 Keyboard Navigation → Focus order per layout (engine), Script and framework exposure · §1 → Dioxus crates
**Change:**
- Focus order: was "the latest mounted focussable node with `autofocus="true"` is focused on flush"; now the node whose `autofocus` attribute is present with any value but "false" — bare, empty or "true" — and `autofocus="false"` is ignored.
- Script exposure: was "`autofocus` reflection writes the value "true" because blitz-dom's autofocus handling expects it"; now it writes "true", one of the values the handling takes.
- Dioxus crates: a re-read clause — on the merged lock, where the accesskit family moved (accesskit 0.25.1, accesskit_unix 0.24.0, accesskit_android 0.9.0), the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit`.
**Why:** upstream changed the engine's reading of the attribute (`== "true"` became `!= "false"`), with a five-test check of its own. No stand source carries `autofocus`, so no stand boot's focus moves; the a11y leg is unmoved at 6 + 6 + 3.
**Kept:** the bridge still removes a falsy Dioxus `autofocus`, so that bullet stands; the two app bullets (the browser's new-tab input, the Preact TodoMVC input) state no value and stand.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
