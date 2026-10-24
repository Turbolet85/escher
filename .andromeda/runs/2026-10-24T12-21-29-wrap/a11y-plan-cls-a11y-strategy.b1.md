# Classified — a11y-plan · a11y-strategy.b1

## Answers
a11y-strategy#1.1 · yes · add-to · the stand's windowed binary now boots a launch that resolves styles Parallel under PARALLEL_TRAVERSAL_LOCK (windowed smoke green, still no platform adapter), which this group must carry
a11y-strategy#1.2 · no · the changed-set marking rules are untouched by the session, and keyboard.rs, document.rs, settle.rs and the harness stay unchanged
a11y-strategy#1.3 · no · the two TODO arms in blitz-shell application.rs stay unchanged, since the windowed launch changes style threading and not event handling
a11y-strategy#1.4 · no · window focus and bounds forwarding in blitz-shell is unchanged, and the session touches no window event path
a11y-strategy#2.1 · no · role mapping in accessibility.rs and accessibility_roles.rs is unchanged and no session change alters an element's role
a11y-strategy#2.2 · no · only html and body mapping to Role::Unknown stands, with accessibility.rs and accessibility_roles.rs unchanged and no new role
a11y-strategy#2.3 · yes · add-to · it names the windowed AT witness it owes, which must carry the windowed launch change (Parallel styling under PARALLEL_TRAVERSAL_LOCK, smoke green, still no platform adapter)
a11y-strategy#2.4 · yes · add-to · it records the 2026-10-17 upstream sync, which must carry the 2026-10-24 sync (upstream main at f9cbb2756, ten commits taken, none touching accessibility.rs, merge base 23b9b26a9 unmoved)
a11y-strategy#3.1 · no · accesskit_xplat is unchanged, tokio carries no a11y surface, and the platform module selection stays as it was
a11y-strategy#3.2 · no · accesskit_xplat lib.rs is unchanged by the session
a11y-strategy#3.3 · no · the Android set_focus, Windows set_focus and macOS set_window_bounds in accesskit_xplat are unchanged
a11y-strategy#3.4 · no · the Android adapter's Rect import and parameters in accesskit_xplat are unchanged
a11y-strategy#3.5 · no · the Android adapter's mSurfaceView field in accesskit_xplat is unchanged
a11y-strategy#4.1 · yes · stale · the cited packages/dioxus-native/src/lib.rs:7 is in a file the windowed launch change shifted, so the line digit moved while the forwarding itself stands
