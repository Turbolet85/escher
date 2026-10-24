### color-palette#1.4 · values · palette · ## Color Palette · lines 11–11 · stale
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:148-152)
### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12 · stale
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1184-1191)
### color-palette#3.7 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 46–46 · add-to
| custom-widget, static-HTML and wgpu_texture examples | `#f4e8d2` | Main background (examples/custom_widget.rs:187; examples/html.rs:16; examples/wgpu_texture/src/styles.css:15) |
### color-palette#6.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 79–82 · stale
**Domain status colors** — WPT runner terminal result colors:
| Result | Terminal color |
|--------|----------------|
| PASS | green (wpt/runner/src/main.rs:388-398) |
### color-palette#6.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 83–83 · stale
| FAIL with some passing subtests | yellow (wpt/runner/src/main.rs:388-398) |
### color-palette#6.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 84–84 · stale
| FAIL | red (wpt/runner/src/main.rs:388-398) |
### color-palette#6.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 85–85 · stale
| TIMEOUT | bright red (wpt/runner/src/main.rs:388-398) |
### color-palette#6.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 86–86 · stale
| SKIP | bright black (wpt/runner/src/main.rs:388-398) |
### color-palette#6.6 · values · status-states · ## Color Palette › ### Semantic Colors · lines 87–87 · stale
| CRASH | bright magenta (wpt/runner/src/main.rs:388-398) |
### typography#1.14 · values · typography · ## Typography · lines 23–23 · stale
| List bullets | `"Bullet, monospace, sans-serif"` | not stated | not stated | not stated | (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/construct.rs:1106-1112) |
### typography#3.6 · values · typography · ## Typography · lines 47–47 · stale
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1199-1201)
### spacing#3.1 · values · spacing · ## Spacing · lines 26–26 · stale
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1075-1101)
### spacing#3.2 · values · spacing · ## Spacing · lines 27–27 · stale
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1145-1182)
### spacing#3.4 · values · spacing · ## Spacing · lines 29–29 · stale
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:855-873)
### depth-strategy#2.1 · values · depth · ## Depth Strategy · lines 19–19 · stale
- Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:121-125; packages/blitz-dom/src/lib.rs:89)
### depth-strategy#2.2 · values · depth · ## Depth Strategy · lines 20–20 · stale
- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1323-1366); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1368-1397)
### depth-strategy#3.1 · values · depth · ## Depth Strategy · lines 26–26 · add-to
- Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1106-1169); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1555-1617), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box (packages/blitz-dom/src/node/node.rs:1521-1554; as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md); since 2026-10-14-stand-requirement-sweep its answer at an element's own box is measured, in both layout modes and both builds — an `img` holding raster data answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter the sub-document, pinned as it reads) and a filled text `input` answers the `input` — and a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:431-584; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)
### border-radius#2.3 · values · radius · ## Border Radius · lines 24–24 · stale
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1122-1143)
### motion#1.6 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 13–13 · stale
- Overlay scrollbars show at full opacity on scroll, stay opaque for `FADE_DELAY` = 500 ms after last activity, then fade linearly over `FADE_DURATION` = 200 ms, documented as Chromium's overlay timings (packages/blitz-dom/src/node/scrollbar.rs:12-26; packages/blitz-dom/src/document.rs:1975-1992; packages/blitz-paint/src/render.rs:725-730; packages/blitz-paint/src/render.rs:752-755); finished fades are dropped each resolve (packages/blitz-dom/src/resolve.rs:67-74); hovering where a hidden thumb would be does not summon it, and the thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166; tests/blitz-tests/tests/scrollbar_drag.rs:168-222)
### motion#2.1 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 19–19 · stale
- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:105-123)
### surface#1.1 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 8–8 · stale
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:76-95) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
### surface#13.1 · values · cli-tokens · ## Surface: cli › ### Tokens (platform-specific) · lines 105–105 · stale
- Result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:388-398)
### surface#13.2 · values · cli-tokens · ## Surface: cli › ### Tokens (platform-specific) · lines 106–106 · stale
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:401; wpt/runner/src/main.rs:411-438; wpt/runner/src/main.rs:793; wpt/runner/src/main.rs:797; wpt/runner/src/main.rs:811; wpt/runner/src/main.rs:816)
### surface#13.3 · values · cli-tokens · ## Surface: cli › ### Tokens (platform-specific) · lines 107–107 · stale
- Test names are wrapped in OSC 8 hyperlinks to `https://wpt.live/{name}` only when `supports_hyperlinks::on(Stdout)` (stdout is a terminal, honouring `FORCE_HYPERLINK`) (wpt/runner/src/main.rs:56-65; wpt/runner/src/main.rs:366-376)
