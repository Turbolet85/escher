# design extract

## Relevance
partial — the chunk adds no painted surface and no token; design binds it at three points only: the stand's state styling that the new markup states sit beside, the driver command line's written form that carries the new state keys, and the stand's boot conditions the checks read under.

## Constraints
- The stand's invalid state is recorded as a painted colour set, and its success message as another (per design-system §Color Palette → Semantic Colors). The chunk states `invalid` in markup beside that paint; design-system §Color Palette → Semantic Colors requires the declared values to stand as they read — a changed value is a plan amendment, not a side effect of this chunk. Whether the code keeps the CSS class as the style hook once the attribute is added is research's question.
- The plan records no painted value for the flight booker's active mode button or the CRUD's selected row: the stand's rows hold the accent with its hover and active steps, the page and the text colour only (per design-system §Color Palette → Core Colors). What those two states paint today, and whether the accent is what marks them, is research's question — nothing here may be asserted about them from the plan.
- A state stated by attribute on the existing element leaves paint alone; a change of element (decision 2 — a row or a mode control re-made as another tag) brings in the user-agent defaults every document is given (per design-system §Surface: desktop-native → Tokens (platform-specific)), among them the default button background and link rule (per design-system §Color Palette → Core Colors) and the default button radius (per design-system §Border Radius), against the stand's own radii for buttons, inputs and cards (per design-system §Border Radius). Whether any option under decision 2 changes a tag is research's question.
- `escher-session` is a surface an agent reads: one line of JSON on stdout keyed by the driver schema's own words, nothing else there, no colour, no ESC byte, the same output piped or not (per design-system §Surface: cli → Component Patterns). The three states in a diff node's written form are bound by it: schema words as keys, no decoration.
- The stand's styles hold no transition and no animation (per design-system §Motion → This project's values); a state change paints at once. The chunk adds none to the state styling it touches.
- A settle waits on no animation and advances none, and a driver `click`, `type` or `press` leaves the animation clock where it was (per design-system §Motion → Animation runtime). The diff that returns a pressed, selected or invalid node is read on that settled instance — no check may depend on time passing to see the state.
- The stand boots headlessly with one bundled font registered for every generic and system fonts off, and the blitz-tests crate builds the DOM with system fonts on because text otherwise measures 0x0 and a font-dependent assertion passes vacuously (per design-system §Typography → Loading). The before-and-after snapshots and the text-budget reading of each stand screen are taken through the stand's own boot, not a bare document.

## Patterns to follow
- Stand styling is untokenized values declared in each task's own file (per design-system §Color Palette → Core Colors); the plan observes no colour tokens or custom properties in the engine and integration crates (per design-system §Color Palette → Token sets). The chunk follows that as built and introduces no token.
- The command line's answer is keyed by the schema's words and written in one place (per design-system §Surface: cli → Component Patterns); new state keys join that form rather than a second one.
- The stand's checks read answer lines through a shared reader that asserts the no-ESC reading on every line (per design-system §Surface: cli → Component Patterns); a check of the new keys on the command line goes through that reader.
- Engine-painted control state follows the element's state, not an attribute of its own — the disabled checkbox and radio accent is the recorded case (per design-system §Color Palette → Semantic Colors). The builder's new reads add no paint.

## Anti-patterns to avoid
- Colour, an ESC byte, a terminal test or any second line on a command's stdout to mark a state (per design-system §Surface: cli → Component Patterns).
- A transition or animation added to the stand's state styling, which would make the stand the first of its styles to animate and put a painted state behind time a settle does not advance (per design-system §Motion → This project's values; §Motion → Animation runtime).
- A changed or new colour value for the invalid or success state arriving with the markup change (per design-system §Color Palette → Semantic Colors).

## Contract bindings
- design ↔ a11y: the invalid state is recorded as colour (per design-system §Color Palette → Semantic Colors) and the chunk adds its non-colour statement; painted colours are a11y surface, so any option that touches the stand's state styling re-opens the contrast reading. Which criterion binds, and whether a role supports each attribute, is the a11y plan's.
- design ↔ tests: the no-ESC and one-line readings of the command line are asserted by the stand checks' shared reader (per design-system §Surface: cli → Component Patterns); the stand's font and boot conditions (per design-system §Typography → Loading) are the harness's.
- design ↔ architecture / security: the written form's keys are the driver schema's words (per design-system §Surface: cli → Component Patterns); which keys, and whether they sit inside the ratified crossing, is the driver contract's and the security plan's, not design's.
- design ↔ layouts: per-screen structure — a container role for the CRUD list, the rows' place in it — is layout-templates', not extracted here.

## Acceptance criteria contributions
- The stand's declared colour values for the invalid and the success state read the same after the chunk as before it — no colour value in the flight booker's style declarations is added, removed or changed (per design-system §Color Palette → Semantic Colors)
- Every `escher-session` answer line that carries one of the three states is one line of JSON keyed by schema words and holds no ESC byte, piped or not (per design-system §Surface: cli → Component Patterns)
- A search for `transition`, `animation` and `@keyframes` over the stand's style sources reads no match after the chunk, as it does in the plan (per design-system §Motion → This project's values)
- The measured-first snapshots and each stand screen's text-budget reading are taken through the stand's headless boot with its bundled font, and none is taken where text measures 0x0 (per design-system §Typography → Loading)
