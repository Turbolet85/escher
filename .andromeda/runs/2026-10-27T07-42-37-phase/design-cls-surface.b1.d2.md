# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the once-only UA sheet mount behind the engine-painted defaults the Needs read through Stylo; dioxus_document.rs (reached whole) must not mount it again
surface#1.2 · yes · the select's UA border, white background and chosen-option display rule are the engine-painted colours the Needs re-read through Stylo; its 2px padding is spacing the Not needed section drops
surface#1.3 · yes · the flight booker select's colours (#1a1a1a on #ffffff, #3a5ce5 focus border) are a stand control text/background pair that stand_contrast re-asserts row for row
surface#1.4 · yes · window.rs (reached whole) holds the initial theme read from the window's theme, in the winit windows the desktop-native Need keeps
surface#1.5 · yes · blitz-traits/src/shell.rs (reached whole) holds the default viewport and logical-size reads that the stand's pinned viewport and screenshots run on
surface#1.6 · yes · blitz-vibey-script/src/runtime.rs is a conflict surface, and its innerWidth, innerHeight and devicePixelRatio reads sit in it
surface#2.1 · yes · apps/browser is reached whole, so its macOS merged-titlebar class and 44px tabstrip height bind; the 90px padding is spacing the Not needed section drops
surface#2.2 · yes · lib.rs (reached whole) holds the shell provider's minimize, maximize, decorations and drag, which the desktop-native Need's winit calls must keep working
surface#3.1 · no · the Not needed section names the desktop-native custom widget pattern, and examples/custom_widget.rs is not in the delta
surface#4.1 · no · packages/blitz-traits/src/devtools.rs is not reached, and no Need or scope item names the devtools overlays
surface#5.1 · no · the Not needed section names the stand load page's component pattern; the take changes no stand page, and the scope's load-wait re-proof states the clock, not this page's markup
surface#6.1 · yes · apps/browser (reached whole) sets the browser title from its active tab, and window.rs:200-206 (reached whole) sets the window title from the document's title node
surface#6.2 · no · apps/readme is not reached, and no Need or scope item names rdme's title or its Ctrl/Cmd+T theme toggle
surface#7.1 · yes · apps/browser (reached whole) holds the macOS transparent, unified, hidden-title titlebar attributes at main.rs:82-89, on the winit windows the desktop-native Need keeps
surface#7.2 · no · examples/transparent is not reached, and the scope names no transparent or decoration-less window; its close through the shell provider is surface#2.2
surface#7.3 · no · the alpha mode lives in packages/dioxus-native/src/config.rs, which is not reached (only its Cargo.toml is), and the scope names no compositing change
surface#8.1 · no · the wasm32 canvas surface lives in packages/dioxus-native/src/config.rs, which is not reached, and the scope names no wasm surface or build change
surface#8.2 · yes · packages/dioxus-native/Cargo.toml is reached whole, and the take edits its default features (clipboard out) in the same manifest that holds the scrollbars forward
surface#9.1 · yes · the mobile urlbar's 16px font sits in apps/browser (reached whole) and is among the URL bar citations d63ed2c8c moves; its 8px 6px padding is spacing the Not needed section drops
surface#10.1 · yes · the mobile frame class and the urlbar sizing it drives are URL bar citations d63ed2c8c moves, in apps/browser (reached whole)
surface#11.1 · yes · apps/browser is reached whole, so the Android back-button arm at main.rs:140-145 binds
surface#12.1 · yes · apps/browser is reached whole, so the mobile capture's default file name at capture.rs:109-110 binds
surface#13.1 · yes · the per-OS tallies re-read the test-bearing job logs, and this states that the iOS and Android targets carry no tests, so their builds stay in the fork CI the chunk re-proves
surface#14.1 · no · the Not needed section names the WPT runner's terminal colours and tokens; the delta touches no runner source or colour library
surface#14.2 · no · the Not needed section names the WPT runner's terminal colours and tokens; the delta touches no runner source or colour library
