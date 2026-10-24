# design extract

## Relevance
partial — the contrast re-read reads the stand's declared colours (design-system's domain); the Tab-order, Enter/Space and a11y-leg work carry no design tokens.

## Constraints
- design-system §Color Palette › Core Colors requires the seven_guis accent to stay `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, white button text on the accent controls passing SC 1.4.3's 4.5 at rest, on hover and when active. The re-read reads these pairs again. It changes no colour.
- design-system §Color Palette › Core Colors records every measured stand colour as equal to its declared value. A re-read row that reads a colour other than the declared one is a finding to name, never a re-baseline. Whether 0.2.0's engine changes (the 2026-10-24 upstream cherry-picks included) moved any computed stand colour is research's question.
- design-system §Color Palette › Text Hierarchy names the seven_guis text `#1a1a1a`, and design-system §Color Palette › Surface Scale (elevation hierarchy) names the seven_guis page `#f5f5f5`. These are the declared values for the text-outside-controls rows (`TEXT_ROWS`) re-read here.
- design-system §Color Palette › Semantic Colors names the seven_guis invalid state (`#e53e3e` / `#fff5f5` / `#c53030`) and success state (`#ebf8ee` / `#68d391` / `#276749`) as declared in the flight booker. Research must answer whether `STATE_ROWS` reaches both states, and which of these values act as text and which as background.
- design-system §Typography gives seven_guis a `sans-serif` family at a 14px base (home title 36px / 700). It records that the headless stand registers bundled DejaVu Sans for every generic, with system fonts off. Size and weight decide each row's required ratio, so a re-read row has to read the stand's own font context.
- design-system §Typography lists blitz-dom's default form-control family as `system-ui, sans-serif`, and design-system §Color Palette › Core Colors lists blitz-dom's default button background as `#EFEFEF`. These are the engine defaults a stand control falls back to where its task CSS declares nothing. Research must answer whether the flight booker's `select` (the candidate new keyboard row) declares its own colours or reads these defaults.

## Patterns to follow
- design-system §Color Palette › Core Colors keeps the measured ratios beside the declared colours (5.47 at rest, 6.83 on hover, 8.27 when active, and the old shade's 4.39 as the recorded fail). A re-read that confirms a row or moves one writes the ratio and its source chunk in the same way, citing `stand_contrast`'s rows.
- design-system §Color Palette › Core Colors recorded the 2026-10-14 accent re-colour with its prior value and the operator's choice named. Any colour-state row this chunk re-records gets the same change-named record.

## Anti-patterns to avoid
- design-system §Anti-Patterns (NEVER do these) reads `> NO RECORDED INTENT`, so no design ban is cited. Within design-system §Color Palette › Core Colors, the old accent `#4a6cf7` is the recorded fail (4.39). A re-read must not restore it, or any shade that reads under 4.5 for white text.

## Contract bindings
- Token contrast ↔ a11y §Contrast (SC 1.4.3). design-system §Color Palette › Core Colors / Text Hierarchy / Semantic Colors hold the declared pairs, and `stand_contrast` (`TEXT_ROWS`, `STATE_ROWS`, through `Harness::text_contrast`) is the harness that measures them. The ratio threshold (4.5, or 3.0 for large text) belongs to a11y. design-system §Typography's sizes and weights feed `TextContrast::required_ratio`.
- State colour ↔ a11y §Use of Color (SC 1.4.1). design-system §Color Palette › Semantic Colors gives the flight booker's invalid and success colours. The stand also states invalid by `aria-invalid` (CLAUDE.md §Modules, seven_guis). Whether every coloured state on the stand carries a non-colour signal is the a11y extractor's binding to confirm.

## Acceptance criteria contributions
- (design) Every non-exempt stand text/background pair re-read by `stand_contrast` equals its declared colour and meets SC 1.4.3 in both layout modes. The five accent controls read white on `#3a5ce5` / `#2a4cd3` / `#1f3fc1` (per design-system §Color Palette › Core Colors).
- (design) The text-outside-controls rows read `#1a1a1a` on `#f5f5f5` as declared (per design-system §Color Palette › Text Hierarchy).
- (design) Any invalid- or success-state row the re-read reaches reads the flight booker's declared values with no colour drift (per design-system §Color Palette › Semantic Colors).
- (design) No stand task CSS colour value is changed by this chunk, and a row whose measured colour moved is reported with the change named, not re-baselined (per design-system §Color Palette › Core Colors).
