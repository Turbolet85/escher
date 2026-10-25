# design-system · Border Radius
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Border Radius detail.

## Border Radius

<!-- detail: values · radius -->

| Token | Value | Usage |
|-------|-------|-------|
| Browser chrome (untokenized) | 4px tabs (top corners), tooltip, urlbar input, icon buttons, menu; 3px close button and status bar (top-right) | (apps/browser/assets/browser.css:45-46; apps/browser/assets/browser.css:67; apps/browser/assets/browser.css:104; apps/browser/assets/browser.css:166; apps/browser/assets/browser.css:180; apps/browser/assets/browser.css:216; apps/browser/assets/browser.css:295) |
| Browser about pages (untokenized) | clear button 6px, history rows 8px, newtab search input 8px | (apps/browser/assets/about-history.css:27; apps/browser/assets/about-history.css:48; apps/browser/assets/about-newtab.css:29) |
| seven_guis (untokenized) | cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px, the flight booker's `flight-type` select 6px | (examples/seven_guis/src/tasks/counter.rs:41; examples/seven_guis/src/tasks/counter.rs:61; examples/seven_guis/src/tasks/temp_converter.rs:71; examples/seven_guis/src/app.rs:228; examples/seven_guis/src/app.rs:269; examples/seven_guis/src/tasks/flight_booker.rs:160-167) |
| transparent example card | 16px | (examples/transparent/src/app.rs:121) |
| blitz-dom default button | 1px | (packages/blitz-dom/assets/default.css:108) |
| Checkbox frame (engine) | 2 × control scale | (packages/blitz-paint/src/render/form_controls.rs:28-33) |
| Scrollbar thumb (engine) | half the thumb thickness (fully rounded) | (packages/blitz-paint/src/render.rs:823-833) |

<!-- /detail -->

<!-- detail: values · radius -->

- Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1395-1412)
- `inset()` clip-path ignores border-radius per a TODO (packages/blitz-paint/src/render/clip_path.rs:167-168)
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1124-1145)

<!-- /detail -->

<!-- detail: values · radius -->

- Uniform, percentage, per-corner and elliptical radii (examples/assets/border.html:16-20; examples/assets/border-styles.html:65-73; examples/assets/cursor.html:34)
- google search box 24px, buttons 4px, also 2px, 8px and 50% (examples/assets/google.html:3151; examples/assets/google.html:3491; examples/assets/google.html:100; examples/assets/google.html:923; examples/assets/google.html:459); graphite inputs 0 and carousel dots 50% (examples/assets/graphite.html:341; examples/assets/graphite.html:577); hr pills 999px (examples/assets/hr.html:128); input fixture 100px (examples/assets/input.html:9)
- `.button` `border-radius: 1.5rem 0` in the inline-flex-transform fixture and the transforms example (examples/assets/inline-flex-transform.html:22; examples/transforms.rs:291)
- servo.css `.button` 4px, `.box` 6px, loader 9999px (examples/assets/servo.css:31; examples/assets/servo.css:3043; examples/assets/servo.css:85); shadow fixture `.card` 12px (examples/assets/shadow.html:24); Preact TodoMVC filter buttons 3px (examples/preact/index.html:37); reference page `code` 4px and `.tag` 999px (examples/preact/core_dom_apis.html:27; examples/preact/core_dom_apis.html:39)

<!-- /detail -->

<!-- detail: absence · radius -->

- border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files
- border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
- border radius values · searched: `radius` over the 8 s07 slice files
- border radius values · searched: `border.radius|radius` over the 16 listed s08 files (no match)
- radius definitions · searched: `border-radius|radius` over the 32 s10 slice files
- border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

<!-- /detail -->
