# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint's border-bevel and scrollbar-thumb ratio helper is not reached (blitz-paint untouched), and the chunk reads the harness's restated ratio without re-asserting a formula
visual-design-verification#1.2 · no · default scrollbar thumb strokes are paint in blitz-paint and scrollbars.rs, neither reached, and a thumb is not text, so SC 1.4.3 does not reach it
visual-design-verification#1.3 · no · a near-white thumb's hover and drag change is a scrollbar interaction in scrollbar_drag.rs, not reached and outside the sweep's requirements
visual-design-verification#1.4 · yes · the lint's contrast witness must read stand_contrast's exempt rule (exempt exactly where snapshot enabled reads Some(false), None unmeasured) and its 3.0 or 4.5 threshold by text size, and stand_contrast runs in the a11y leg that ci-leg.sh keeps intact; its row counts stay with the A11y re-read entry
visual-design-verification#1.5 · yes · the visual lint's contrast reading in lint.rs (direct text or a text control read through text_contrast, reported below required_ratio, exempt where enabled reads Some(false), a None reading unmeasured, a contrast finding written with id, defect, bounds, ratio and required; no contrast finding at boot on the four lean tasks) is what the sweep's lint witnesses and fixtures test, and the a11y leg's 43 tests over 6 targets must stay intact
visual-design-verification#1.6 · no · the user-agent select's default border, padding and background (default.css) are not reached, its text/background pair is unmeasured, and the stand holds no select
visual-design-verification#2.1 · no · a search absence of contrast checks over the 15 s05 files binds no reached file, and the contrast readings the chunk uses (1.4, 1.5) are already present
visual-design-verification#2.2 · no · user-preference media is an area the sweep's task names as not needed, and contrast handling is witnessed by the harness reader and the lint (1.4, 1.5), not by this absence
visual-design-verification#3.1 · no · the browser urlbar focus border and outline (apps/browser/assets/browser.css) are not reached, and focus ring is an area the sweep's task names as not needed
visual-design-verification#3.2 · no · the new-tab search input's focus border (apps/browser/assets/about-newtab.css) is not reached, and focus ring is outside the sweep
visual-design-verification#3.3 · no · todomvc's `:focus` outline reset (examples/todomvc/src/todomvc.css) is not reached, and focus ring is outside the sweep
visual-design-verification#3.4 · no · seven_guis's focus border and box-shadow (tasks/temp_converter.rs, tasks/flight_booker.rs) are not reached, and focus ring is outside the sweep; the chunk reads the flight booker's states, not its focus styling
visual-design-verification#3.5 · no · the counter and transparent 4px focus outlines sit in examples not reached, and focus ring is outside the sweep
visual-design-verification#3.6 · no · the rdme stylesheet's focus-visible outline (apps/readme/assets/github-markdown.css) is not reached, and focus ring is outside the sweep
visual-design-verification#3.7 · no · the blitz-dom default stylesheet's input focus outline (default.css) is not reached, and focus ring is outside the sweep, its criterion (SC 2.4.7) not in the target
visual-design-verification#3.8 · no · the google fixture's forced-colors focus outline is user-preference media, which the sweep does not reach, and the file is not reached
visual-design-verification#3.9 · yes · the password clause binds: the chunk needs the password mask painted as bullets with the caret placed by the bullets, and a focused password input paints its caret at the bullet of its caret index, in caret colour, its bullets in text colour, the field scrolling only to keep the caret in its content box (password_painted_masked); the generic caret and selection clause is focus styling the sweep does not reach
visual-design-verification#4.1 · no · the BBC fixture's prefers-reduced-motion queries (examples/assets/bbc_reduced.html) are not reached, and motion is an area the sweep's task names as not needed
visual-design-verification#5.1 · no · a search absence of prefers-reduced-motion and prefers-color-scheme rules over the 21 s02 files binds no reached file, and motion and colour scheme are outside the sweep
visual-design-verification#6.1 · no · the browser's 0.35-opacity disabled icon buttons (apps/browser/assets/browser.css) are not reached, and the stand's disabled rows are exempt by snapshot enabled, not by opacity (1.4)
visual-design-verification#6.2 · no · the WPT runner's status word beside its colour (wpt/runner/src/main.rs) is not reached, and a colour-only cue (SC 1.4.1) is outside the sweep's target
visual-design-verification#6.3 · no · the comment on accent-color in examples/form.rs is not reached and states no rule the chunk must hold
visual-design-verification#7.1 · no · the viewport's colour scheme and zoom (blitz-traits shell.rs) are areas the sweep's task names as not needed, and the stand's contrast pins Light at scale 1 (1.4)
visual-design-verification#7.2 · no · colour-scheme change tracking in stylo_device.rs is not reached, and colour scheme is outside the sweep
visual-design-verification#7.3 · no · hidpi or zoom invalidating text shaping (stylo_device.rs) is not reached, zoom is outside the sweep, and the stand pins scale 1 (1.4), so no reading the chunk adds meets a scale change
