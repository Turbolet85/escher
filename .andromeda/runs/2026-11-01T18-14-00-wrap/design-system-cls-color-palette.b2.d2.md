# Classified — design-system · color-palette.b2

## Answers
color-palette#3.7 · no · the example fixture backgrounds sit in examples/custom_widget.rs, examples/html.rs and examples/wgpu_texture/src/styles.css, none of which the take reaches
color-palette#4.1 · no · the browser tooltip and status text values live in apps/browser/assets/browser.css, which the session did not edit
color-palette#4.2 · no · the browser about-page muted greys live in apps/browser assets, which the session did not edit
color-palette#4.3 · no · the browser error page paragraph colour lives in apps/browser/assets/error.html, which the take does not reach
color-palette#4.4 · no · the todomvc colours live in examples/todomvc, which the session did not edit
color-palette#4.5 · no · the wasm_hello colours live in examples/wasm_hello/src/lib.rs, which the session did not edit
color-palette#5.1 · no · the disabled checkbox and radio value sits in blitz-paint's form_controls.rs, which the take does not reach and no hunk names
color-palette#6.1 · no · the WPT PASS terminal colour sits in wpt/runner/src/main.rs, which the take does not reach; the quarantine list holds no colour
color-palette#6.2 · no · the WPT yellow terminal colour sits in wpt/runner/src/main.rs, which the take does not reach
color-palette#6.3 · no · the WPT red terminal colour sits in wpt/runner/src/main.rs, which the take does not reach
color-palette#6.4 · no · the WPT bright red terminal colour sits in wpt/runner/src/main.rs, which the take does not reach
color-palette#6.5 · no · the WPT bright black terminal colour sits in wpt/runner/src/main.rs, which the take does not reach
color-palette#6.6 · no · the WPT bright magenta terminal colour sits in wpt/runner/src/main.rs, which the take does not reach
color-palette#7.1 · no · the browser urlbar border values live in apps/browser/assets/browser.css, which the session did not edit
color-palette#7.2 · no · the browser chrome focus colour lives in apps/browser/assets/browser.css, which the session did not edit
color-palette#7.3 · yes · stale · default.css is a merge surface and the take removed upstream's picture rule from it, so the cited lines 43-46, 98-101 and 111 may shift
color-palette#8.1 · yes · stale · render.rs is a merge surface that gained lines, so the scrollbar palette citation at render.rs:799-818 may move
color-palette#8.2 · yes · stale · render.rs gained lines, so the author-thumb citations at render.rs:820-822 and :857-864 may move
color-palette#8.3 · no · form_controls.rs is not reached and no hunk in the take names the radio ring or checkbox tick paint
color-palette#8.4 · no · debug_overlay.rs is not reached and its fixed overlay colours do not depend on the own-box paint skip
color-palette#8.5 · yes · stale · render.rs gained lines, so the devtools layout stroke citation at render.rs:1398-1405 may move
color-palette#8.6 · no · serialize.rs is not reached and the currentColor serialization it holds is untouched
color-palette#9.1 · no · the rdme GitHub markdown tokens live in apps/readme/assets/github-markdown.css, which the take does not reach
color-palette#10.1 · yes · stale · the absence search covered engine files that are merge surfaces (default.css, render.rs and others) and the report re-reads no absence claim, so it cannot be shown to hold
color-palette#11.1 · no · the clip-path palette lives in examples/assets/clip-path.html, which the take does not reach
