# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the UA sheet every document takes through DocumentConfig::default() is what the new fixture pages' lint and text_extent readings are laid out under
surface#1.2 · yes · the select's UA box, its one visible option, border, padding and white background, is where the select checks named in Needs drive their pointer input
surface#1.3 · no · the initial theme is a windowed default in blitz-shell, which the sweep never opens
surface#1.4 · no · the stand and fixture boots pin their own viewport, so no witness reads the shell's (0,0) window-size default
surface#1.5 · no · innerWidth, innerHeight and devicePixelRatio belong to blitz-vibey-script, which the native stand and the new static fixture pages never run
surface#2.1 · no · the macOS browser tabstrip padding is apps/browser chrome the chunk neither reaches nor needs
surface#2.2 · no · the shell's custom-titlebar provider is a windowed surface that no windowed run in the chunk exercises
surface#3.1 · no · the custom widget's HTML opacity layer belongs to a windowed example the chunk neither reaches nor needs
surface#4.1 · no · the devtools overlay is not reached and no witness reads an overlay
surface#5.1 · no · the window title follows the browser tab, a windowed rule in apps/browser and blitz-shell the chunk does not reach
surface#5.2 · no · rdme's window title and theme toggle belong to the markdown viewer, which the chunk neither reaches nor needs
surface#6.1 · no · the browser's transparent macOS titlebar is windowed apps/browser chrome the chunk does not reach
surface#6.2 · no · the transparent example is a windowed example that no windowed run in the chunk exercises
surface#6.3 · no · the window compositing alpha mode is a windowed renderer config in dioxus-native that no windowed run exercises
surface#7.1 · no · the wasm32 canvas is the web-spa surface, which the sweep never touches as it is headless native only
surface#7.2 · no · a re-check every change owes: the reached tests/blitz-tests/Cargo.toml must not turn on dioxus-native's optional scrollbars feature
surface#8.1 · no · the mobile urlbar sizing belongs to the mobile-native surface, which nothing in the chunk builds or runs
surface#9.1 · no · the mobile frame class drives mobile urlbar sizing on the mobile-native surface, which the chunk does not touch
surface#10.1 · no · the Android back button is mobile-native navigation, which the chunk does not touch
surface#11.1 · no · the mobile capture save rule belongs to apps/browser's capture, which is not the driver's screenshot verb and is not reached
surface#12.1 · no · a re-check every change owes: the reached CI scripts must add no mobile test leg, the rule's jobs living in the unreached ci.yml
surface#13.1 · no · the WPT runner's result colours belong to a surface the chunk excludes and does not reach
surface#13.2 · no · the WPT runner's kind, flag and heading colours belong to a surface the chunk excludes and does not reach
surface#13.3 · no · the WPT runner's OSC 8 hyperlinks belong to a surface the chunk excludes, and escher-session's no-ESC-byte rule is a separate one
surface#14.1 · no · paint_bench's output lines belong to examples/paint_bench.rs, which the chunk does not reach
