# design extract

## Relevance
partial — the chunk builds no surface and changes no token, but it measures the stand's text and background colours and text sizes, which design-system §Color Palette and §Typography record. §Brand Identity, §Spacing, §Depth Strategy, §Border Radius, §Motion and §Iconography do not bear on it.

## Constraints
- Every colour the harness compares has to be read in the engine's colour model. Per design-system §Color Palette (Color model and scheme), the engine's `Color` is `AlphaColor<Srgb>` and Stylo colours convert to it through sRGB. The WCAG relative luminance therefore has to be computed from that sRGB conversion, and any alpha handled explicitly, never dropped. Whether the code already exposes an sRGB-component reader of a computed colour is research's question.
- The background behind a control's text ends at the canvas rule in design-system §Color Palette (Color model and scheme): the canvas background is the root element's background, and body's when the root is transparent. An ancestor-chain reading that reaches a transparent root has to resolve by that rule, not to an assumed white. Whether the stand's root or body states the page colour is research's question.
- The pair is measured under the Light scheme. Per design-system §Color Palette (Color model and scheme), the scheme is an enum whose default is `Light`, it feeds `prefers-color-scheme`, and `light-dark()` and system colours resolve at cascade time. The harness therefore reads computed (post-cascade) colours under the stand's pinned Light boot, and the dark scheme stays out, as the scope says.
- The stand's declared palette is design-system §Color Palette (Core Colors · Surface Scale · Text Hierarchy): accent `#4a6cf7` with hover `#3a5ce5` and active `#2a4cd3`, page `#f5f5f5`, and text `#1a1a1a`. These are the values the stand's stylesheets declare, not measured pairs. The harness's computed reading decides each pair. Whether every stand control takes these values or another one, such as a button's text colour on the accent, is research's question.
- The non-rest colours are design-system §Color Palette (Semantic Colors): seven_guis invalid `#e53e3e` / `#fff5f5` / `#c53030` and success `#ebf8ee` / `#68d391` / `#276749`, "as declared". They belong to states the scope leaves to P3/P4: flight booker invalid, and a selected CRUD row. If P4 measures any of them, the colours are read in that state, not taken from this table.
- A control the stand does not style falls back to the engine's user-agent defaults. Per design-system §Color Palette (Core Colors, blitz-dom default row), the default button background is `#EFEFEF`. Per design-system §Surface: desktop-native (Tokens), every document gets `DEFAULT_CSS` once, through `DocumentConfig::default()`. Whether any stand control's text or background comes from the UA sheet rather than the stand's own CSS is research's question.
- The SC 1.4.3 threshold depends on the text's size and weight, and design-system §Typography records several sizes:
  - the stand's text: seven_guis `sans-serif`, base 14px, home title 700;
  - the stand's font: the headless stand registers bundled DejaVu Sans for every generic, with system fonts off;
  - text-input editors: built with `parley::PlainEditor::new(16.0)`.

  The record has to carry the computed size and weight. Whether a text input's painted value follows the computed `font-size` or the editor's 16.0 is research's question.

## Patterns to follow
- Measure on the stand's own boot, so that text metrics are the bundled font's. Per design-system §Typography (Loading), `seven_guis::stand` registers DejaVu Sans through `HarnessOptions.font_ctx` (`build_single_font_ctx`). Also per design-system §Typography (Loading), a document whose font context resolves no `sans-serif` family is refused a shot before anything is painted. If the raster source is chosen, it inherits that check.
- Read colours after the cascade, not from stylesheet text. Per design-system §Color Palette (Color model and scheme), `light-dark()` and system colours resolve only at cascade time, and the legacy `bgcolor` maps to `background-color`. A computed-style reading catches all of these, and a source-text reading would miss them.
- If P4 chooses the painted raster as the source, account for how the engine paints glyphs. Per design-system §Typography (Engine text mapping), the `font-embolden` feature thickens strokes and turns hinting off. Edge pixels of a glyph are therefore not the text colour, and a raster sample has to target pure text or background pixels.
- Record the result in the form the plan records colours: a value with its provenance. Per design-system §Color Palette, each row carries its value and its source location. The per-pair record names the control's stable id, both colours and their source (computed style, or raster), so a later wrap can amend §Color Palette with measured pairs.

## Anti-patterns to avoid
- Do not settle a pair from a declared hex. The stand rows of design-system §Color Palette (Core Colors · Semantic Colors) are declarations "as declared", so the scope's `#ffffff` on `#4a6cf7` estimate stays a hypothesis until the harness reads it.
- Do not treat the stand's colours as design tokens, and do not re-colour one here. Per design-system §Color Palette (Token sets), colour tokens and CSS custom properties in the engine and integration crates are observed absent. A failing pair is recorded, and its fix belongs to a later entry.
- Do not sample an anti-aliased or emboldened glyph edge as the text colour (per design-system §Typography, Engine text mapping). Note: design-system §Anti-Patterns reads NO RECORDED INTENT, so no universal design ban applies.

## Contract bindings
- Token contrast ↔ a11y: every pair the harness measures is checked against SC 1.4.3 (4.5:1, or 3:1 for large text), and the a11y-plan §3 and §6 readings ("observed absent" and "NOT YET MEASURED") are what the wrap amends. The colours come from design-system §Color Palette and the threshold from the a11y plan.
- Colour ↔ test harness: if the raster source is chosen, the colours are read through `Harness::screenshot_rect` and composited over opaque white. A partly transparent background then blends onto white, not onto the canvas colour from design-system §Color Palette (Color model and scheme), and the record has to say which one it measured.
- State colour + label ↔ a11y SC 1.4.1 (not colour alone): the flight booker's invalid colours in design-system §Color Palette (Semantic Colors) sit beside its `aria-invalid` state. SC 1.4.1 is outside the WCAG tier, so this is flagged only, with no check owed here.

## Acceptance criteria contributions
- Every measured pair records its text colour and background colour as sRGB values read from the engine's `AlphaColor<Srgb>` model, with alpha stated. The background is resolved up the ancestor chain to the canvas rule: the root's background, else body's when the root is transparent (per design-system §Color Palette).
- Every pair is measured on the stand booted under the Light scheme, from computed (post-cascade) colours or the painted raster, never from a stylesheet's declared hex. The recorded colours are set beside the declared seven_guis values (`#1a1a1a` text, `#f5f5f5` page, `#4a6cf7` accent), and any difference is noted (per design-system §Color Palette).
- Every pair record carries the text's computed font size and weight on the stand's bundled DejaVu Sans boot. Large text is decided from those values, and never from the seven_guis base 14px assumed (per design-system §Typography).
