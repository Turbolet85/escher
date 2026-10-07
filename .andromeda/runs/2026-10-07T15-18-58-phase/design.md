# design extract

## Relevance
partial — the chunk renders nothing and touches no markup, id, class or style of a stand task, so no token, brand or component mandate binds it (the plan's brand, anti-pattern and project-wide token sections read `NO RECORDED INTENT` / `NOT YET MEASURED` and are not cited); what applies is the plan's record of engine paint and hit-test order, the viewport's logical size, font loading on the stand, and scroll motion — the readings `covered`, `off-screen` and the covered fixture rest on.

## Constraints
- **`covered` follows the engine's hit-test order, not a driver-side overlap sum.** design-system §Depth Strategy (Engine depth behavior) records the order a hit test walks an element's children (hoisted positive z-index first, regular paint children in reverse, hoisted negative z-index last). The scope's reading of `covered` — "another element is hit at the point the action would land" — is that walk's answer; whether the executor can reach that hit test through the harness or `DioxusDocument` at a point is research's question.
- **The covered fixture covers by a recorded stacking rule.** design-system §Depth Strategy (Engine depth behavior) lists what makes a node a stacking-context root and records that positioned descendants with `z-index: auto` share one paint level and paint in tree order. The fixture's cover must sit above its target by one of those rules, stated in the fixture, so the refusal is caused by stacking and not by source order alone going unnoticed.
- **`off-screen` compares in one unit space.** design-system §Surface: desktop-native (Tokens) records the viewport's logical size as the physical window size divided by hidpi × zoom, and a default viewport of size (0, 0). The comparison of a target's bounds, or its click point, with the viewport must use the logical size; whether the snapshot's bounds are already in that space, and what the stand's pinned viewport reads through the harness, are research's questions.
- **A fixture's bounds must be real under the stand's font.** design-system §Typography (Loading) records that the stand registers one bundled font for every generic with system fonts off, and that without a font source text measures 0×0 and font-dependent assertions pass vacuously. A fixture "booted with the stand's options" whose target is sized by its text must read non-zero bounds, or a centre-point or viewport reading on it proves nothing.
- **Settle does not wait on scroll motion.** design-system §Motion (Animation runtime) records that scroll animations and scrollbar fades belong to the set that keeps a document animating, and that a harness settle neither waits on nor advances any member of it. This binds decision 2 only: if it lands on an action that brings its target into view itself, the step must leave the target in view at settle. Under the other outcome (a verb joining the table, or no scroll) it does not apply.
- **A smooth scroll is a timed travel.** design-system §Motion (This project's values) records the smooth-scroll duration and curve, that `scroll-behavior: smooth` makes an `Auto` scroll animate, and that dioxus-native-dom's programmatic scroll offers `Smooth` and `Instant`. A driver-made scroll, if decision 2 creates one, must be of the instant kind; whether a stand screen or the fixture sets `scroll-behavior: smooth` is research's question.
- **`disabled` is not read from paint.** design-system §Color Palette (Semantic Colors) records a painted disabled accent for checkbox and radio only. The cause's source stays the snapshot's `enabled` reading named in the scope; no painted colour is a detection input, and the chunk changes no painted state.

## Patterns to follow
- A cover built as a positioned overlay with an explicit `z-index` above its sibling content — the shape the recorded examples use (design-system §Depth Strategy (Observed values in the apps and examples)).
- The stand's font boot — one bundled font registered through the harness options' font context — reused unchanged for the fixture (design-system §Typography (Loading)).
- Time moved only through the harness's controlled clock, advanced by tick; no check waits on wall-clock motion (design-system §Motion (Animation runtime)).
- Fixture style written as literal values in the test file: the plan records no colour token or CSS custom property in the engine and integration crates (design-system §Color Palette (Token sets)), so there is no token for a fixture to cite.

## Anti-patterns to avoid
- The plan records no ban: design-system §Anti-Patterns reads `NO RECORDED INTENT`, and nothing is extracted from it. The two cautions below are read from recorded sections, not from a ban list.
- Do not tell `covered` from rectangle overlap of snapshot bounds: stacking order, not geometry, decides what is hit (design-system §Depth Strategy (Engine depth behavior)).
- Do not let an action scroll smoothly and then act in the same step: the target is still travelling when the step settles (design-system §Motion (This project's values)).

## Contract bindings
- **design ↔ tests** — the font-loading record (design-system §Typography (Loading)) binds to the scope's "no font skip in any check": a bounds-based refusal check is font-dependent.
- **design ↔ tests / architecture** — the settle-and-animation record (design-system §Motion (Animation runtime)) binds to the harness settle contract and the driver's "each acting verb one settled step"; a scroll made by an action would be a new member of that contract's reach.
- **design ↔ a11y** — the painted disabled state (design-system §Color Palette (Semantic Colors)) and the snapshot's `enabled` reading describe the same control state from two sides; the refusal reads the a11y side only (the scope names a11y-plan §7 as its owner).
- **design ↔ layouts** — the pinned stand viewport and which stand screen holds an element outside it are layouts' and research's; this domain contributes only the logical-size rule (design-system §Surface: desktop-native (Tokens)).

## Acceptance criteria contributions
- The covered fixture's target and its cover each read non-zero bounds under the stand's bundled font, asserted before the refusal is asserted, in both layout modes (per design-system §Typography (Loading)).
- The covered fixture states the stacking rule that puts its cover above its target, and the same action with the cover removed (or moved below) is not refused `covered` — the control that shows stacking is what the refusal reads (per design-system §Depth Strategy (Engine depth behavior)).
- An `off-screen` check places its target by the viewport's logical size, and a target inside that size is not refused `off-screen` (per design-system §Surface: desktop-native (Tokens)).
- Only if decision 2 lands on an action that scrolls its target into view: at the step's settle the target's bounds lie inside the viewport, with no smooth scroll in flight (per design-system §Motion (Animation runtime)).
