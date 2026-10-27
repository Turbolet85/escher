# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the blitz UA sheet every document gets is the default each stand screen that stand_lint re-reads is painted and laid out under, its boot readings and the flight booker's overlap included
surface#1.2 · yes · the UA select box (inline-block) and the single-choice option rule are in force on the flight booker's select, the box its two options share in the one overlap stand_lint re-reads
surface#1.3 · yes · the flight booker's 15px select text, whose size sets the contrast reading's required ratio (4.5, not large text), and its colours and the box its two options share in stand_lint's one re-read overlap
surface#1.4 · no · the initial theme is the windowed shell's (blitz-shell window.rs), and the headless stand has no window, so no windowed path is touched
surface#1.5 · no · the shell's default window viewport (0,0, hidpi 1.0, zoom 1.0, Light) is not what the stand reads, since the stand boots at its own pinned viewport
surface#1.6 · no · innerWidth, innerHeight and devicePixelRatio are script-runtime reads, and the lint runs no script and no windowed path is touched
surface#2.1 · no · the macOS merged-titlebar padding and height are a browser app stylesheet and titlebar (apps/browser), an app the chunk does not reach
surface#2.2 · no · the shell provider's minimize, maximize, decorations toggle and drag are windowed titlebar calls the headless lint and the stand never make
surface#3.1 · no · the custom widget's HTML-layer opacity is a windowed example's composite (examples/custom_widget.rs), not reached
surface#4.1 · no · devtools overlays are a windowed paint aid for a hovered node, and the lint reads layout and text, never paint
surface#5.1 · yes · stand_lint's boot re-read of the load page (0 findings) reads its keyed load-fetch and load-done elements, the paragraph held display: none until its stylesheet lands
surface#6.1 · no · the browser window title follows the tab's display title through the windowed shell, which the headless lint never reads
surface#6.2 · no · rdme's window title and Ctrl/Cmd+T theme toggle are the readme app's, and no app is reached
surface#7.1 · no · the browser's macOS transparent unified titlebar is a windowed app setting (apps/browser main.rs), not reached
surface#7.2 · no · the transparent example's decoration-less 360x300 window is a windowed example, not reached
surface#7.3 · no · window compositing alpha mode is a windowed dioxus-native config, not reached
surface#8.1 · no · the wasm32 canvas is the web-spa path, which the chunk does not build or touch
surface#8.2 · no · the optional scrollbars feature only forwards to blitz-paint, which the lint never reads, and the stack pattern is not touched
surface#9.1 · no · the mobile urlbar's 16px font and padding are the mobile surface, not built or touched
surface#10.1 · no · the mobile frame class sizes the mobile urlbar, a mobile surface the chunk does not touch
surface#11.1 · no · the Android hardware back button is the mobile surface, not built or touched
surface#12.1 · no · mobile capture (apps/browser capture.rs) is the mobile surface, not built or touched
surface#13.1 · no · the iOS and Android CI targets (ci.yml:353-368) sit outside the fast and doc legs the chunk re-runs, and ci.yml is not touched
surface#14.1 · no · the WPT runner's result colours are the CLI tokens of a runner the chunk does not reach
surface#14.2 · no · the WPT runner's test-kind and summary-heading colours are its own output, and the runner is not reached
