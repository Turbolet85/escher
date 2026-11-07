@answer n=1 entity=color-palette#3.7 v=no
why: the session changes only the Dioxus bridge's mutation replay order and touches no background colour value or stylesheet.
@answer n=2 entity=color-palette#4.1 v=no
why: the browser tooltip and status bar colours live in apps/browser/assets/browser.css, which the session does not reach.
@answer n=3 entity=color-palette#4.2 v=no
why: the browser about-page muted greys live in apps/browser assets outside the session's reached files.
@answer n=4 entity=color-palette#4.3 v=no
why: the browser error page paragraph colour in error.html is outside the session's reached files and unchanged.
@answer n=5 entity=color-palette#4.4 v=no
why: the todomvc body and heading colours are in examples/todomvc CSS, which the session does not reach.
@answer n=6 entity=color-palette#4.5 v=no
why: the wasm_hello heading and code colours in examples/wasm_hello/src/lib.rs are outside the session's reached files and unchanged.
@answer n=7 entity=color-palette#5.1 v=no
why: the disabled checkbox and radio accent is painted in blitz-paint form_controls, which the session does not reach and whose colour is unchanged.
@answer n=8 entity=color-palette#6.1 v=no
why: the WPT runner terminal colours in wpt/runner/src/main.rs are outside the session's reached files and unchanged.
@answer n=9 entity=color-palette#6.2 v=no
why: the WPT runner yellow for partial-pass FAIL in wpt/runner/src/main.rs is outside the session's reached files and unchanged.
@answer n=10 entity=color-palette#6.3 v=no
why: the WPT runner red for FAIL in wpt/runner/src/main.rs is outside the session's reached files and unchanged.
@answer n=11 entity=color-palette#6.4 v=no
why: the WPT runner bright red for TIMEOUT in wpt/runner/src/main.rs is outside the session's reached files and unchanged.
@answer n=12 entity=color-palette#6.5 v=no
why: the WPT runner bright black for SKIP in wpt/runner/src/main.rs is outside the session's reached files and unchanged.
@answer n=13 entity=color-palette#6.6 v=no
why: the WPT runner bright magenta for CRASH in wpt/runner/src/main.rs is outside the session's reached files and unchanged.
@answer n=14 entity=color-palette#7.1 v=no
why: the browser urlbar border colour in apps/browser/assets/browser.css is outside the session's reached files and unchanged.
@answer n=15 entity=color-palette#7.2 v=no
why: the browser chrome focus colour in apps/browser/assets/browser.css is outside the session's reached files and unchanged.
@answer n=16 entity=color-palette#7.3 v=no
why: the session changes which node set_focus acts on, not the default.css focus outline colour, which is not reached and still holds.
@answer n=17 entity=color-palette#8.1 v=no
why: the default scrollbar thumb palette in blitz-paint render.rs is not reached and its values are unchanged by the session.
@answer n=18 entity=color-palette#8.2 v=no
why: the scrollbar-color resolution and author thumb blending in blitz-dom and blitz-paint are not reached and unchanged.
@answer n=19 entity=color-palette#8.3 v=no
why: the radio ring and checkbox tick colours in blitz-paint form_controls.rs are not reached and unchanged.
@answer n=20 entity=color-palette#8.4 v=no
why: the debug overlay box colours in blitz-paint debug_overlay.rs are not reached and unchanged.
@answer n=21 entity=color-palette#8.5 v=no
why: the devtools layout stroke colours in blitz-paint render.rs are not reached and unchanged.
@answer n=22 entity=color-palette#8.6 v=no
why: the currentColor substitution in blitz-dom serialize.rs is not reached and unchanged by the session.
@answer n=23 entity=color-palette#9.1 v=no
why: the rdme GitHub markdown colour tokens in apps/readme/assets are outside the session's reached files and unchanged.
@answer n=24 entity=color-palette#10.1 v=no
why: the session adds no colour token or CSS custom property to the reached Rust files, and the task states no token changed, so the absence still holds.
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture palette in examples/assets/clip-path.html is outside the session's reached files and unchanged.
