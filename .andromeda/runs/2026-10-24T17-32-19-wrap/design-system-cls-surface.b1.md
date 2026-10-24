# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the default user-agent sheet is added in blitz-dom's config.rs, a file the session does not reach
surface#1.2 · no · the select's default border, padding and background sit in blitz-dom's default.css, a file the session does not reach and no colour or spacing value moved
surface#1.3 · no · the initial theme is set in blitz-shell's window.rs, a file the session does not reach
surface#1.4 · no · the default viewport is set in blitz-traits' shell.rs, a file the session does not reach
surface#1.5 · no · the JS window size and devicePixelRatio readers sit in blitz-vibey-script's runtime.rs, a file the session does not reach
surface#2.1 · no · the macOS tabstrip's padding and height sit in apps/browser's tab_strip.rs and browser.css, files the session does not reach
surface#2.2 · no · the titlebar provider sits in blitz-shell's lib.rs and event.rs, files the session does not reach
surface#3.1 · no · the widget's 0.8 opacity sits in examples/custom_widget.rs, a file the session does not reach
surface#4.1 · no · the devtools overlays sit in blitz-traits' devtools.rs, a file the session does not reach
surface#5.1 · no · the browser title follow sits in apps/browser/src/main.rs and blitz-shell's window.rs, files the session does not reach
surface#5.2 · no · rdme's title and Ctrl/Cmd+T theme toggle sit in apps/readme, files the session does not reach
surface#6.1 · no · the macOS unified titlebar sits in apps/browser/src/main.rs, a file the session does not reach
surface#6.2 · no · the transparent window sits in examples/transparent/src/app.rs, a file the session does not reach
surface#6.3 · no · the alpha-mode config sits in dioxus-native's config.rs, a file the session does not reach
surface#7.1 · no · the wasm32 canvas sits in dioxus-native's config.rs, a file the session does not reach and no app, shell or packaging file changed
surface#7.2 · no · the scrollbars forwarding sits in dioxus-native's Cargo.toml, a file the session does not reach and no manifest changed
surface#8.1 · no · the mobile urlbar sizing sits in apps/browser's browser.css, a file the session does not reach
surface#9.1 · no · the mobile frame class sits in apps/browser's main.rs and browser.css, files the session does not reach
surface#10.1 · no · the Android back button sits in apps/browser/src/main.rs, a file the session does not reach
surface#11.1 · no · the mobile capture file name sits in apps/browser's capture.rs, a file the session does not reach
surface#12.1 · no · the iOS and Android CI lines sit in .github/workflows/ci.yml, a file the session does not reach, and the session's ci-leg.sh change adds stand-repeat targets, no mobile build
surface#13.1 · no · wpt/runner's result colours sit in wpt/runner/src/main.rs, a file the session does not reach, and no colour value moved
surface#13.2 · no · wpt/runner's bright-black headings sit in wpt/runner/src/main.rs, a file the session does not reach, and no colour value moved
surface#13.3 · no · wpt/runner's OSC 8 hyperlink rule sits in wpt/runner/src/main.rs, a file the session does not reach
surface#14.1 · no · paint_bench's output lines sit in examples/paint_bench.rs, a file the session does not reach, and the session's tests cover escher-session, not paint_bench
