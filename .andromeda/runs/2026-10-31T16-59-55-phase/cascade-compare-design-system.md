# Cascade compare — design-system · phase · 2026-10-31-select-popup
E items 63 · held 37 · lost at classify 1 · lost at strong 1 · beside a placed fact - · cascade only 27 · listed 4 · body 17 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### color-palette#1.4 · lost at strong · .andromeda/design-system-detail/color-palette.md:11
- (.andromeda/design-system-detail/color-palette.md:11) The list opens in every document, so a select inside an iframe must paint its list under the scheme its sub-document holds from the parent.
  > The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:148-152)

### depth-strategy#4.2 · lost at classify · .andromeda/design-system-detail/depth-strategy.md:35
- (.andromeda/design-system-detail/depth-strategy.md:35) If the list takes an outset shadow, its background must be opaque and its opacity 1, or the shadow is clipped under the list.
  > Outset box shadows are clipped when opacity is below 1 or the background is not opaque

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### motion#3.1 · listed (lost at classify) · .andromeda/design-system-detail/motion.md:28
- (.andromeda/design-system-detail/motion.md:28) Opening, choosing in and closing the list by driver `click` or `press` must leave the `VirtualClock` where it was, so the list's state must read true at the settled step with no `advance` owed, and any timed list motion must run only on that clock.
  > settle reads animation time only through that clock, never advancing it; the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was: a driver click beside a running CSS animation returns settled without waiting on it

### surface#15.1 · listed (lost at strong) · .andromeda/design-system-detail/surface.md:126
- (.andromeda/design-system-detail/surface.md:126) The driver's answers about an open list, and any new reading of `covered`, must stay inside this output shape: one uncoloured JSON line on stdout, no ESC byte, no terminal test and the same four exit classes.
  > `escher-session`, the driver's command line (since 2026-10-10-driver-cli), is a surface an agent reads, not a person: a command prints one line of JSON on stdout — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, and a session error its fixed message on stderr; it uses no colour and no colour dependency, writes no ESC byte on any answer line, and no code asks whether a stream is a terminal, so its output is the same piped or not; the exit status carries the class — `0` accepted · `1` refused · `2` usage · `3` session error

### surface#15.1 · listed (lost at strong) · .andromeda/design-system-detail/surface.md:127
- (.andromeda/design-system-detail/surface.md:127) A snapshot whose open list's options read real boxes must keep this answer's form, a longer screen reading in parts by this same line.
  > `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`

### typography#2.6 · listed (lost at classify) · .andromeda/design-system-detail/typography.md:33
- (.andromeda/design-system-detail/typography.md:33) A driver `screenshot` of an open list must honour this guard: with no `sans-serif` family resolved it stays refused `fonts-unavailable`, with no shot that paints the list's text skipped, and the overlay's text adds no path around the check.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1164-1177; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)

### typography#2.8 · listed (lost at classify) · .andromeda/design-system-detail/typography.md:35
- (.andromeda/design-system-detail/typography.md:35) Any `blitz-tests` assertion on the list's text or on boxes sized by it must run with `system-fonts` on, or it passes vacuously on 0x0 text.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)

## Held

- border-radius#1.3
- border-radius#2.1
- color-palette#1.2
- color-palette#1.3
- color-palette#10.1
- color-palette#2.3
- color-palette#2.5
- color-palette#7.3
- color-palette#8.1
- depth-strategy#2.1
- depth-strategy#2.2
- depth-strategy#3.1
- depth-strategy#3.2
- depth-strategy#3.3
- depth-strategy#4.1
- depth-strategy#4.4
- depth-strategy#6.2
- iconography#1.1
- motion#1.5
- motion#1.7
- motion#1.8
- motion#2.2
- motion#6.1
- spacing#2.6
- spacing#3.3
- surface#1.1
- surface#1.2
- surface#1.3
- surface#1.4
- surface#1.5
- surface#1.6
- surface#1.7
- surface#7.1
- typography#1.12
- typography#1.6
- typography#3.4
- typography#5.4

## Cascade only

- border-radius#1.6
- border-radius#1.7
- border-radius#4.1
- border-radius#4.2
- border-radius#4.3
- border-radius#4.4
- border-radius#4.5
- border-radius#4.6
- color-palette#1.1
- color-palette#2.4
- color-palette#2.6
- color-palette#5.1
- color-palette#8.2
- depth-strategy#6.1
- depth-strategy#6.3
- depth-strategy#6.4
- motion#2.1
- surface#1.8
- surface#16.1
- surface#7.2
- typography#2.1
- typography#2.2
- typography#2.3
- typography#3.1
- typography#3.11
- typography#3.2
- typography#5.1
