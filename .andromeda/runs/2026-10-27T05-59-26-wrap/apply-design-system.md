# Apply — design-system

## Verdicts
- fanout-design-system--cascade.md 2 · refused · a hand removal · .andromeda/design-system.md:16–17
- fanout-design-system--cascade.md 1 · refused · a hand removal · .andromeda/design-system-detail/color-palette.md:27–27
- fanout-design-system--cascade.md 3 · refused · a hand removal · .andromeda/design-system-detail/color-palette.md:28–28
- fanout-design-system--cascade.md 4 · refused · a hand removal · .andromeda/design-system-detail/color-palette.md:73–73
- fanout-design-system--cascade.md 5 · refused · a hand removal · .andromeda/design-system-detail/color-palette.md:101–101
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/color-palette.md:26+#1
- fanout-design-system--cascade.md 7 · refused · a hand removal · .andromeda/design-system-detail/typography.md:31–31
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/typography.md:30+#1
- fanout-design-system--cascade.md 9 · refused · a hand removal · .andromeda/design-system-detail/spacing.md:19–19
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system-detail/spacing.md:18+#1
- fanout-design-system--cascade.md 11 · refused · a hand removal · .andromeda/design-system-detail/depth-strategy.md:10–10
- fanout-design-system--cascade.md 12 · applied · .andromeda/design-system-detail/depth-strategy.md:9+#1
- fanout-design-system--cascade.md 13 · refused · a hand removal · .andromeda/design-system-detail/border-radius.md:12–12
- fanout-design-system--cascade.md 14 · applied · .andromeda/design-system-detail/border-radius.md:11+#1

## Edits
- after: .andromeda/design-system-detail/color-palette.md:26
  anchor: «| Browser about:history link | `#1a73e8` | History link color (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |»
  add: «| seven_guis stand | — | The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |»
- after: .andromeda/design-system-detail/typography.md:30
  anchor: «- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)»
  add: «- The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value»
- after: .andromeda/design-system-detail/spacing.md:18
  anchor: «- rdme markdown body max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)»
  add: «- The seven_guis stand's card and Home padding is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as a design-system value»
- after: .andromeda/design-system-detail/depth-strategy.md:9
  anchor: «- Browser menu and suggestions share `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15)` (apps/browser/assets/browser.css:217; apps/browser/assets/browser.css:323)»
  add: «- The seven_guis stand's card and circle-dialog shadows are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values»
- after: .andromeda/design-system-detail/border-radius.md:11
  anchor: «| Browser about pages (untokenized) | clear button 6px, history rows 8px, newtab search input 8px | (apps/browser/assets/about-history.css:27; apps/browser/assets/about-history.css:48; apps/browser/assets/about-newtab.css:29) |»
  add: «| seven_guis stand | — | The stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |»

## Patterns
(none)
