# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the helper is blitz-paint's (color.rs, border.rs), no engine crate moved, and the session reaches none of those files
visual-design-verification#1.2 · no · the default thumb contrast stroke lives in blitz-paint and scrollbars.rs, neither reached, and no engine crate moved
visual-design-verification#1.3 · yes · add-to · F4: right after a scroll a main-button pointerdown inside the shown overlay bar's band is taken as a thumb drag and dispatches nothing, the bar stays shown until an advance passes its fade, and a pointer resting on the thumb keeps it shown (stand_cells_served)
visual-design-verification#1.4 · yes · stale · stand_contrast's control rows go from 18 to 17 (pass 14 to 13, fail 0, exempt 3, no-text 1 unchanged) and STATE_ROWS from 19 to 18 (pass 18 to 17), the flight-one-way and flight-return accent rows give way to flight-type (#1a1a1a on #ffffff, 17.40), and the cited stand_contrast.rs lines moved
visual-design-verification#1.5 · yes · add-to · F3: the boot lint now reads one overlap on the flight booker (its two options carry the select's client rect), so the lean stand no longer reads 0 findings, the defect stays unresolved, and stand_lint is re-pinned to it; the a11y leg's count also moved from 84 to 85 tests, so the 84 here is stale too
visual-design-verification#1.6 · yes · contradict · the stand now holds a select (flight-type, #1a1a1a on #ffffff, 17.40) whose text and background pair is measured, and stand_contrast's tables moved, so the claim that the stand holds no select and its tables are unmoved no longer holds
visual-design-verification#2.1 · no · the session adds no contrast check to any searched file (its contrast change is new rows in the existing stand_contrast table), so contrast checks stay absent
visual-design-verification#2.2 · no · the session adds no prefers-contrast or user-preference media rule (the select's focus rule and the load page's accent colours are plain rules), so the absence stands
visual-design-verification#3.1 · no · apps/browser/assets/browser.css is not reached and no engine crate moved
visual-design-verification#3.2 · no · apps/browser/assets/about-newtab.css is not reached
visual-design-verification#3.3 · no · examples/todomvc/src/todomvc.css is not reached
visual-design-verification#3.4 · yes · add-to · the flight booker's select drops the outline for the #3a5ce5 border as the date inputs do, so the select joins the seven_guis inputs that replace the outline, and flight_booker.rs changed in its markup and CSS, so the cited 183-186 may have moved
visual-design-verification#3.5 · no · examples/counter/src/app.rs and examples/transparent/src/app.rs are not reached
visual-design-verification#3.6 · no · apps/readme/assets/github-markdown.css is not reached
visual-design-verification#3.7 · no · blitz-dom's default.css is an engine asset and no engine crate moved, and the select's own :focus rule does not change the default outline
visual-design-verification#3.8 · no · examples/assets/google.html is not reached
visual-design-verification#3.9 · no · blitz-paint's caret and selection code and password_painted_masked.rs are not reached, and a select is not a text input
visual-design-verification#4.1 · no · examples/assets/bbc_reduced.html is not reached
visual-design-verification#5.1 · no · the session adds no prefers-reduced-motion or prefers-color-scheme rule and no engine crate moved
visual-design-verification#6.1 · no · apps/browser/assets/browser.css is not reached, and the retired .type-btn-active state is the stand's, which this line does not name
visual-design-verification#6.2 · no · wpt/runner/src/main.rs is not reached
visual-design-verification#6.3 · no · examples/form.rs is not reached
visual-design-verification#7.1 · no · packages/blitz-traits/src/shell.rs is not reached and no engine crate moved
visual-design-verification#7.2 · no · packages/blitz-dom/src/stylo_device.rs is not reached and no engine crate moved
visual-design-verification#7.3 · no · packages/blitz-dom/src/stylo_device.rs is not reached and no engine crate moved
