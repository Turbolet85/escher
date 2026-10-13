# design extract

## Relevance
partial — the chunk is engine input behaviour (arrow step, click-by-position, `input` event). Its design surface is the painted range control and its focus ring, the stand's colour record, motion and settle, and the driver CLI's output form. The design plan names no range-input paint at all, so whether blitz paints a track or thumb that follows the value is research's question.

## Constraints
- Engine-painted form controls take their colours from the literals in `blitz-paint`'s form-control painter: a disabled accent of rgba 209,209,209,255, and otherwise the element's `color` standing in for `accent-color` (per design-system §Color Palette, Semantic Colors). Any range paint this chunk touches stays on that model rather than adding a new accent. Whether a range input reaches that painter at all is research's question.
- A focused input's default outline is `#4D90FE`, from the `DEFAULT_CSS` user-agent stylesheet (per design-system §Color Palette, Border Progression). If P4 has a pointer click focus the slider, that outline is what paints, and the chunk adds no new focus colour.
- Every document gets the `DEFAULT_CSS` user-agent stylesheet through `DocumentConfig::default()` (per design-system §Surface: desktop-native, Tokens). Any default range styling belongs there, not in per-app CSS, and the scope rules out a styled track or thumb beyond what blitz already paints.
- The stand's colours are recorded by `stand_contrast`: 18 control rows, every measured colour equal to its declared value, and the accent `#4a6cf7` measured at 4.39 (per design-system §Color Palette, Core Colors). A range change that moves a painted stand colour moves that record.
- A driver `click`, `press` or `advance` leaves the harness's animation clock where it was, and settle never waits on or advances an animation (per design-system §Motion, Animation runtime). A value step must not add a transition or animation on the thumb that keeps the document animating.
- The driver CLI's `escher-session` answers are one uncoloured line of JSON with no ESC byte and no terminal test, and `help` serves the schema's fixed table text the same way (per design-system §Surface: cli, Component Patterns). This applies if the schema's help text on range inputs moves.

## Patterns to follow
- Engine-drawn controls such as the checkbox frame and radio ring are painted in `blitz-paint`'s form-control module, sized by the control scale: the checkbox frame radius is 2 × that scale, unchecked radio rings use the CSS palette GRAY, and checkbox ticks are white (per design-system §Border Radius and §Color Palette, Engine-painted defaults). Follow the same shape if a range thumb is painted or moved.
- Interaction-driven appearance, such as the scrollbar thumb's hover and drag states, is resolved in the engine from computed colours and the viewport colour scheme (per design-system §Color Palette, Engine-painted defaults). This is a precedent if the slider is given any active or focused look.
- The driver's into-view `scroll` is instant inside one settled step (per design-system §Motion, This project's values). A range value change from `click` or `press` follows the same instant, single-step model.

## Anti-patterns to avoid
- Adding a new hard-coded colour literal, or a colour that differs in a state, for the range track or thumb. design-system §Color Palette, Token sets records no engine colour tokens, so each new literal is untracked palette drift outside the `stand_contrast` record.
- Animating the thumb toward its new position with a transition or smooth motion. That would hold the document animating against the settle model (design-system §Motion, Animation runtime), and design-system §Motion, Reduced motion records no reduced-motion handling to suppress it.

## Contract bindings
- design ↔ a11y (SC 1.4.3): the `stand_contrast` record of 18 rows (design-system §Color Palette, Core Colors) is the a11y plan's stand record. A painted colour change on the Timer slider row moves both.
- design ↔ a11y (focus visibility, SC 2.4.3 order): the `#4D90FE` default focus outline (design-system §Color Palette, Border Progression) is what shows if P4 has a click focus the slider. The decision on focus is P4's, and focus order belongs to layouts and a11y, not here.
- design ↔ tests (settle harness): the rule of no animation on a step (design-system §Motion, Animation runtime) binds to `Harness::settle` returning `Settled` without advancing the clock, as read in `stand_settle`.
- design ↔ driver `screenshot`: if blitz paints a thumb, its pixels after a step are what a shot carries (design-system §Typography, Loading notes that the shot is refused `fonts-unavailable` when text cannot be painted). Whether the thumb's painted position follows the value is research's question.

## Acceptance criteria contributions
- `stand_contrast` reads its 18 rows unchanged, row for row, after the range change, and no stand colour moves (per design-system §Color Palette, Core Colors).
- A driver `click` or arrow `press` on the Timer slider returns settled in one step, with the document's animating flag unchanged by the value change (per design-system §Motion, Animation runtime).
- If the schema's help text moves, every `escher-session` answer line, `help`'s included, stays one uncoloured JSON line with no ESC byte (per design-system §Surface: cli, Component Patterns).
- If research finds a range paint path, a disabled range paints with the existing disabled accent rgba 209,209,209,255 and an enabled one from the element's `color`, with no new colour literal (per design-system §Color Palette, Semantic Colors).
