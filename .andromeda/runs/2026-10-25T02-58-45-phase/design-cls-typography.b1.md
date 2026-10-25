# Classified — design · typography.b1

## Answers
typography#1.1 · no · the reference browser's chrome text (tab title, urlbar, menu items, status bar); the statement covers the driver and the stand, not the upstream browser
typography#1.2 · no · the browser's suggestion section header; browser chrome, outside the statement's reach
typography#1.3 · no · the browser's FPS overlay monospace; browser chrome, outside the statement's reach
typography#1.4 · no · the about:history page heading styles; a browser page, outside the statement's reach
typography#1.5 · no · the rdme viewer's body type; apps/readme is not a surface the statement describes
typography#1.6 · no · blitz-dom's user-agent default control fonts; the UA stylesheet is excluded, and the statement quotes the stand's measured pairs, not engine defaults
typography#1.7 · no · blitz-dom's user-agent default heading sizes; the UA stylesheet is excluded
typography#1.8 · no · blitz-dom's user-agent default code and pre family; the UA stylesheet is excluded
typography#1.9 · no · the seven_guis example's type scale (base 14px, home title 36px at 700); the statement quotes no type scale, and the stand's contrast rows read each row's size from computed style
typography#1.10 · no · the todomvc example's type scale; an example the statement does not describe, and no type scale is quoted
typography#1.11 · no · the wgpu_texture example's system-ui family; an example outside the headless and native capabilities the statement names
typography#1.12 · no · the text input editor's default 16px size; an engine layout default that no statement claim quotes or rests on
typography#1.13 · no · the engine's generic base sizes (monospace 13px, other generics 16px); a type-scale default the statement does not quote
typography#1.14 · no · the list bullets' family chain; the statement describes no list rendering
typography#2.1 · no · WASM builds' bundled DejaVu registration; the web-spa surface is excluded and the statement's capabilities are headless and native
typography#2.2 · yes · the stand boots the bundled DejaVu Sans for every generic family with system fonts off, the font the statement names as the reason a headless shot paints text offline
typography#2.3 · yes · build_single_font_ctx makes one font the fallback for sans-serif with system fonts disabled, the mechanism behind the stand's bundled font and its passing sans-serif check
typography#2.4 · yes · dioxus-native's system-fonts and woff features, the build conditions that headless text measurement and the bundled font's decoding rest on
typography#2.5 · no · the bullet font names no sans-serif family and moves no can_paint_text reading; the statement describes no list bullets
typography#2.6 · yes · a shot whose font context resolves no sans-serif family is refused fonts-unavailable with nothing painted, the limit the statement states for shot and lint
typography#2.7 · no · document.fonts is a script-side stub; the statement's load wait and settle concern engine loads and describe no scripts
typography#2.8 · yes · blitz-tests builds blitz-dom with system-fonts, without which text measures 0x0; the statement's headless measurement build requirement rests on it
typography#3.1 · yes · None maps to Parley's sans-serif generic, the default family unstyled text takes, which the shot's sans-serif check rests on
typography#3.2 · no · weight, width, style and variation conversions to Parley; contrast reads computed weight from Stylo and the statement states no shaping value
typography#3.3 · no · the font-variant OpenType feature tags; no claim in the statement rests on feature tags
