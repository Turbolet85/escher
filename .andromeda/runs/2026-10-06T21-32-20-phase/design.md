# design extract

## Relevance
partial — the chunk is a state-reading change in the snapshot data model and paints nothing of its own; design reaches it only through the minimal fixture beside the stand (new checkbox / radio / password markup in `examples/seven_guis/`) and through the optional bridge change, which can alter what every Dioxus app paints.

## Constraints
- design-system §Color Palette → Token sets records no colour tokens or CSS custom properties in the engine and integration crates, and the stand's colours are untokenized literals; a fixture beside the stand therefore has no token set to cite and is required to stay inside the seven_guis values the plan records (per design-system §Color Palette → Core Colors, → Text Hierarchy, → Semantic Colors) or to declare no colour at all and take the engine defaults. The project-wide token set itself is unmeasured in the plan and gives no coverage.
- A checkbox or radio in the fixture takes the engine-painted control defaults — the disabled accent, the unchecked radio ring, the tick/gap colour (per design-system §Color Palette → Semantic Colors "Disabled checkbox/radio", → Engine-painted defaults); the chunk is required not to restyle or re-colour them to make a state reading pass. Whether a fixture needs any author styling at all is research's question.
- Every document gets the blitz user-agent stylesheet (per design-system §Surface: desktop-native → Tokens (platform-specific)); the fixture's form controls are required to render through it, and its default form-control font and focus outline are the recorded baseline (per design-system §Typography "blitz-dom default form controls"; §Color Palette → Border Progression "Focus — blitz-dom default input outline").
- The fixture is required to boot with the stand's font setup — the bundled DejaVu Sans registered for every generic, system fonts off (per design-system §Typography → Loading); the same section records that text measures 0x0 without a font and font-dependent assertions then pass vacuously. Whether the fixture's boot path already inherits `stand::boot`'s font context is research's question.
- If the fixture carries seven_guis-style chrome, the recorded values are base 14px sans-serif (per design-system §Typography "seven_guis"), inputs 4px / buttons 6px / cards 8px radius (per design-system §Border Radius "seven_guis (untokenized)"), card padding 24px 32px with a 16px gap (per design-system §Spacing → Untokenized values in the apps and examples). None of these is a token; a minimal fixture may equally carry no chrome.
- The fixture is required to add no transition or animation: the plan records none in the seven_guis styles and no reduced-motion handling anywhere in the engine slices (per design-system §Motion → This project's values, → Reduced motion), so an animated fixture would have no reduce-motion override to stand on.

## Patterns to follow
- seven_guis styling is declared inline beside the component in each task's Rust source, as literal values (per design-system §Color Palette → Core Colors "seven_guis accent"; §Border Radius "seven_guis (untokenized)") — a fixture placed in `examples/seven_guis/` follows that shape rather than introducing a stylesheet or custom properties.
- The stand's existing state colours are the reference for any state a fixture shows in paint: invalid and success triples (per design-system §Color Palette → Semantic Colors "seven_guis invalid" / "seven_guis success").
- Single-font headless boot through `build_single_font_ctx` is the recorded loading pattern for the stand (per design-system §Typography → Loading).
- Text inputs are built on a fixed-size plain editor and single-line inputs centre vertically in their content box (per design-system §Typography "Text input editors"; §Spacing → Engine spacing) — bounds read for a typed-into field follow from these, not from fixture styling.

## Anti-patterns to avoid
- Reading or proving a state from paint: the plan records that a disabled checkbox/radio differs from an enabled one only by accent colour (per design-system §Color Palette → Semantic Colors "Disabled checkbox/radio") — the snapshot's `enabled` / `checked` are required to come from the state readers the scope names, never from a painted colour, and a proof by pixel is not a state proof.
- Introducing a colour token layer, custom properties or a new palette for the fixture: the plan records none for the engine and integration crates and holds the project-wide set as unmeasured (per design-system §Color Palette → Token sets) — a fixture is not the place to found one.
- The plan's own §Anti-Patterns carries no recorded intent, so no universal design ban is extracted from it.

## Contract bindings
- design ↔ a11y: any colour a fixture declares is a painted colour and so falls under the a11y contrast target; the focus outline the UA stylesheet paints (per design-system §Color Palette → Border Progression) is the visual counterpart of the snapshot's `focused` reading — the two are required to name the same element. Whether they can disagree is research's question.
- design ↔ a11y: the disabled control's painted difference is colour only (per design-system §Color Palette → Semantic Colors); the snapshot's `enabled: Some(false)` is the non-colour carrier of that state for an agent.
- design ↔ architecture (Dioxus DOM bridge): widening the bridge's falsy-clear list changes the attributes every Dioxus document carries, and every document is styled by the UA stylesheet (per design-system §Surface: desktop-native → Tokens (platform-specific)) — so the change can alter painted output (a node the stylesheet hides or styles on attribute presence). Whether the UA stylesheet keys any rule on the presence of `hidden`, `open`, `selected`, `readonly`, `required`, `multiple` or `autofocus` is research's question; the plan does not record it.
- design ↔ tests: the fixture's font context binds to the harness options the stand passes (per design-system §Typography → Loading).
- design ↔ layouts: where the fixture's controls sit and their focus order belong to layout-templates, not to this extract.
- The masked form of a password value has no design coverage: the plan records no masking glyph or password-field rendering.

## Acceptance criteria contributions
- (design) The fixture declares no colour outside the seven_guis values the plan records, or declares none and renders on engine defaults; it adds no custom property (per design-system §Color Palette → Core Colors; §Color Palette → Token sets)
- (design) The fixture boots with the bundled single font and system fonts off, and its controls read non-zero bounds in the snapshot (per design-system §Typography → Loading)
- (design) No `transition`, `animation` or `@keyframes` is added under `examples/seven_guis/` (per design-system §Motion → This project's values)
- (design) If the bridge's falsy-boolean handling changes, the seven tasks' painted output is unchanged for controls whose app state is `false` — or each difference is named and accepted (per design-system §Surface: desktop-native → Tokens (platform-specific))
