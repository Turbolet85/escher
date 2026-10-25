# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the blitz default user-agent sheet is set in packages/blitz-dom/src/config.rs, which the session does not reach
surface#1.2 · no · the engine's default select rules in packages/blitz-dom/assets/default.css are unchanged and the flight booker's select styling is app CSS that overrides them
surface#1.3 · no · the window's initial Light theme is set in packages/blitz-shell/src/window.rs, which the session does not reach
surface#1.4 · no · the default viewport and scale are set in packages/blitz-traits/src/shell.rs, which the session does not reach
surface#1.5 · no · the JS innerWidth, innerHeight and devicePixelRatio readings are in packages/blitz-vibey-script/src/runtime.rs, which the session does not reach
surface#2.1 · no · the macOS tabstrip padding and height are in apps/browser, which the session leaves unchanged
surface#2.2 · no · the shell provider's window controls are in packages/blitz-shell/src/lib.rs and event.rs, which the session does not reach
surface#3.1 · no · the custom widget's 0.8 opacity is in examples/custom_widget.rs, which the session does not reach
surface#4.1 · no · the devtools overlays are in packages/blitz-traits/src/devtools.rs, which the session does not reach
surface#5.1 · no · the browser window title rule is in apps/browser/src/main.rs and the shell's window.rs, which the session does not reach
surface#5.2 · no · rdme's window title and theme toggle are in apps/readme, which the session does not reach
surface#6.1 · no · the macOS browser titlebar is in apps/browser/src/main.rs, which the session does not reach
surface#6.2 · no · the transparent example is in examples/transparent, which the session does not reach
surface#6.3 · no · the compositing alpha mode is configured in packages/dioxus-native/src/config.rs, which the session does not reach
surface#7.1 · no · the wasm32 canvas is in packages/dioxus-native/src/config.rs, which the session does not reach, and no WASM page or canvas changed
surface#7.2 · no · the scrollbars feature's forwarding in packages/dioxus-native/Cargo.toml is unchanged, and the session does not reach that file
surface#8.1 · no · the mobile urlbar sizing is in apps/browser/assets/browser.css, which the session does not reach
surface#9.1 · no · the mobile frame class is in apps/browser/src/main.rs, which the session does not reach
surface#10.1 · no · the Android back button is in apps/browser/src/main.rs, which the session does not reach
surface#11.1 · no · mobile screenshot saving is in apps/browser/src/capture.rs, which the session does not reach
surface#12.1 · no · the CI target note is in .github/workflows/ci.yml, which the session does not reach
surface#13.1 · no · the WPT result colors are in wpt/runner/src/main.rs, which the session does not reach
surface#13.2 · no · the WPT test-kind and heading colors are in wpt/runner/src/main.rs, which the session does not reach
surface#13.3 · no · the WPT hyperlink wrapping is in wpt/runner/src/main.rs, which the session does not reach
surface#14.1 · no · the paint_bench output line is in examples/paint_bench.rs, which the session does not reach
