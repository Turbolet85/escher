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
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:302-305)
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#3.1 · values · spacing · ## Spacing · lines 27–27
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
### spacing#3.2 · values · spacing · ## Spacing · lines 28–28
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1147-1184)
### spacing#3.3 · values · spacing · ## Spacing · lines 29–29
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
### spacing#3.4 · values · spacing · ## Spacing · lines 30–30
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
### spacing#3.5 · values · spacing · ## Spacing · lines 31–31
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1057-1068)
### spacing#3.6 · values · spacing · ## Spacing · lines 32–32
- The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)
### spacing#4.1 · values · spacing · ## Spacing · lines 38–38
- Grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
### spacing#4.2 · values · spacing · ## Spacing · lines 39–39
- graphite :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80, lowered at width<=780px and for print or width<=500px; sibling main sections spaced by calc(120*var(--variable-px)) (examples/assets/graphite.html:907-914; examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951; examples/assets/graphite.html:1203-1205)
### spacing#4.3 · values · spacing · ## Spacing · lines 40–40
- hr gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)
### spacing#4.4 · values · spacing · ## Spacing · lines 41–41
- servo.css `--columnGap: 0.75rem` as column padding and negative margin; `.container.is-fluid` 32px side padding; `.button` padding `calc(0.5em - 1px)` / `calc(0.75em - 1px)` (examples/assets/servo.css:1577-1585; examples/assets/servo.css:342-345; examples/assets/servo.css:38-41)
### spacing#4.5 · values · spacing · ## Spacing · lines 42–42
- Preact TodoMVC `.app` max-width 520px with 40px auto margin, list items 14px 16px with 12px gap; reference page body max-width 60rem, padding 0 1.25rem (examples/preact/index.html:16; examples/preact/index.html:24; examples/preact/core_dom_apis.html:11-13)
