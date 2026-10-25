# design-system · Spacing
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Spacing detail.

## Spacing

<!-- detail: values · spacing -->

| Token | Value | Usage |
|-------|-------|-------|
| `--base-size-4/8/16/24/40` | 0.25–2.5rem | rdme spacing tokens (apps/readme/assets/github-markdown.css:2-6) |

<!-- /detail -->

<!-- detail: values · spacing -->

- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
- about:history padding 32px 48px; list items 12px 16px with gap 12px and margin-bottom 8px (apps/browser/assets/about-history.css:4; apps/browser/assets/about-history.css:49-54)
- rdme markdown body max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)
- seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)
- The seven_guis flight booker's `flight-type` select pads 8px vertical, 10px horizontal (examples/seven_guis/src/tasks/flight_booker.rs:160-167)
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)

<!-- /detail -->

<!-- detail: values · spacing -->

- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1147-1184)
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1057-1068)
- The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)

<!-- /detail -->

<!-- detail: values · spacing -->

- Grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
- graphite :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80, lowered at width<=780px and for print or width<=500px; sibling main sections spaced by calc(120*var(--variable-px)) (examples/assets/graphite.html:907-914; examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951; examples/assets/graphite.html:1203-1205)
- hr gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)
- servo.css `--columnGap: 0.75rem` as column padding and negative margin; `.container.is-fluid` 32px side padding; `.button` padding `calc(0.5em - 1px)` / `calc(0.75em - 1px)` (examples/assets/servo.css:1577-1585; examples/assets/servo.css:342-345; examples/assets/servo.css:38-41)
- Preact TodoMVC `.app` max-width 520px with 40px auto margin, list items 14px 16px with 12px gap; reference page body max-width 60rem, padding 0 1.25rem (examples/preact/index.html:16; examples/preact/index.html:24; examples/preact/core_dom_apis.html:11-13)

<!-- /detail -->
