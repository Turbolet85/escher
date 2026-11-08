@answer n=1 entity=surface#6.1 v=no
why: the macOS browser titlebar rule lives in apps/browser/src/main.rs which the session does not reach
@answer n=2 entity=surface#6.2 v=no
why: the transparent example window lives in examples/transparent/src/app.rs which the session does not reach
@answer n=3 entity=surface#6.3 v=no
why: the compositing alpha mode lives in packages/dioxus-native/src/config.rs which the session does not reach
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 canvas sizing lives in packages/dioxus-native/src/config.rs which the session does not reach
@answer n=5 entity=surface#7.2 v=yes mode=stale
why: packages/dioxus-native-dom/src/dioxus_document.rs is reached whole and gains the registry dispatch around handle_event, so the cited lines 167-169 and 315-317 may have moved
@answer n=6 entity=surface#7.3 v=no
why: the scrollbars feature forwarding lives in packages/dioxus-native/Cargo.toml and the session changes no manifest
@answer n=7 entity=surface#7.4 v=no
why: the select list scrollbar thumb lives in packages/blitz-paint which the session does not reach
@answer n=8 entity=surface#8.1 v=no
why: the mobile urlbar font and padding live in apps/browser/assets/browser.css which the session does not reach
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class sizing lives in apps/browser/src/main.rs and browser.css which the session does not reach
@answer n=10 entity=surface#10.1 v=no
why: the Android back button behaviour lives in apps/browser/src/main.rs which the session does not reach
@answer n=11 entity=surface#11.1 v=no
why: the mobile screenshot save path lives in apps/browser/src/capture.rs which the session does not reach
@answer n=12 entity=surface#12.1 v=no
why: the iOS and Android build-only status lives in .github/workflows/ci.yml which the session does not reach
@answer n=13 entity=surface#13.1 v=no
why: the WPT runner result colours live in wpt/runner/src/main.rs which the session does not reach
@answer n=14 entity=surface#13.2 v=no
why: the WPT runner bright black output lives in wpt/runner/src/main.rs which the session does not reach
@answer n=15 entity=surface#13.3 v=no
why: the WPT runner OSC 8 hyperlinks live in wpt/runner/src/main.rs which the session does not reach
@answer n=16 entity=surface#14.1 v=no
why: the paint_bench output lines live in examples/paint_bench.rs which the session does not reach
@answer n=17 entity=surface#14.2 v=no
why: the screenshot example output lines live in examples/screenshot.rs which the session does not reach
@answer n=18 entity=surface#15.1 v=yes mode=add-to
why: the escher-session command line and MCP output contract gains the graph verb one uncoloured JSON line with exit 0 (F1) and its unpaged one-line answer (F3)
@answer n=19 entity=surface#15.2 v=no
why: the six-task apps list and the MCP start enum are unchanged and their cited program text in examples/seven_guis/src/session_host.rs is not reached
@answer n=20 entity=surface#16.1 v=no
why: the WASM bundled DejaVu font registration lives in examples/wasm_hello/src/lib.rs and fonts are unchanged
@answer n=21 entity=surface#17.1 v=no
why: the wasm_hello canvas size and focus rule lives in examples/wasm_hello and the session does not reach it
