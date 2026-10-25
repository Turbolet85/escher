# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the blitz UA sheet every document gets once through DocumentConfig::default() supplies the select's and inputs' engine defaults, so the offline boot the chunk edits must keep it once
surface#1.2 · yes · the select's UA border, padding, white background and chosen-option-only display are what the flight booker's new select lays out with, under its contrast rows and the lint
surface#1.3 · no · the window's initial theme (blitz-shell window.rs) never enters a headless boot, which takes its scheme from the pinned viewport (surface#1.4)
surface#1.4 · yes · the pinned 800 × 600 screen the shot and lint read is this viewport's logical size on scale 1.0, zoom 1.0 and the Light scheme, which Cells and the select must fit
surface#1.5 · no · the JS window's innerWidth, innerHeight and devicePixelRatio live in blitz-vibey-script, and the Dioxus stand runs no script
surface#2.1 · no · the macOS browser tabstrip under apps/browser, which the chunk does not touch
surface#2.2 · no · the shell provider's titlebar controls for custom chrome, a pattern the task excludes since the stand runs with no window
surface#3.1 · no · the custom-widget example's opacity composite, a pattern the task excludes
surface#4.1 · no · devtools overlay drawing, a pattern the task excludes
surface#5.1 · no · the browser window title from the active tab, window chrome the headless stand never opens
surface#5.2 · no · rdme's window title and theme toggle under apps/readme, which the chunk does not touch
surface#6.1 · no · the browser's macOS titlebar under apps/browser, which the chunk does not touch
surface#6.2 · no · the transparent example's decoration-less window, an example the chunk does not touch
surface#6.3 · no · the windowed compositing alpha mode, which the headless stand (composited over opaque white) never reaches
surface#7.1 · no · the wasm32 canvas on the page body, a web surface the task excludes
surface#7.2 · no · an optional dioxus-native feature forwarding to blitz-paint, which no reached manifest enables
surface#8.1 · no · the mobile urlbar's font and padding under apps/browser, a mobile surface the task excludes
surface#9.1 · no · the mobile frame class driving urlbar sizing, a mobile surface the task excludes
surface#10.1 · no · the Android back button in the browser, a mobile surface the task excludes
surface#11.1 · no · mobile capture saving without a dialog in the browser, a mobile surface the task excludes
surface#12.1 · no · the iOS and Android CI status in ci.yml, a workflow the chunk does not reach
surface#13.1 · no · the WPT runner's terminal result colours, which the task excludes
surface#13.2 · no · the WPT runner's bright-black terminal output, which the task excludes
surface#13.3 · no · the WPT runner's OSC 8 hyperlinks behind a terminal test, which the task excludes
surface#14.1 · no · paint_bench's printed output, which the task excludes
