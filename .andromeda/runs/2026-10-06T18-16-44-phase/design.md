# design extract

## Relevance
partial — the chunk builds a non-rendering data model, so no tokens, colours or motion are authored. Its `bounds` and `state` fields still read values that design-system.md describes: the viewport model, the stand's bundled font, animation timing and the stand's colour-coded states. §Brand Identity, §Anti-Patterns, §Self-Validation Protocol and §Design Decisions Log read NO RECORDED INTENT and are not cited.

## Constraints
- Bounds are CSS px in the logical viewport. Per design-system §Surface: desktop-native §Tokens, the viewport's logical size is the physical size divided by hidpi × zoom, and the default viewport is hidpi 1.0, zoom 1.0 and the Light scheme. A snapshot's single coordinate space must use that logical unit, never device pixels. Whether the stand's pinned viewport sets a non-1.0 scale is research's question.
- Text-bearing bounds depend on the font context. Per design-system §Typography (Loading), the seven_guis stand's font context is the bundled DejaVu Sans registered for every generic with system fonts off (`build_single_font_ctx` via `HarnessOptions.font_ctx`). The same section records that blitz-tests text measures 0x0 without `system-fonts`. The snapshot's non-zero-bounds proof must therefore boot through the stand's font context, so text-only boxes do not measure 0x0 or vary by host.
- Bounds must be read at a quiescent frame. Per design-system §Motion (This project's values), smooth scrolls run 300 ms and overlay scrollbars hold for 500 ms then fade over 200 ms. Per §Motion (Animation runtime), active animations keep the document animating, and the test harness advances a controlled clock by tick. A snapshot taken mid-scroll-animation would give scroll-dependent bounds. Settle detection belongs to a later chunk, so the proof should snapshot after the harness pump with no scroll animation in flight.
- Paint-time geometry is not layout bounds. Per design-system §Depth Strategy (Engine depth behavior), paint children and stacking contexts are built after layout and transforms, and the stand's cards carry a box-shadow (§Depth Strategy, Observed values). Whether `bounds` includes transforms, or excludes shadow and outline overflow, needs a stated choice. Whether the existing AccessKit node bounds already apply transforms is research's question.
- Stand state is partly signalled by colour. Per design-system §Color Palette (Semantic Colors), the flight booker's invalid and success states are declared as colour triples. The snapshot's state record (enabled, checked, value, focused) must come from engine and DOM readings, never from painted colour. Proving that fidelity belongs to v010-05.

## Patterns to follow
- Headless stand boot with a pinned, bundled font context: `seven_guis::stand` → `HarnessOptions.font_ctx` = `build_single_font_ctx(DEJAVU_SANS)` (per design-system §Typography, Loading).
- Logical-viewport arithmetic (physical ÷ hidpi × zoom), as the viewport type defines it (per design-system §Surface: desktop-native §Tokens).
- Harness-controlled animation clock advanced by tick, for deterministic frames (per design-system §Motion, Animation runtime).
- The viewport colour scheme defaults to Light, and a scheme change triggers a full recascade (per design-system §Color Palette, engine colour model). Take the snapshot at the stand's default scheme, and do not toggle the scheme during the build.

## Anti-patterns to avoid
- Inferring a state field from a painted colour, such as reading "invalid" from the flight booker's `#e53e3e` border (per design-system §Color Palette, Semantic Colors). This binds to a11y SC 1.4.1, not colour alone.
- Asserting non-zero bounds in a test context without the stand's bundled font, where text can measure 0x0 and font-dependent assertions pass vacuously (per design-system §Typography, Loading).

## Contract bindings
- design §Typography (stand font context) ↔ the tests harness (`HarnessOptions.font_ctx` in blitz-test-harness): the snapshot check's bounds are deterministic only through that option.
- design §Motion (Animation runtime, controlled clock) ↔ the tests harness pump and tick: snapshot timing relative to in-flight scroll animations and scrollbar fades.
- design §Color Palette (Semantic Colors) ↔ a11y §Use of Color SC 1.4.1: state reaches the snapshot as a field, not as colour.
- design §Surface: desktop-native §Tokens (logical viewport) ↔ a11y: the AccessKit node bounds the snapshot may share with the accessibility tree must use the same CSS-px space.

## Acceptance criteria contributions
- Every snapshot node's bounds is expressed in CSS px of the logical viewport (physical ÷ hidpi × zoom), and a stand control's bounds lies inside the pinned viewport's logical size (per design-system §Surface: desktop-native §Tokens).
- The stand snapshot check boots each task through `seven_guis::stand` with the bundled DejaVu Sans font context. Text-bearing controls (for example the counter's button and label) report non-zero width and height (per design-system §Typography, Loading).
- Two snapshots of the same booted task, taken after the harness pump with no scroll animation in flight, give identical bounds (per design-system §Motion, This project's values / Animation runtime).
- No snapshot state field is derived from a computed or painted colour value (per design-system §Color Palette, Semantic Colors).
