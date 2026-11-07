@answer n=1 entity=color-palette#3.7 v=no
why: the session changes only the dioxus-native-dom and runtime-probe files and moves no colour value, so the example background claim still holds.
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status text colours sit in apps/browser/assets/browser.css, which the session does not reach and which changed no colour.
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted greys sit in apps/browser about-*.css files the session does not reach and no stylesheet changed.
@answer n=4 entity=color-palette#4.3 v=no
why: the browser error page paragraph colour in apps/browser/assets/error.html is outside the reached files and no stylesheet changed.
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours in examples/todomvc/src/todomvc.css are outside the reached files and no stylesheet changed.
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours in examples/wasm_hello/src/lib.rs are outside the reached files and no colour changed.
@answer n=7 entity=color-palette#5.1 v=no
why: the disabled checkbox and radio rgba values in packages/blitz-paint form_controls.rs are outside the reached files and no colour changed.
@answer n=8 entity=color-palette#6.1 v=no
why: the WPT runner PASS terminal colour in wpt/runner/src/main.rs is outside the reached files and the WPT runner's output did not change.
@answer n=9 entity=color-palette#6.2 v=no
why: the WPT runner yellow terminal colour in wpt/runner/src/main.rs is outside the reached files and the runner's output did not change.
@answer n=10 entity=color-palette#6.3 v=no
why: the WPT runner red terminal colour in wpt/runner/src/main.rs is outside the reached files and the runner's output did not change.
@answer n=11 entity=color-palette#6.4 v=no
why: the WPT runner bright red TIMEOUT colour in wpt/runner/src/main.rs is outside the reached files and the runner's output did not change.
@answer n=12 entity=color-palette#6.5 v=no
why: the WPT runner bright black SKIP colour in wpt/runner/src/main.rs is outside the reached files and the runner's output did not change.
@answer n=13 entity=color-palette#6.6 v=no
why: the WPT runner bright magenta CRASH colour in wpt/runner/src/main.rs is outside the reached files and the runner's output did not change.
@answer n=14 entity=color-palette#7.1 v=no
why: the browser urlbar border #EEE in apps/browser/assets/browser.css is outside the reached files and no stylesheet changed.
@answer n=15 entity=color-palette#7.2 v=no
why: the browser chrome focus colour in apps/browser/assets/browser.css is a stylesheet value the session does not change, and set_focus behaviour does not alter it.
@answer n=16 entity=color-palette#7.3 v=no
why: the blitz-dom default input outline colour in packages/blitz-dom/assets/default.css is outside the reached files and no stylesheet changed.
@answer n=17 entity=color-palette#8.1 v=no
why: the default scrollbar thumb palette in packages/blitz-paint/src/render.rs is outside the reached files, and the session changes no scroll behaviour or colour.
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution in packages/blitz-dom scrollbar.rs and blitz-paint render.rs is outside the reached files and paint is unchanged.
@answer n=19 entity=color-palette#8.3 v=no
why: the unchecked radio ring and checkbox tick colours in blitz-paint form_controls.rs are outside the reached files and no colour changed.
@answer n=20 entity=color-palette#8.4 v=no
why: the debug overlay colours in packages/blitz-paint/src/debug_overlay.rs are outside the reached files and paint is unchanged.
@answer n=21 entity=color-palette#8.5 v=no
why: the devtools layout stroke colours in packages/blitz-paint/src/render.rs are outside the reached files and paint is unchanged.
@answer n=22 entity=color-palette#8.6 v=no
why: currentColor substitution in packages/blitz-dom serialize.rs is outside the reached files, and the change reorders mutations without altering their content.
@answer n=23 entity=color-palette#9.1 v=no
why: the rdme GitHub colour tokens in apps/readme/assets/github-markdown.css are outside the reached files and no token or stylesheet changed.
@answer n=24 entity=color-palette#10.1 v=no
why: the session states no token or custom property changed, and the new buffer module adds none, so the absence claim still holds.
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture palette in examples/assets/clip-path.html is outside the reached files and no fixture changed.
