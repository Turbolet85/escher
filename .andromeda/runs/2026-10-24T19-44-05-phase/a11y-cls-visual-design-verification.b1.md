# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint's bevel and thumb helper is a paint surface in files the chunk does not reach, and the formula the chunk asserts is the harness restatement that #1.4 carries
visual-design-verification#1.2 · no · the default scrollbar thumb's contrast stroke is a paint rule in unreached blitz-paint and scrollbars.rs, read by no stand check and no a11y witness
visual-design-verification#1.3 · no · the near-white author thumb's hover and drag change is a scrollbar_drag.rs rule the chunk does not reach, read by no stand check and no a11y witness
visual-design-verification#1.4 · yes · stand_contrast.rs is reached and the chunk re-asserts its control, text and colour-state tables (18, 14 and 19 rows), the exempt rule and the white on #3a5ce5 accent
visual-design-verification#1.5 · yes · the lint's contrast finding shares stand_contrast's exempt rule and threshold, the task's indirect need, so a change to that rule moves the lint too
visual-design-verification#1.6 · yes · the flight booker's select is a stand control the chunk re-reads, and SC 1.4.3 must hold on every non-exempt stand row, so this select's unmeasured text and background pair binds
visual-design-verification#2.1 · no · a historical observed-absent search over the old s05 slices, not a rule over the stand checks, which the task's not-needed scope drops
visual-design-verification#2.2 · no · a historical observed-absent search over the old s11 slices, not a rule over the stand checks, which the task's not-needed scope drops
visual-design-verification#3.1 · no · the browser urlbar focus border is a focus indicator the task drops, since the chunk changes no focus indicator, in an unreached app stylesheet
visual-design-verification#3.2 · no · the new-tab search input's outline removal is a focus indicator the task drops, in an unreached browser stylesheet
visual-design-verification#3.3 · no · the todomvc focus outline of 0 is a focus indicator the task drops, in an unreached example stylesheet
visual-design-verification#3.4 · no · the seven_guis inputs' focus border or box-shadow is a focus indicator the task drops, and flight_booker.rs and temp_converter.rs are not reached
visual-design-verification#3.5 · no · the counter and transparent example outlines are focus indicators the task drops, in unreached standalone examples rather than the stand's counter
visual-design-verification#3.6 · no · the rdme focus-visible outline is a focus indicator the task drops, in an unreached markdown viewer stylesheet
visual-design-verification#3.7 · no · the blitz-dom input focus outline and the iframe, body and html suppression are focus indicators the task drops, and default.css is not reached
visual-design-verification#3.8 · no · the google fixture's forced-colors outline is a focus indicator and a user-preference rule the task drops, in an unreached fixture
visual-design-verification#3.9 · no · the focused caret and selection paint and the password caret placement are focus-indicator and password-masking surfaces the task drops, in unreached blitz-paint
visual-design-verification#4.1 · no · the BBC fixture's prefers-reduced-motion queries are a motion rule the task drops, since no stand check reads motion, in an unreached fixture
visual-design-verification#5.1 · no · a historical observed-absent search over the old s02 slices for reduced-motion and colour-scheme rules, not a rule over the stand checks
visual-design-verification#6.1 · no · the browser's disabled icon buttons at 0.35 opacity are app CSS in an unreached app, and the stand's disabled rows are exempt by the snapshot's enabled reading, not by opacity
visual-design-verification#6.2 · no · the WPT runner's printed status word is a result-line rule in an unreached runner, not a stand colour-state cue
visual-design-verification#6.3 · no · the form example's accent-colour comment is a note in an unreached example, not a stand control
visual-design-verification#7.1 · no · the viewport's colour-scheme and zoom carriage in blitz-traits is a user-preference surface the task drops, since the stand pins Light and scale 1 at boot and no stand check reads either preference
visual-design-verification#7.2 · no · the colour-scheme device rebuild in blitz-dom stylo_device.rs is a user-preference surface the task drops, and the chunk changes no scheme
visual-design-verification#7.3 · no · the scale-change text-shaping invalidation in stylo_device.rs is a zoom surface the task drops, and the stand stays at scale 1
