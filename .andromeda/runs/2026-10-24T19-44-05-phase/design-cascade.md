# design · cascade read

## Carried
- (.andromeda/design-system-detail/color-palette.md:8) Each re-asserted contrast row must read the colour Stylo computes, converted through sRGB to this type, against the declared hex it records.
  > The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB
- (.andromeda/design-system-detail/color-palette.md:27) White button text on the five accent controls must still pass SC 1.4.3 on the accent at rest, on hover and when active after 0.2.0, each pair re-read as a `stand_contrast` row, and every measured stand colour must still equal its declared value.
  > measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
- (.andromeda/design-system.md:16) The body's accent rule must hold on every task CSS the re-read reaches.
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:28) A stand control whose pair falls through to the default stylesheet must be re-read against these engine colours, not a stand token.
  > Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF`
- (.andromeda/design-system-detail/color-palette.md:99) A Tab-focused stand input carries the engine's default focus outline, so a focus-state row, where one reads a colour, must read this value.
  > | Focus — blitz-dom default input outline | `#4D90FE` |
- (.andromeda/design-system-detail/color-palette.md:44) The page background must stay the backdrop of the text outside the controls that `TEXT_ROWS` re-reads.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
- (.andromeda/design-system-detail/color-palette.md:59) The stand's text colour must stay the foreground of the text-outside-controls rows, each passing SC 1.4.3 on the page background.
  > | seven_guis text | `#1a1a1a` |
- (.andromeda/design-system-detail/color-palette.md:71) Where a state row reaches the flight booker's invalid state, its pair must read these declared invalid colours and pass SC 1.4.3 unless exempt.
  > | seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state
- (.andromeda/design-system-detail/typography.md:18) The stand's base size and title weight must decide each row's required ratio — 3.0 for large text, else 4.5.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated |
- (.andromeda/design-system-detail/typography.md:16) A stand heading a text row reads takes the default sheet's bold and em size, so its large-text threshold must be judged from those.
  > | blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated |
- (.andromeda/design-system-detail/typography.md:30) Every re-asserted keyboard and contrast row must be measured with the headless stand's bundled font and system fonts off.
  > The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature
- (.andromeda/design-system-detail/typography.md:31) The stand's boot must keep registering its one font as the fallback for every generic, so text in the re-read rows is measurable.
  > `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled
- (.andromeda/design-system-detail/typography.md:36) The blitz-tests build the re-read runs under, the `a11y` leg's widened set included, must keep blitz-dom's `system-fonts` feature, or text and contrast assertions pass vacuously.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace
- (.andromeda/design-system.md:48) The body states the same loading rule at the broad view, and it binds every check the leg runs.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.
- (.andromeda/design-system-detail/depth-strategy.md:26) A synthesized pointer that drives a hover, active or click row must reach its control through hit-testing that stops at a paint-clipping box and skips a dropped node.
  > hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1568-1632), skipping a node dropped since the last resolve as the paint tree's walk does (architecture §Standard Contracts → Scrolling, selection, tree), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted
- (.andromeda/design-system.md:87) The body's hit-stop rule binds the same reads.
  > Hit-testing stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, and the node itself is still answered inside its border box.
- (.andromeda/design-system-detail/motion.md:20) Each keyboard and state row must read after a settle that holds open for, waits on and advances no member of the animating set.
  > a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag
- (.andromeda/design-system.md:127) The body states the same settle rule.
  > A harness settle answers every member of the document's animating set the same way — none holds it open, none is waited on or advanced: it returns `Settled`, `animating` reading the document's animating flag.
- (.andromeda/design-system-detail/motion.md:26) A select typeahead row must run on the harness's `VirtualClock`, which a key press leaves where it was, so characters typed in one row join one buffer unless the clock is moved past 1 s.
  > the test harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance` (packages/blitz-test-harness/src/harness.rs:152-174), and settle reads animation time only through that clock, never advancing it
- (.andromeda/design-system.md:129) The body states the clock rule the typeahead row depends on.
  > The test harness drives animation from the `VirtualClock` its documents read, and settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.
- (.andromeda/design-system-detail/surface.md:8) Every stand document the re-read boots must take the one default user-agent sheet, the source of its unstyled control colours.
  > Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()`
- (.andromeda/design-system-detail/surface.md:9) A new keyboard row on the flight booker's select must read its chosen option through the select's default box, which shows that option alone.
  > In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token

## Bindings
- **palette ↔ typography**: each contrast row's pass is the accent, page, text or invalid pair against the required ratio its text size and weight set — 4.5, or 3.0 for large text (.andromeda/design-system-detail/color-palette.md:27) (.andromeda/design-system-detail/typography.md:18)
- **palette ↔ desktop-native-tokens**: a stand control left unstyled reads the default sheet's colours, the select's white background and the button and focus-outline defaults (.andromeda/design-system-detail/color-palette.md:28) (.andromeda/design-system-detail/surface.md:8)
- **typography loading ↔ every row**: the bundled font and blitz-dom's `system-fonts` feature must hold under every check the widened `a11y` leg runs, or a row passes vacuously (.andromeda/design-system-detail/typography.md:30) (.andromeda/design-system-detail/typography.md:36)
- **depth ↔ palette states**: the hover and active state rows reach their colours only through a pointer the hit-test rule delivers to the control (.andromeda/design-system-detail/depth-strategy.md:26) (.andromeda/design-system-detail/color-palette.md:27)
- **expression-motion ↔ desktop-native-tokens**: a select keyboard row reads its chosen option through the default select box, its typeahead buffered on the `VirtualClock` a press leaves still (.andromeda/design-system-detail/motion.md:26) (.andromeda/design-system-detail/surface.md:9)
