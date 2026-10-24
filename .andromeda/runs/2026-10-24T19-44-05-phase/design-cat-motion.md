# design · Motion — category read

## Carried
- (.andromeda/design-system-detail/motion.md:8) The stand's styles must hold no transition or animation, so the hover and active colour states `stand_contrast`'s `STATE_ROWS` re-assert are reached at once and read at one time; a re-read that finds one in a seven_guis style records it before trusting a state row.
  > observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
- (.andromeda/design-system-detail/motion.md:26) A keyboard row the chunk adds through the driver (the select's arrows, `Home` · `End` and typeahead) must move time only by an explicit `advance`, since `press` leaves the clock where it was; a typeahead buffer's 1 s reset on the document's clock is reached by `advance`, never by key presses alone.
  > the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was
- (.andromeda/design-system-detail/motion.md:20) A re-asserted row read after a settle must not depend on an animation having run: a settle neither waits on nor advances any member of the animating set.
  > a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag
- (.andromeda/design-system.md:135) The re-read makes no reduced-motion claim: reduced-motion handling stands absent and is outside the a11y target the chunk re-asserts.
  > Observed absent — reduced-motion handling.

## Bindings
- **Stand styles without transitions ↔ a11y-plan (SC 1.4.3, `stand_contrast` `STATE_ROWS`) and Color Palette (the accent's hover `#2a4cd3` · active `#1f3fc1`)**: a state row's colour is the declared state colour at once only while no transition stands on the stand, (.andromeda/design-system-detail/motion.md:8)
- **`press` leaves the clock ↔ a11y-plan (SC 2.1.1, the select's keyboard row) and the document's clock (the 1 s typeahead buffer)**: a row asserting typeahead afresh after the buffer expires advances time explicitly, (.andromeda/design-system-detail/motion.md:26)
- **Reduced motion absent ↔ a11y-plan (SC 2.3.3 not in the WCAG target 2.1.1 · 1.4.3 · 2.4.3)**: no new a11y-leg check asserts reduced motion, (.andromeda/design-system.md:135)
