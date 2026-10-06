# design extract

## Relevance
partial. The chunk is mostly a non-visual identity and semantics change: the id on AccessKit nodes, plus roles and names. Design applies only where a missing accessible name is fixed in the stand's markup with a visible `<label>` or text, rather than with a non-painting `aria-label`. Whether any stand control lacks a name, and so needs markup, is research's question.

## Constraints
- Any visible label or text added to the lean four or to the shell chrome renders in the seven_guis type. design-system §Typography (seven_guis row) records sans-serif at a 14px base. The chunk introduces no new font family, size or weight.
- On the headless stand, text measures against the bundled DejaVu Sans registered for every generic, with system fonts off, per design-system §Typography → Loading. A visible label's geometry depends on that font, not a system font.
- Visible label text reuses the seven_guis text colour recorded in design-system §Color Palette → Text Hierarchy (seven_guis text), on the page background in §Color Palette → Surface Scale (seven_guis / todomvc page). No new hex value is introduced. The engine and integration crates have no colour-token set (observed absent, §Color Palette → Token sets), so "use tokens" means reusing the seven_guis values already declared.
- A visible label must not add new spacing values. It must fit the seven_guis card and home padding and gap recorded in design-system §Spacing (untokenized values, seven_guis cards and home), so that card layout keeps its declared rhythm.
- The flight booker's invalid and success styling in design-system §Color Palette → Semantic Colors (seven_guis invalid / success) is not recoloured or removed by a naming change.
- Focus painting stays as recorded. The blitz-dom default input focus outline is in design-system §Color Palette → Border Progression (Focus — blitz-dom default input outline). The chunk does not intend to change focusability, so no focus colour changes.

## Patterns to follow
- The seven_guis task styles sit next to their markup in the task files: per design-system §Color Palette → Core Colors (seven_guis accent) and §Typography (seven_guis), they are declared in `examples/seven_guis/src/tasks/*.rs` and `app.rs`. A visible label follows those existing in-file style declarations and adds no new stylesheet.
- A name supplied by existing button text, or by `aria-label`, leaves painted output unchanged. That keeps the stand's recorded colours, type and radii intact: §Color Palette, §Typography, and §Border Radius (seven_guis: cards 8px, buttons 6px, inputs 4px, task cards 6px).

## Anti-patterns to avoid
- Adding a new colour, font size or radius to the stand as a by-product of naming a control. Only the seven_guis values already recorded in design-system §Color Palette, §Typography and §Border Radius apply.
- Carrying the flight booker's invalid or success state in its colour change alone after a markup edit. The colour pairs in design-system §Color Palette → Semantic Colors need a text counterpart (this binds to a11y, below).

## Contract bindings
- Token contrast ↔ a11y §Contrast (SC 1.4.3). Any visible label text uses the seven_guis text and page pair from design-system §Color Palette → Text Hierarchy / Surface Scale, and that pair must meet 4.5:1.
- State colour + label ↔ a11y §Use of Color (SC 1.4.1). The flight booker's invalid and success colours (design-system §Color Palette → Semantic Colors) stay paired with text. An accessible name on those controls must not depend on colour.
- Typography ↔ tests harness. A visible label's layout on the headless stand depends on the bundled DejaVu Sans font context (design-system §Typography → Loading), so a stand check that measures text relies on that pinned font.

## Acceptance criteria contributions
- (design) A visible label or text added to the stand uses only seven_guis values already declared: the text colour, the 14px sans-serif base and the existing spacing. The diff introduces no new hex, px font size or font family (per design-system §Color Palette → Text Hierarchy and §Typography, seven_guis rows).
- (design) Where a control's name comes from `aria-label` or from existing text, that task's style declarations are byte-identical before and after the chunk (per design-system §Color Palette → Core Colors and §Border Radius, seven_guis rows).
- (design) The flight booker's invalid and success styles are unchanged, and each state stays paired with text (per design-system §Color Palette → Semantic Colors).
