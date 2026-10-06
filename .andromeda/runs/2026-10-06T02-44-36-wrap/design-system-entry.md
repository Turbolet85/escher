
## 2026-10-06-headless-stand — the headless stand registers the bundled DejaVu Sans on native
**Section:** §Typography → Loading
**Change:** a new Loading bullet: the seven_guis headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, the woff2 decoded by seven_guis' native `woff` feature. The WASM bullet stands.
**Why:** the headless stand chunk pins text measurement to one bundled face so layout is the same on the dev host and the CI runner.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
