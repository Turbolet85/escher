# Classified — design · surface.b1

## Answers
surface#1.1 · yes · every stand document is built with the UA sheet once through DocumentConfig::default(), the default layer under the control colours and the select's option reading
surface#1.2 · yes · the flight booker's select row reads the one chosen option this rule leaves displayed, and the rule's white background and border are the defaults where the stand sets none
surface#1.3 · no · the windowed shell's initial theme; the stand boots headless with no window
surface#1.4 · no · the default viewport a window starts from (size 0 by 0, scale 1.0, Light); the stand boots at its own pinned viewport
surface#1.5 · no · the JS window-geometry globals of blitz-vibey-script; the stand is a Dioxus app that runs no script
surface#2.1 · no · the browser app's macOS tabstrip; the chunk reaches no browser app
surface#2.2 · no · the shell's custom-titlebar provider; the headless stand has no window to decorate or drag
surface#3.1 · no · the custom_widget example's canvas at 0.8 opacity; the chunk reads the stand, not the example
surface#4.1 · no · devtools overlays of a hovered or chosen node; the headless checks draw no devtools overlay
surface#5.1 · no · the browser app's window title; the chunk reaches no browser app
surface#5.2 · no · rdme's window title and Ctrl or Cmd+T theme toggle; the chunk reaches no rdme
surface#6.1 · no · the browser app's macOS unified titlebar; the chunk reaches no browser app
surface#6.2 · no · the transparent example's decoration-less window; the chunk reaches no example
surface#6.3 · no · the windowed compositing alpha mode; the chunk runs no window and takes no screenshot
surface#7.1 · no · the wasm32 canvas appended to a page body; the chunk builds no WASM
surface#7.2 · no · the optional scrollbars feature forwards to paint; the chunk's checks read computed styles and the accessibility tree, not paint
surface#8.1 · no · the mobile urlbar font size; the stand is not checked on mobile
surface#9.1 · no · the mobile frame class; the stand is not checked on mobile
surface#10.1 · no · the Android hardware back button; the stand is not checked on mobile
surface#11.1 · no · mobile screenshot file names; the chunk takes no screenshot and the stand is not checked on mobile
surface#12.1 · no · the iOS and Android build jobs in ci.yml; the chunk widens the a11y leg in ci-leg.sh and moves its pin, not those jobs
surface#13.1 · no · the WPT runner's result colours; the chunk adds no CLI output
surface#13.2 · no · the WPT runner's bright-black labels; the chunk adds no CLI output
surface#13.3 · no · the WPT runner's OSC 8 test-name hyperlinks; the chunk adds no CLI output
surface#14.1 · no · the paint_bench output lines; the chunk adds no CLI output
