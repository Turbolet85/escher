### iconography#1.1 · values · icons · ## Iconography · lines 8–8
- The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there — upstream's #1220, taken on F1 answered B by the founder's delegate, ratified by the founder (2026-10-18) (packages/blitz-dom/src/document.rs:2390-2450; packages/blitz-dom/src/document.rs:3279-3287)
### iconography#1.2 · values · icons · ## Iconography · lines 9–9
- CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49); cursor icons use the `cursor-icon` crate and `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15); in the shell they are winit CursorIcon values, and None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:108-119)
### iconography#1.3 · values · icons · ## Iconography · lines 10–10
- `favicon_url` returns the `href` of the first `<link>` whose `rel` contains `icon` (packages/blitz-dom/src/document.rs:631-646)
### iconography#2.1 · values · icons · ## Iconography · lines 16–16
- A fixture enumerates 36 CSS cursor values as "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54-91)
### iconography#2.2 · values · icons · ## Iconography · lines 17–17
- Inline SVG octicons in the captured GitHub fixture (examples/assets/github_profile_reduced2.html:65-67); google inline SVG paths in 24-unit viewBoxes (examples/assets/google.html:3086-3095; examples/assets/google.html:3159-3161; examples/assets/google.html:3219-3221)
### iconography#2.3 · values · icons · ## Iconography · lines 18–18
- Font Awesome classes in the gosub, pseudo, servo-new and servo fixtures, servo.html loading from use.fontawesome.com; pseudo defines .fa-github:before and .gh:before with content "\f09b" (examples/assets/gosub.html:107-109; examples/assets/pseudo.html:71-73; examples/assets/servo-new.html:206-218; examples/assets/servo.html:26; examples/assets/servo.html:154; examples/assets/servo.html:161; examples/assets/servo.html:168; examples/assets/servo.html:253; examples/assets/pseudo.html:55-65)
### iconography#2.4 · values · icons · ## Iconography · lines 19–19
- graphite positions icons from one sprite atlas through `--atlas-index` and embeds SVG icons as CSS data URIs (examples/assets/graphite.html:661-666; examples/assets/graphite.html:748-753; examples/assets/graphite.html:1948; examples/assets/graphite.html:1229; examples/assets/graphite.html:1627)
### iconography#2.5 · values · icons · ## Iconography · lines 20–20
- Inline SVG icons with `fill="currentColor"` (examples/assets/svg.html:4-12); svg_native fills an icon circle with `currentColor` set from the svg's `color` (examples/svg_native.rs:27-29); the Preact TodoMVC destroy button renders `×` (examples/preact/index.html:99-103)
### iconography#3.1 · absence · icons · ## Iconography · lines 26–26
- icon assets or icon handling · searched: `icon` over the 16 listed s08 files (no match)
### iconography#3.2 · absence · icons · ## Iconography · lines 27–27
- icons or icon sets · searched: `icon` over the 21 listed s11 files
