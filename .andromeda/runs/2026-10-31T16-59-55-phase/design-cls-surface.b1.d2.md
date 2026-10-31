# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the list's new defaults go in default.css, which reaches every document only through this one user-agent sheet added by DocumentConfig::default()
surface#1.2 · yes · the closed single select's border, padding, background and chosen-option label set the list's width, anchor and the label it shows
surface#1.3 · yes · the scope names the multiple-select rows as built and must not move what they paint, and this rule is their row layout
surface#1.4 · yes · the scope names the multiple-select rows as built, and this is their chosen-row colour, the same state the list's chosen option paints
surface#1.5 · yes · the group label is this chunk's to paint or read, and this rule sets how a single select's optgroup and its chosen option lay out
surface#1.6 · yes · states the gap the chunk closes: no popup, Enter and Space open nothing, group label unpainted, size unread
surface#1.7 · yes · the list's colours are read against the colour scheme, and the contrast rows measure at the window's default Light theme
surface#1.8 · yes · the new option bounds, screenshot pixels and contrast rows are measured at this default viewport, size and scale
surface#1.9 · no · JavaScript innerWidth, innerHeight and devicePixelRatio are host-script values the engine-painted list does not read
surface#2.1 · no · macOS browser tabstrip merged-titlebar is browser chrome, which the chunk does not touch
surface#2.2 · no · the shell provider's window controls for custom titlebars are window chrome, outside the list
surface#3.1 · no · the custom_widget example's own 0.8 opacity is an example fixture value, not an engine default
surface#4.1 · no · devtools overlays of content, padding, border and margin are not reached or named by the chunk
surface#5.1 · no · the browser window title following the active tab is browser chrome
surface#5.2 · no · the rdme app's own title and theme toggle are app behaviour, not an engine default
surface#6.1 · no · the browser's macOS transparent titlebar is browser chrome
surface#6.2 · no · the transparent example's own 360x300 window is an example fixture
surface#6.3 · no · window compositing alpha mode is not named by the scope and the list is not painted over a transparent window
surface#7.1 · yes · the scope names the list on the WASM canvas surface, which is a canvas appended to the page body and sized from host CSS
surface#7.2 · yes · the new nested scrolling box for a long list paints its scrollbars through blitz-paint, which the optional scrollbars feature forwards
surface#8.1 · no · the mobile urlbar input's font and padding are browser chrome
surface#9.1 · no · the mobile frame class driving urlbar sizing is browser chrome
surface#10.1 · no · the Android hardware back button is browser navigation
surface#11.1 · no · mobile screenshot default file names are browser capture, not the driver's screenshot
surface#12.1 · yes · the scope requires the list to paint in every renderer, and this states that iOS and Android are built but not tested in CI
