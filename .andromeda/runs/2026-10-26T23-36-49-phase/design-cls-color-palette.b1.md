# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · no · the colour type and its sRGB conversion are blitz-dom internals the lint never holds, since it reads only a ratio from Harness::text_contrast
color-palette#1.2 · no · the viewport colour-scheme enum, which the stand's fixed declared colours and the lint's scheme-free reading do not reach
color-palette#1.3 · no · the recascade a scheme change triggers, and the lint changes no scheme and runs no step
color-palette#1.4 · no · the scheme copy to iframe sub-documents, and no stand screen or lint path names an iframe
color-palette#1.5 · no · the legacy bgcolor attribute mapping, which no scoped stand markup names and the computed-style reading does not reach
color-palette#1.6 · no · the windowed renderer's optional frame clear colour, and the windowed path is not touched (task Not needed: desktop-native)
color-palette#1.7 · no · the paint canvas fallback, which the contrast reader does not read, since it composites ancestors' backgrounds over opaque white
color-palette#1.8 · no · the WPT runner's reference-test white fill, and the WPT runner is not reached
color-palette#2.1 · no · the reference browser's chrome focus colour, and apps/browser stylesheets are not reached (task Not needed: colour tokens of apps/browser and apps/readme)
color-palette#2.2 · no · the reference browser's about-history link colour, and apps/browser stylesheets are not reached
color-palette#2.3 · yes · the stand's accent #3a5ce5 with its hover and active shades, which stand_lint's contrast reading on the stand screens depends on through its white-on-accent pairs, measured at 14–16 px weight 500
color-palette#2.4 · yes · the load page's accent button and its #1a1a1a text on #f5f5f5, which stand_lint reads at 0 findings on the load page though no stand_contrast row does
color-palette#2.5 · no · the engine's default link, focus outline and button background, since no stand row the batch names paints with them and a focus outline is not a contrast input
color-palette#2.6 · no · the engine's default text-selection highlight, which the lint's computed colour and background reading never reads
color-palette#2.7 · no · the examples/counter and examples/transparent buttons, since the stand's counter is examples/seven_guis' counter task and neither root example is reached
color-palette#3.1 · no · the reference browser's tabstrip surfaces, and apps/browser stylesheets are not reached
color-palette#3.2 · no · the reference browser's tooltip, FPS overlay and status bar, and apps/browser stylesheets are not reached
color-palette#3.3 · no · the reference browser's about-page backgrounds, and apps/browser stylesheets are not reached
color-palette#3.4 · no · the rdme page background per scheme, and apps/readme is not reached (task Not needed)
color-palette#3.5 · yes · the seven_guis page background #f5f5f5 under the stand's #1a1a1a text, on which stand_lint's 0-findings contrast reading on counter, timer, CRUD and the load page depends
color-palette#3.6 · no · the wasm_hello dark palette, and the WASM canvas web-spa surface is not built or touched (task Not needed)
color-palette#3.7 · no · the custom-widget, static-HTML and wgpu_texture backgrounds, and the desktop-native custom widget and windowed paths are not touched (task Not needed)
color-palette#4.1 · no · the reference browser's tooltip and status-bar text, and apps/browser stylesheets are not reached
color-palette#4.2 · no · the reference browser's about-page muted text, and apps/browser stylesheets are not reached
color-palette#4.3 · no · the reference browser's error page paragraph (apps/browser/assets/error.html), and apps/browser is not reached
