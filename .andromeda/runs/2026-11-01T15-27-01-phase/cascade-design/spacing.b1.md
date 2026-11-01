### spacing#1.1 · values · spacing · ## Spacing · lines 8–10
| Token | Value | Usage |
|-------|-------|-------|
| `--base-size-4/8/16/24/40` | 0.25–2.5rem | rdme spacing tokens (apps/readme/assets/github-markdown.css:2-6) |
### spacing#2.1 · values · spacing · ## Spacing · lines 16–16
- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
### spacing#2.2 · values · spacing · ## Spacing · lines 17–17
- about:history padding 32px 48px; list items 12px 16px with gap 12px and margin-bottom 8px (apps/browser/assets/about-history.css:4; apps/browser/assets/about-history.css:49-54)
### spacing#2.3 · values · spacing · ## Spacing · lines 18–18
- rdme markdown body max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)
### spacing#2.4 · values · spacing · ## Spacing · lines 19–19
- The seven_guis stand's card and Home padding is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as a design-system value
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:313-316)
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#2.7 · values · spacing · ## Spacing · lines 22–22
- blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)
### spacing#3.1 · values · spacing · ## Spacing · lines 28–28
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
### spacing#3.2 · values · spacing · ## Spacing · lines 29–29
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1163-1200)
### spacing#3.3 · values · spacing · ## Spacing · lines 30–30
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
### spacing#3.4 · values · spacing · ## Spacing · lines 31–31
- An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### spacing#3.5 · values · spacing · ## Spacing · lines 32–32
- `place` sets an open select list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:399-457)
### spacing#3.6 · values · spacing · ## Spacing · lines 33–33
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
### spacing#3.7 · values · spacing · ## Spacing · lines 34–34
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1060-1071)
### spacing#3.8 · values · spacing · ## Spacing · lines 35–35
- The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)
### spacing#4.1 · values · spacing · ## Spacing · lines 41–41
- Grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
### spacing#4.2 · values · spacing · ## Spacing · lines 42–42
- graphite :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80, lowered at width<=780px and for print or width<=500px; sibling main sections spaced by calc(120*var(--variable-px)) (examples/assets/graphite.html:907-914; examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951; examples/assets/graphite.html:1203-1205)
### spacing#4.3 · values · spacing · ## Spacing · lines 43–43
- hr gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)
### spacing#4.4 · values · spacing · ## Spacing · lines 44–44
- servo.css `--columnGap: 0.75rem` as column padding and negative margin; `.container.is-fluid` 32px side padding; `.button` padding `calc(0.5em - 1px)` / `calc(0.75em - 1px)` (examples/assets/servo.css:1577-1585; examples/assets/servo.css:342-345; examples/assets/servo.css:38-41)
### spacing#4.5 · values · spacing · ## Spacing · lines 45–45
- Preact TodoMVC `.app` max-width 520px with 40px auto margin, list items 14px 16px with 12px gap; reference page body max-width 60rem, padding 0 1.25rem (examples/preact/index.html:16; examples/preact/index.html:24; examples/preact/core_dom_apis.html:11-13)
