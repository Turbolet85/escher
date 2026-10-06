# design extract

## Relevance
partial. The chunk is identity and proof work with no painted change intended (scope §Surfaces and contracts touched). Design applies only through three things: the stand's pinned font, which the fresh-process proof relies on; the lean tasks' declared styling, which re-keying and author-id edits must leave intact; and any visible remount affordance P4 might add to the stand.

## Constraints
- design-system §Typography → Loading requires the seven_guis headless stand to register the bundled DejaVu Sans for every generic, with system fonts off, through `HarnessOptions.font_ctx`. A persistence check that boots the stand in a second process must use that same pinned font context, so the two processes lay out the same tree. Whether every boot path already does this (`stand::boot`, a re-exec, or `agent-run.sh`) is research's question.
- design-system §Color Palette → Core Colors / Surface Scale / Text Hierarchy (seven_guis rows) record the lean tasks' accent, page and text values as declared. Re-keying CRUD rows (`crud.rs`) or adding author `id`s to lean-task elements is structural only. It must not change those declared values.
- design-system §Color Palette → Semantic Colors (seven_guis invalid / success rows) records flight booker's invalid and success styling. The scope names a conditionally shown validation message as a possible `{tag}:{n}` shifter. Any fix there (an author key, or a change to the conditional structure) must keep that state's styling and its text message as declared. Whether the message is rendered conditionally today is research's question.
- design-system §Border Radius (seven_guis row), §Typography (seven_guis row), §Spacing (seven_guis cards and home values) and §Depth Strategy (seven_guis card shadow) apply only if P4 adds a visible stand-side remount control. The control must then reuse the seven_guis values recorded there for buttons, base text, card padding and shadow. It must not introduce new ones. Navigating through the app's existing Back/Home path adds no new styling.
- design-system §Color Palette → Token sets records no colour tokens or CSS custom properties in seven_guis ("Observed absent"), and its project-wide token set reads NOT YET MEASURED. The chunk therefore has no token system to bind to. "Uses tokens" here means "reuses the seven_guis values already declared", not `var(--…)`.
- design-system §Motion → Animation runtime records that the test harness drives animation from a controlled clock that `tick` advances. §Motion → This project's values records no transitions or animations in seven_guis styles. A timer-tick re-render check advances state through the harness clock, not through a CSS animation. Whether a timer re-render changes tree shape (and not only text) is research's question.

## Patterns to follow
- Fixed stand presentation: viewport, bundled font and offline boot are fixed for every boot (design-system §Typography → Loading). Use the same `stand::boot` configuration in both processes of the fresh-process check, and in the remount check before and after.
- Inline per-task styles: seven_guis declares its colours, radii and shadows inline in each task file (design-system §Color Palette / §Border Radius / §Depth Strategy cite `examples/seven_guis/src/tasks/*.rs` and `app.rs`). Any visual element added follows that inline convention rather than adding a stylesheet or token layer.
- Windowed / wasm parity: the web-spa surface renders seven_guis into a canvas with the bundled font (design-system §Surface: web-spa → Tokens). A stand-side remount affordance must render the same tree in windowed, wasm and headless builds (scope §Surfaces).

## Anti-patterns to avoid
- Changing the stand's font context, for example by enabling system fonts or swapping DejaVu Sans, to make a check pass. Text then measures differently across hosts and processes, and the fresh-process proof stops being reproducible (per design-system §Typography → Loading).
- Adding a new colour, radius or shadow value for a remount control or keyed row that the seven_guis rows in design-system §Color Palette / §Border Radius / §Depth Strategy do not already record.

## Contract bindings
- Painted colours ↔ a11y: any colour on a new visible control binds to a11y SC 1.4.3 contrast (CLAUDE.md a11y target). Reusing the seven_guis accent and text pair keeps the existing pairing.
- Semantic state colour ↔ a11y: flight booker's invalid and success states pair colour with text (design-system §Color Palette → Semantic Colors). That pairing binds to a11y's not-colour-alone rule if the validation message's structure is touched.
- Stand font ↔ test harness: the pinned font context reaches the stand through `HarnessOptions.font_ctx` (design-system §Typography → Loading). This binds the fresh-process proof to the harness contract (test-plan §3).
- Author ids ↔ layouts: layout-templates records the lean tasks' author ids. New or changed author keys from this chunk are a layout-templates amendment at wrap, which is the layouts distiller's domain, not design's.

## Acceptance criteria contributions
- (design) The diff adds no colour, radius or shadow value to `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud}.rs`, `app.rs` or `stand.rs` beyond the seven_guis values already recorded (per design-system §Color Palette / §Border Radius / §Depth Strategy).
- (design) Every stand boot used by the persistence checks, including the second-process boot, registers the bundled single-font context with system fonts off (per design-system §Typography → Loading).
- (design) If flight booker's validation message or success message is restructured or keyed, its invalid or success state still renders its text label alongside the state colour (per design-system §Color Palette → Semantic Colors).
- (design) Any visible remount control added to the stand reuses the seven_guis button radius, accent and base type recorded in the plan, and renders the same tree in windowed and wasm builds (per design-system §Border Radius / §Color Palette → Core Colors / §Typography / §Surface: web-spa).
