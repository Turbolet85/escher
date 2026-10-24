# design · Color Palette — category read

## Carried
- (.andromeda/design-system.md:16) The contrast re-read must hold the stand accent and its two shades as declared, with white button text on the accent controls clearing SC 1.4.3's 4.5 at rest, on hover and when active.
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:27) `stand_contrast`'s accent rows re-read after 0.2.0 must match these values and ratios on the five named accent controls, the hover and active shades as reached colour states, and every measured stand colour must still equal its declared value; a moved figure is re-recorded, never silently re-baselined.
  > | seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) | Accent on page `#f5f5f5` with text `#1a1a1a`
  > (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183); one step darker since 2026-10-14-stand-a11y-assertions, the operator's choice — until then `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) — in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses reading `rgba(58, 92, 229, …)`, alpha kept; measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
- (.andromeda/design-system-detail/color-palette.md:44) The text outside the controls (`TEXT_ROWS`) is read over the stand page background, which must stay `#f5f5f5`.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
- (.andromeda/design-system-detail/color-palette.md:59) The stand's text colour that `stand_contrast`'s text rows assert must stay `#1a1a1a`.
  > | seven_guis text | `#1a1a1a` |
- (.andromeda/design-system-detail/color-palette.md:71) The flight booker's invalid state is a colour state the contrast re-read reaches; its declared values bind any row that asserts it, and the state must stay stated by `aria-invalid`, not colour alone.
  > | seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state
- (.andromeda/design-system-detail/color-palette.md:72) The flight booker's success message colours bind any contrast row that asserts that message's text over its background.
  > | seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | Success message
- (.andromeda/design-system-detail/color-palette.md:28) A stand control that takes the engine default (a link, an input's focus outline, a button background) is read against these default-stylesheet values, not a stand literal.
  > Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF`

## Bindings
- **seven_guis accent and its shades ↔ a11y-plan §Contrast (SC 1.4.3)**: white `#ffffff` on `#3a5ce5` 5.47, on hover `#2a4cd3` 6.83, on active `#1f3fc1` 8.27 against 4.5 are the figures `stand_contrast`'s rows and `STATE_ROWS` must re-read on the five accent controls (.andromeda/design-system-detail/color-palette.md:27) (.andromeda/design-system.md:16)
- **page and text colours ↔ a11y-plan §Contrast and `stand_contrast`'s `TEXT_ROWS`**: text `#1a1a1a` on page `#f5f5f5` is the declared pair the text outside the controls is measured against (.andromeda/design-system-detail/color-palette.md:44) (.andromeda/design-system-detail/color-palette.md:59)
- **invalid state colour ↔ a11y-plan §Use of Color (SC 1.4.1) and the accessibility tree's `aria-invalid` read**: the invalid colours are paired with the state the flight booker states by `aria-invalid`, so the state is never colour alone (.andromeda/design-system-detail/color-palette.md:71)
- **success message colours ↔ a11y-plan §Contrast**: any contrast row that reaches the success message reads these declared values (.andromeda/design-system-detail/color-palette.md:72)
