# Cascade compare — design-system · phase · 2026-10-31-engine-debts
E items 34 · held 15 · lost at classify 1 · lost at strong 2 · beside a placed fact - · cascade only 18 · listed 5 · body 6 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### color-palette#5.1 · lost at strong · .andromeda/design-system-detail/color-palette.md:70
- (.andromeda/design-system-detail/color-palette.md:70) A native-checkedness fix must leave the disabled checkbox and radio paint at this value, since the debt is the tree's stated `toggled` and not the control's colour.
  > | Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO

### color-palette#8.3 · lost at strong · .andromeda/design-system-detail/color-palette.md:104
- (.andromeda/design-system-detail/color-palette.md:104) A native-checkedness fix must leave the unchecked radio ring and checkbox tick colours as they are.
  > Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white

### color-palette#10.1 · lost at classify · .andromeda/design-system-detail/color-palette.md:119
- (.andromeda/design-system-detail/color-palette.md:119) A painted select popup, if the founder's fork takes it, must not cite an engine colour token, because the engine crates hold none; its colours are a new engine default the fork must state.
  > Observed absent — color tokens or CSS custom properties in the engine and integration crates

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### motion#2.2 · listed (lost at classify) · .andromeda/design-system-detail/motion.md:21
- (.andromeda/design-system-detail/motion.md:21) Any fixture or row this chunk adds must treat settle as answering `Settled` without waiting on or advancing animation, and must expect `animating` to read true for 700 ms of the clock after a driver `scroll`.
  > a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass (packages/blitz-test-harness/src/settle.rs:151-186; one member, a CSS animation, exercised at tests/blitz-tests/tests/stand_settle.rs:247-275 — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md)

### motion#4.1 · listed (lost at classify) · .andromeda/design-system-detail/motion.md:33
- (.andromeda/design-system-detail/motion.md:33) A keyboard-activation row on the load page's `load-fetch` Button starts a load, so it must expect the clock to stand still through the load wait, for at most `LOAD_WAIT`.
  > The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait (packages/escher-driver/src/session.rs:16-53; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md)

### surface#15.1 · listed (lost at classify) · .andromeda/design-system-detail/surface.md:122
- (.andromeda/design-system-detail/surface.md:122) Whatever a debt's fix turns from a refusal into an answer (`off-screen`, `covered`, a `lint` reading), the command line must keep this output shape and exit classes, with no new exit, no colour and no ESC byte.
  > `escher-session`, the driver's command line (since 2026-10-10-driver-cli), is a surface an agent reads, not a person: a command prints one line of JSON on stdout — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, and a session error its fixed message on stderr; it uses no colour and no colour dependency, writes no ESC byte on any answer line, and no code asks whether a stream is a terminal, so its output is the same piped or not; the exit status carries the class — `0` accepted · `1` refused · `2` usage · `3` session error

### surface#15.1 · listed (lost at classify) · .andromeda/design-system-detail/surface.md:123
- (.andromeda/design-system-detail/surface.md:123) A checkedness or select-option fix that moves what a snapshot reads (debts 3, 4 and 5) must move only the screen text inside this line, leaving its shape as it stands.
  > `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`

### typography#2.8 · listed (lost at classify) · .andromeda/design-system-detail/typography.md:35
- (.andromeda/design-system-detail/typography.md:35) Every check this chunk adds or extends in the blitz-tests crate must build with `system-fonts`, so that a font-dependent assertion on a text target's bounds or an option's rect fails when it should rather than passing on text measured 0x0.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)

### typography#2.6 · listed (lost at classify) · .andromeda/design-system-detail/typography.md:33
- (.andromeda/design-system-detail/typography.md:33) A debt proven through a driver `screenshot` must run with a font context that resolves `sans-serif`, and must treat a `fonts-unavailable` refusal as no proof, never as a pass and never as a shot with its text skipped.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)

## Held

- color-palette#1.2
- color-palette#1.7
- color-palette#2.3
- color-palette#2.4
- depth-strategy#2.2
- depth-strategy#3.1
- depth-strategy#4.1
- motion#1.4
- motion#1.6
- motion#1.7
- motion#3.1
- surface#1.1
- surface#1.2
- surface#15.2
- typography#1.6

## Cascade only

- color-palette#1.1
- color-palette#1.6
- color-palette#7.3
- depth-strategy#2.1
- depth-strategy#4.2
- motion#1.2
- motion#1.5
- spacing#2.5
- spacing#3.4
- surface#1.4
- typography#1.11
- typography#1.12
- typography#1.7
- typography#2.2
- typography#3.1
- typography#3.10
- typography#3.2
- typography#3.4
