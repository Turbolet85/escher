# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:8) Every candidate take must leave the cursor's resolution order as written, a pointer over no box included, and any shape carrying upstream's hit-testing commits (`ebacd68bd`, `0980cce5f`) must resolve to this order; a change to it is the founder's, this being a rule he ratified.
  > The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there — upstream's #1220, taken on F1 answered B by the founder's delegate, ratified by the founder (2026-10-18)
- (.andromeda/design-system-detail/iconography.md:9) A take that moves the coupled pins (winit beta.4, or the whole merge's git Stylo) must keep the one-to-one keyword map, `cursor: none` as no cursor, the optional `CursorIcon` on `set_cursor`, and the shell's None behaviour, re-reading each against the moved crates.
  > CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49); cursor icons use the `cursor-icon` crate and `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15); in the shell they are winit CursorIcon values, and None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:108-119)

## Bindings
- **Cursor resolution ↔ the body's §Iconography**: the rule stands in both the detail and the broad view, and a take must leave the two saying the same, (.andromeda/design-system-detail/iconography.md:8) (.andromeda/design-system.md:151)
- **A pointer over no box ↔ Depth Strategy's hit stops**: whatever hit-testing hunks a take brings, the hit stops must stand as the body writes them and a point answered by no box must still read Default, (.andromeda/design-system-detail/iconography.md:8) (.andromeda/design-system.md:87)
