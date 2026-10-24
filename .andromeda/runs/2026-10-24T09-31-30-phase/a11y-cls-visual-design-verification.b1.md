# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint's bevel and scrollbar-thumb helper is paint code in a crate the chunk does not change; its contrast proofs use the harness's restated formula (#1.4)
visual-design-verification#1.2 · no · the default thumb's contrast stroke is blitz-paint painting in a crate the chunk does not change, and no scroll or pointer reading the chunk reaches it
visual-design-verification#1.3 · yes · a thumb's hover and drag are pointer and scroll behaviour in events/pointer.rs and scrolling.rs, both reached by the upstream delta, so the merge must keep scrollbar_drag's reading
visual-design-verification#1.4 · yes · the stand's measured SC 1.4.3 reading (text_contrast, contrast_ratio, stand_contrast's controls and its TEXT_ROWS and STATE_ROWS tables with the exempt rule) is the Colour contrast need the chunk re-measures on the merged engine
visual-design-verification#1.5 · yes · the visual lint's contrast reading calls Harness::text_contrast and must follow the same exempt rule, which the Colour contrast need names
visual-design-verification#1.6 · no · the user-agent select's default box and colours are stylesheet values no chunk code reads, and the stand holds no select, so no contrast table reaches its pair
visual-design-verification#2.1 · no · an observed-absent record of a contrast-check search, which the task's Not-needed list excludes and no chunk measurement resolves
visual-design-verification#2.2 · no · an observed-absent record of a contrast or user-preference media search, which the task's Not-needed list excludes and no chunk measurement resolves
visual-design-verification#3.1 · no · the browser urlbar's focus border sits in apps/browser, an upstream-only surface outside the a11y surface, and focus-ring tokens are Not needed
visual-design-verification#3.2 · no · the new-tab search input's outline removal sits in apps/browser, an upstream-only surface outside the a11y surface, and focus-ring tokens are Not needed
visual-design-verification#3.3 · no · the todomvc focus outline sits in an example the chunk does not touch, and focus-ring tokens are Not needed
visual-design-verification#3.4 · no · the stand inputs' focus border or box-shadow is focus-visible styling that none of SC 2.1.1, 1.4.3 or 2.4.3 reads, and focus-ring tokens are Not needed
visual-design-verification#3.5 · no · the counter and transparent examples' 4px focus outline sits outside the stand and the chunk, and focus-ring tokens are Not needed
visual-design-verification#3.6 · no · the rdme focus-visible outline sits in apps/readme, outside the chunk and the a11y surface, and focus-ring tokens are Not needed
visual-design-verification#3.7 · no · blitz-dom's default input outline is focus-ring styling, and the chunk's focus proofs read focus order and focusability, not outline styling
visual-design-verification#3.8 · no · the google fixture's forced-colours outline switch is fixture markup the sync does not change, and a colour preference the task's Not-needed list excludes
visual-design-verification#3.9 · yes · a focused password input paints its caret at its bullet and its bullets in the text colour, the password mask in painted frames that the chunk must keep through any blitz-paint move
visual-design-verification#4.1 · no · the BBC fixture's prefers-reduced-motion queries are motion preferences the task's Not-needed list excludes, and the sync changes no fixture markup
visual-design-verification#5.1 · no · an observed-absent record of motion and colour-scheme rule searches, which the task's Not-needed list excludes and the chunk does not add
visual-design-verification#6.1 · no · disabled icon buttons at 0.35 opacity sit in apps/browser, an upstream-only surface outside the a11y surface
visual-design-verification#6.2 · no · the WPT runner's status-word print sits in wpt/runner, an upstream-only surface outside the a11y surface
visual-design-verification#6.3 · no · a comment in examples/form.rs about an input's accent colour, in an example the chunk does not touch and stating no rule a stand check reads
visual-design-verification#7.1 · no · the viewport's colour scheme and zoom level are colour and zoom preferences the task's Not-needed list excludes, and the stand pins Light and scale 1
visual-design-verification#7.2 · no · colour-scheme change tracking in stylo_device.rs is a colour preference the task's Not-needed list excludes, and no chunk check exercises a scheme change
visual-design-verification#7.3 · no · scale changes from hidpi or zoom in stylo_device.rs are a zoom preference the task's Not-needed list excludes, and the stand is pinned at scale 1
