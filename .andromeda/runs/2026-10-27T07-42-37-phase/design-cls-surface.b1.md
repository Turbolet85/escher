# Classified — design · surface.b1

## Answers
surface#1.1 · yes · every document's UA sheet, added once through DocumentConfig::default(), is what the scope's engine-painted-defaults re-read paints with, and dioxus_document.rs (reached whole) is where upstream #1189 moved the add
surface#1.2 · yes · the UA sheet's select border and background colours and its option display rules are engine-painted defaults the scope re-reads through Stylo where the details styling moves (53e9d81c6)
surface#1.3 · yes · the flight booker's select text (15px, #1a1a1a on #ffffff) is a stand control's text and background pair that stand_contrast re-asserts row for row
surface#1.4 · yes · the initial theme at blitz-shell window.rs:181-182 is the window's theme, and window.rs is reached whole for the winit windows the take moves
surface#1.5 · yes · blitz-traits shell.rs is reached whole, and its default viewport and logical size (physical size over hidpi × zoom) are window-side values the take's winit moves can change
surface#1.6 · yes · blitz-vibey-script runtime.rs is a conflict surface, and its innerWidth, innerHeight and devicePixelRatio lines (1414-1419, 2137-2159) are what the conflict resolution must keep
surface#2.1 · yes · apps/browser is reached whole, and the 44px macOS tab-strip height is no spacing value the Not needed list rules out (only its 90px padding is)
surface#2.2 · yes · blitz-shell lib.rs is reached whole, and its shell provider's minimize, maximize, decorations and window-drag calls are winit window calls the take's winit beta.4 pin moves
surface#3.1 · no · examples/custom_widget.rs is not in the delta, and the task's Not needed names the custom widget pattern
surface#4.1 · no · blitz-traits devtools.rs is not reached (only events.rs and shell.rs are), and no Need or standing check reads a devtools overlay
surface#5.1 · yes · the load wait the scope re-proves after the take reads the load page's in-flight stylesheet and its load-done reveal (stand_load_wait), so the Not needed line's no-page-change is no reason to drop it
surface#6.1 · yes · window.rs:200-206 is reached whole for the shell's title sync, and apps/browser main.rs:166 and :173 (reached whole) set the active tab's title
surface#6.2 · no · apps/readme's window titling and its Ctrl/Cmd+T toggle sit in an app no reached file names, and no Need or standing check reads them
surface#7.1 · yes · apps/browser is reached whole, and its macOS transparent, unified, full-size titlebar (main.rs:82-89) is winit builder calls the take's winit beta.4 pin can move
surface#7.2 · no · examples/transparent is not reached, and the scope names no transparency, decoration-less or close behaviour the take must meet
surface#7.3 · no · dioxus-native config.rs is not reached, and the scope names no compositing alpha mode
surface#8.1 · no · dioxus-native config.rs is not reached, and the scope names no wasm32 canvas
surface#8.2 · yes · dioxus-native Cargo.toml is reached whole, and its scrollbars feature forward sits in the feature table whose default features the take edits
surface#9.1 · yes · its URL bar citation (browser.css:140-161) is one of the apps/browser toolbar, URL bar and favicon citations that d63ed2c8c moves, which the Needs name
surface#10.1 · yes · its URL bar citations (main.rs:48, main.rs:172, browser.css:140-161) are among the apps/browser citations d63ed2c8c moves, which the Needs name
surface#11.1 · yes · apps/browser is reached whole, and the Android back-button navigation at main.rs:140-145 is event handling the take's winit beta.4 pin can move
surface#12.1 · yes · apps/browser is reached whole and capture.rs sits in that reach, and no Need or Not needed line drops the mobile default-name save
surface#13.1 · yes · the fork CI re-proof and the per-OS tallies (test-plan §Per-OS test tallies) witness whether iOS and Android targets are built without tests, and the take's mobile builds stay in that CI
surface#14.1 · no · the WPT runner's result colours sit in wpt/runner/src/main.rs, which no reached file names and no standing check runs, and the task's Not needed names this colour set
surface#14.2 · no · the WPT runner's bright-black tokens sit in wpt/runner/src/main.rs, which no reached file names and no standing check runs, and the task's Not needed names this colour set
