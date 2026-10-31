# Classified — design-system · color-palette.b2

## Answers
color-palette#4.5 · no · the wasm_hello heading and code colours sit in examples/wasm_hello/src/lib.rs, which the session does not reach
color-palette#5.1 · no · the disabled checkbox and radio accent is painted in packages/blitz-paint/src/render/form_controls.rs, which shows no diff, and disabled reads as before
color-palette#6.1 · no · the WPT runner's PASS green sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#6.2 · no · the WPT runner's yellow colour for FAIL with some passing subtests sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#6.3 · no · the WPT runner's red FAIL colour sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#6.4 · no · the WPT runner's bright red TIMEOUT colour sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#6.5 · no · the WPT runner's bright black SKIP colour sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#6.6 · no · the WPT runner's bright magenta CRASH colour sits in wpt/runner/src/main.rs, which the session leaves unchanged
color-palette#7.1 · no · the browser urlbar border sits in apps/browser/assets/browser.css, which no app stylesheet changed
color-palette#7.2 · no · the browser chrome focus colour sits in apps/browser/assets/browser.css, which no app stylesheet changed
color-palette#7.3 · yes · add-to · F7, the Cells current-cell focus outline in the stand accent #3a5ce5, joins this focus-colour table as its next row beside the browser and default-input focus rows
color-palette#8.1 · no · the default scrollbar thumb colours are painted in packages/blitz-paint/src/render.rs, which shows no diff, and this scrollbar-only list does not claim the engine's defaults as a whole
color-palette#8.2 · no · the scrollbar-color resolution and thumb blend sit in packages/blitz-dom/src/node/scrollbar.rs and packages/blitz-paint/src/render.rs, neither reached, and the scrollbar tests are not among the changed files
color-palette#8.3 · no · the radio ring and checkbox tick colours are painted in packages/blitz-paint/src/render/form_controls.rs, which shows no diff
color-palette#8.4 · no · the debug overlay colours are painted in packages/blitz-paint/src/debug_overlay.rs, which shows no diff
color-palette#8.5 · no · the devtools layout strokes are painted in packages/blitz-paint/src/render.rs and documented in packages/blitz-traits/src/devtools.rs, neither reached
color-palette#8.6 · no · the SVG currentColor replacement sits in packages/blitz-dom/src/node/serialize.rs, which the session does not reach
color-palette#9.1 · no · rdme's GitHub colour tokens sit in apps/readme/assets/github-markdown.css, and the pulldown-cmark lock move changes no output style
color-palette#10.1 · yes · add-to · the default sheet's new chosen-row colour pair is hardcoded with no token, so this engine-wide absence stays true and gains that case
color-palette#11.1 · no · the clip-path fixture palette sits in examples/assets/clip-path.html, which the session does not touch
color-palette#11.2 · no · the border-styles fixture colours sit in examples/assets/border-styles.html, which the session does not touch
color-palette#11.3 · no · the reduced GitHub fixture's border colour sits in examples/assets/github_profile_reduced.html, which the session does not touch
color-palette#11.4 · no · the BBC fixture's relative colour syntax sits in examples/assets/bbc_reduced.html, which the session does not touch
color-palette#11.5 · no · the graphite fixtures' custom properties sit in examples/assets/graphite.html and its subsets, which the session does not touch
color-palette#11.6 · no · the google fixture's text and link colours sit in examples/assets/google.html, which the session does not touch
