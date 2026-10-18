# design extract

## Relevance
partial — the chunk adds no surface, colour, type or spacing token; it moves the clock under the plan's recorded motion (the overlay scrollbar fade, the animation runtime, scroll and fling timing), whose timings and painted look must come through unchanged.

## Constraints
- The overlay scrollbar fade keeps its recorded shape: full opacity on scroll, opaque for `FADE_DELAY` = 500 ms after the last activity, then a linear fade over `FADE_DURATION` = 200 ms. Only the source of "now" changes, never the timings or the curve (per design-system §Motion, This project's values).
- Two more fade readings must hold on the one clock: a finished fade is dropped on each resolve, and hovering where a hidden thumb would be does not bring it back. Whether moving the stamp and the elapsed reads onto a virtual clock keeps both is research's question (per design-system §Motion, This project's values).
- The windowed shell's animation clock is recorded as seconds since the first animation-time query, with frames redrawing while the document animates. A windowed fade must still play out by wall time unless the P4 founder fork rules otherwise. The harness's controlled clock, moved by `tick` and only read by settle, is the recorded headless counterpart (per design-system §Motion, Animation runtime).
- The plan records that the driver's `advance` moves an app's own time and leaves the harness's animation clock where it was. The chunk's `advance moves the one clock` lean would change that recorded behaviour, so the plan line becomes a reconcile item at wrap. Whether settle still never advances the clock under the new design is research's question (per design-system §Motion, Animation runtime).
- Smooth scrolls keep their 300 ms cubic ease-in-out, and touch flings keep decelerating per frame until velocity drops below 0.1. If either moves onto the one clock, its duration and curve stay as recorded (per design-system §Motion, This project's values).
- The driver's `scroll` stays instant in every box and in the viewport, made inside one settled step, and `scroll_into_view` never animates a nested box. This rule was ratified by the founder (2026-10-07), and no clock change may route a driver scroll through the smooth path (per design-system §Motion, This project's values).
- The pixels the repeat-shot proof compares include the engine-painted default thumb palette (rest, hover and active per colour scheme, fill plus contrast stroke), the thumb geometry (thickness 10.0 or thin 6.0, margin 2.0, minimum length 32.0 CSS px) and the fully rounded thumb radius. The chunk changes none of them (per design-system §Color Palette, Engine-painted defaults; design-system §Spacing, Engine spacing; design-system §Border Radius).

## Patterns to follow
- Settle answers every animating member the same way, scrollbar fades included: none holds it open, none is waited on or advanced, and it returns `Settled` with `animating` read from the document's flag. A fade on the one clock joins that set unchanged (per design-system §Motion, Animation runtime).
- The headless instance already has one controlled animation clock, the harness's, moved by `tick` and handed to style resolution. CSS animations and transitions move from pending to running to finished by that current time. Extend this clock rather than adding a second one beside it (per design-system §Motion, Animation runtime).
- Overlay scrollbars paint only behind the optional `scrollbars` feature, which forwards to blitz-paint. Whether the stand's headless build enables it, so that the proof is not vacuous, is research's question (per design-system §Surface: desktop-native, Platform-Specific Notes).
- When the `screenshot` help sentence about the fade window is rewritten, it keeps the command-line surface's form: one uncoloured JSON line on stdout, no ESC byte, no terminal test, and the image as base64 inside the line (per design-system §Surface: cli, Component Patterns).

## Anti-patterns to avoid
- Do not retune or duplicate the motion constants (`FADE_DELAY`, `FADE_DURATION`, the 300 ms smooth-scroll curve, the 0.1 fling floor) to make a shot repeat. Determinism comes from the clock, not from changed timings (per design-system §Motion, This project's values).
- Do not make a driver `scroll` or `scroll_into_view` animate in order to exercise the clock. The instant driver scroll is a ratified recorded behaviour (per design-system §Motion, This project's values).
- Do not change the thumb palette, geometry or radius as a side effect, for example by painting a proof fixture with an author `scrollbar-color`. The proof reads the default engine-painted thumb (per design-system §Color Palette, Engine-painted defaults).

## Contract bindings
- design §Motion (Animation runtime) ↔ tests: the harness's controlled clock and settle's never-advance reading are the test-plan harness contract. A change to who moves that clock binds both plans and the `Harness::settle` record.
- design §Motion (Animation runtime) ↔ the driver schema: the plan's record that `advance` leaves the animation clock in place is mirrored in the `advance` and `screenshot` help text, the program's own `'static` text on a ratified carrier. The two must move together.
- design §Motion (Reduced motion, observed absent) ↔ a11y: the chunk adds no animation, so no reduced-motion binding is created. SC 2.3.3 is outside the project's a11y target (2.1.1 · 1.4.3 · 2.4.3), and thumb contrast is not on a stand contrast row this chunk moves.
- design §Color Palette (Engine-painted defaults) ↔ blitz-paint: the thumb's opacity at a given clock time is the painted alpha the repeat-shot proof compares.

## Acceptance criteria contributions
- In a headless build that paints overlay scrollbars, two shots taken right after a scroll, with wall time passing between them and no driver advance, are pixel-identical. After the driver moves time past 500 ms the thumb's alpha drops, and past 700 ms the thumb no longer paints (per design-system §Motion, This project's values).
- `FADE_DELAY` reads 500 ms and `FADE_DURATION` 200 ms with a linear fade, and the smooth-scroll and fling timings read as recorded. No motion constant changes in the diff (per design-system §Motion, This project's values).
- The default thumb paints the recorded palette, geometry and radius in the proof's shots, so no colour or geometry change rides the chunk (per design-system §Color Palette, Engine-painted defaults).
- The `escher-session screenshot` answer and `help` stay one uncoloured JSON line with no ESC byte after the help sentence changes (per design-system §Surface: cli, Component Patterns).
