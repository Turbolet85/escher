# design · Color Palette — category read

## Carried
- (.andromeda/design-system.md:16) The sweep re-proves v020-05's text-contrast reading on the stand (each lean task reading 0 lint findings at boot, `contrast` among them); the stand's accent must stay at the value that passes SC 1.4.3 at rest, on hover and when active, and any fixture or stand check the sweep adds must not move it.
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:27) The figures a stand contrast reading (the lint's `contrast` defect through `Harness::text_contrast`, and `stand_contrast` inside `stand-repeat`) must reproduce: white on the accent 5.47, the hover shade 6.83, the active shade 8.27, every measured stand colour equal to its declared value.
  > measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
- (.andromeda/design-system-detail/color-palette.md:44) The stand's page background the contrast composite reads over, which a stand-based sweep witness must leave as declared.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
- (.andromeda/design-system-detail/color-palette.md:59) The stand's text colour, the other half of every non-control text pair a stand lint reading measures.
  > | seven_guis text | `#1a1a1a` |

## Bindings
- **seven_guis accent and its measured ratios ↔ a11y-plan §Contrast (`stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS`) and the escher-driver lint's `contrast` defect**: the palette names the values and figures; the a11y check and the lint's `Harness::text_contrast` reading assert them, so a stand witness the sweep names for v020-05 reads 0 `contrast` findings only while these values hold — «A11y re-read» owns re-asserting the contrast rows, this chunk only must not move them (.andromeda/design-system-detail/color-palette.md:27)
- **Page `#f5f5f5` and text `#1a1a1a` ↔ the body's accent line**: the stand's surface and text pair complete the composite every stand contrast reading is taken over (.andromeda/design-system.md:16)
