# Classified — design-system · surface.b2

## Answers
surface#6.2 · no · the transparent example's window and shell close sit in examples/transparent, which the session did not reach
surface#6.3 · no · the alpha-mode handling sits in packages/dioxus-native/src/config.rs, not a reached file
surface#7.1 · no · the wasm32 canvas wiring sits in packages/dioxus-native/src/config.rs, not a reached file
surface#7.2 · no · the scrollbars forward sits in packages/dioxus-native/Cargo.toml, a manifest the session did not move
surface#7.3 · no · the scrollbar thumb sits in blitz-paint's select_list.rs and its test in select_popup_painted.rs, neither reached; the select list's paint position is re-read and holds
surface#8.1 · no · the urlbar font and padding sit in apps/browser/assets/browser.css, not a reached file
surface#9.1 · yes · stale · class and attribute changes no longer take the blanket restyle_subtree in mutator.rs, so the reading that the mobile frame class drives urlbar sizing moved with the restyle narrowing
surface#10.1 · no · the Android back handling sits in apps/browser/src/main.rs, not a reached file
surface#11.1 · no · the default screenshot name sits in apps/browser/src/capture.rs, not a reached file
surface#12.1 · no · ci.yml is reached, but the only change is matrix_test's free-disk step at base lines 392–398, past the cited lines 359–374, so the mobile build lines stand
surface#13.1 · no · the result colours sit in wpt/runner/src/main.rs, not a reached file
surface#13.2 · no · the print styles sit in wpt/runner/src/main.rs, not a reached file
surface#13.3 · no · the OSC 8 hyperlink code sits in wpt/runner/src/main.rs, not a reached file
surface#14.1 · no · the paint_bench output lines sit in examples/paint_bench.rs, not a reached file
surface#14.2 · no · the screenshot output lines sit in examples/screenshot.rs, not a reached file
surface#15.1 · no · escher-session sits in packages/escher-driver, which the session left untouched
surface#15.2 · no · the six-task list sits in examples/seven_guis, which the session left untouched
surface#16.1 · no · the DejaVu registration sits in examples/wasm_hello/src/lib.rs and dioxus-native config.rs, neither a reached file
surface#17.1 · no · the canvas styling sits in examples/wasm_hello/index.html and lib.rs, not a reached file
