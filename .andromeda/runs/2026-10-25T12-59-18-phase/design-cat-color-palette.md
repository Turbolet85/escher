# design · Color Palette — category read

## Carried
- (.andromeda/design-system-detail/color-palette.md:27) The flight booker's one-way / return `select` and any stand screen the chunk serves (Cells) must take the stand's accent, page and text literals as declared, not new hex values.
  > | seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) | Accent on page `#f5f5f5` with text `#1a1a1a`
- (.andromeda/design-system-detail/color-palette.md:27) `flight-one-way` is one of the five measured accent controls; replacing the two buttons with a `select` moves this measured set and its `stand_contrast` rows. The re-pinned rows must still pass SC 1.4.3 for every accent control that remains, at rest, on hover and when active, and every measured stand colour must still equal its declared value.
  > in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses reading `rgba(58, 92, 229, …)`, alpha kept; measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
- (.andromeda/design-system.md:16) The body's accent invariant binds every task CSS the chunk touches (flight booker, and Cells if it is served): accent `#3a5ce5` with its hover and active shades, so white text on any accent control clears 4.5 in all three states.
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:44) A stand screen the chunk serves to the driver must keep the stand's page background.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
- (.andromeda/design-system-detail/color-palette.md:59) Stand text on the new `select` and on any newly served screen must stay at the stand's text colour.
  > | seven_guis text | `#1a1a1a` |
- (.andromeda/design-system-detail/color-palette.md:71) The flight booker's invalid state colours must hold while the one-way / return control changes. They are reached colour states of the contrast record, and the state stays stated by `aria-invalid`, never by colour alone.
  > | seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |
- (.andromeda/design-system-detail/color-palette.md:72) The flight booker's success message colours must hold through the change, so its rows in the contrast record keep their measured pairs.
  > | seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | Success message (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |

## Bindings
- **seven_guis accent ↔ a11y-plan (SC 1.4.3, `stand_contrast` `TEXT_ROWS` / `STATE_ROWS`)**: the accent's measured controls name `flight-one-way`. When the select replaces it, the contrast rows are re-pinned, every remaining accent control still clears 4.5 at rest, on hover and when active, and any new control's text and background pair is measured (.andromeda/design-system-detail/color-palette.md:27) (.andromeda/design-system.md:16)
- **seven_guis invalid / success ↔ a11y-plan (not colour alone) and the snapshot's `invalid` reading**: the flight booker's invalid colours go with `aria-invalid`, which the snapshot reads, so the state is never conveyed by colour alone (.andromeda/design-system-detail/color-palette.md:71)
- **seven_guis page / text ↔ Surface: web-spa / desktop-native**: the canvas page `#f5f5f5` is the same literal the surfaces map, and a served Cells screen keeps it (.andromeda/design-system-detail/color-palette.md:44)
