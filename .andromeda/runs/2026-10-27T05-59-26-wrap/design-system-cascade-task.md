# Task — design-system · 2026-10-27-readme-and-records

## The session
A documents-and-records chunk: the README's Status now shows 0.1.0 and 0.2.0 done and verified and 0.3.0 planned in six stages, and the capability statement was restated to the 2026-10-25 cold-agent live run, the measured host-sink part read and the measured lint at boot (the flight booker's one `overlap`).
No source, test, CI script, manifest, lock file, master, category file or sidecar changed; no colour, font, spacing, shadow, radius or motion value changed anywhere in the code.
v030-16's records enumerate nine seven_guis stand rows in design-system that leave the product surface, values unchanged, by P2 amendment on F2 «Every stand row», the founder's delegate answer, PROVISIONAL: the colour, shadow, spacing and radius rows go to layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), the stand's DejaVu Sans registration to architecture §Standard Contracts → Headless stand, and after the move design-system's body and detail files hold none of the nine.
Ten further design-system stand lines are listed, not decided: whether they move goes to the operator as a dialog at P2.
Retired: the local `.git/info/exclude` incubator lines; `.gitignore` already ignored the incubator, before and after.

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .git/info/exclude

## Touches
- Core Colors: the seven_guis accent row — `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, its non-text uses and the SC 1.4.3 pass for white text on it — leaves design-system (move-set rows 1–3: the body's Core Colors line and color-palette's two accent rows); it lands in layout-templates' `values · desktop-native-screens`, F2 PROVISIONAL.
- Border Progression: the stand select's overrides (`select#flight-type`, `1px solid #c0c0c0`, the accent's `rgba(58, 92, 229` focus colour) leave design-system (move-set row 5, color-palette's border-progression values).
- Semantic Colors: the stand's success colours `#ebf8ee` · `#68d391` · `#276749` leave design-system (move-set row 4, the status-states group).
- Typography: the stand's DejaVu Sans registration (`font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`) leaves design-system (move-set row 6) for architecture's Headless stand contract.
- Depth Strategy: the stand's card shadow `0 2px 8px` leaves design-system (move-set row 7).
- Spacing: the stand's card padding (`24px 32px`, `48px 32px 64px`, seven_guis untokenized) leaves design-system (move-set row 8).
- Border Radius: the stand's card radius row leaves design-system (move-set row 9).
- Surface: desktop-native tokens: the select's overrides restated there (surface tokens group) are among the lines listed with the move set.
- The subjects comment at the foot of the body (`desktop-native-stand-load-page` among its subjects) is restated with the move.
- Undecided, for the operator's P2 dialog: the stand's page background `#f5f5f5` (shared with todomvc) in the Surface Scale group.
- Undecided: the stand's text colour `#1a1a1a` in the Text Hierarchy group.
- Undecided: the stand's invalid colours `#e53e3e` · `#fff5f5` · `#c53030` in the Semantic Colors group.
- Undecided: two stand typography lines in the first typography values group.
- Undecided: one further stand spacing line beside the card padding.
- Undecided: the `desktop-native-stand-load-page` component group in surface (its `values · desktop-native-stand-load-page` block).
- Undecided: motion's `LoadNet` answering after 200 ms line.
- Undecided: the web-spa platform line's WASM pages on `#f5f5f5` (shared with todomvc).
- Indirect, through the accent's move: a11y-plan's visual-design-verification citation of `design-system §Color Palette` is re-pointed to the accent's new home, so Color Palette stops being the cited source of the stand accent.
- Indirect, through F2: every design-system entry the move edits or removes carries the PROVISIONAL mark until the founder's own word, which supersedes F2 by rule.
- Indirect, through v030-16: its flip to verified reads that design-system's body and detail files hold none of the nine move-set rows.

## New facts
- F1 · color-palette · The seven_guis stand's accent, success and select-border colours are product-surface values held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not design-system tokens — F2, PROVISIONAL.
- F2 · typography · The stand's DejaVu Sans registration is held as architecture §Standard Contracts → Headless stand's contract beside `font_ctx()`, not as a design-system typography row — F2, PROVISIONAL.
- F3 · depth-strategy · The stand's card shadow is held in layout-templates' `values · desktop-native-screens` block, not design-system — F2, PROVISIONAL.
- F4 · spacing · The stand's card padding is held in layout-templates' `values · desktop-native-screens` block, not design-system — F2, PROVISIONAL.
- F5 · border-radius · The stand's card radius is held in layout-templates' `values · desktop-native-screens` block, not design-system — F2, PROVISIONAL.

## Untouched
- Engine colour model, engine-painted defaults and the user-agent stylesheet rows of color-palette: no engine value moved and none is a stand row.
- Color-palette's todomvc text row, the border-styles fixture row and the Preact page row: they share a hex value with the stand but are other examples.
- Typography's generic `build_single_font_ctx` rule for WASM, the `system-fonts` build note and the `fonts-unavailable` refusal of a shot: not stand rows, unchanged.
- Surface's WASM bundled-font line (`wasm_hello`): another example's font, not the stand's.
- Depth-strategy's browser-app menu shadow and the engine hit-test clip rule at a paint-clipping box: no engine or browser-app change.
- Motion's engine rules — the instant nested into-view scroll, settle's animating set, the `VirtualClock` and the load wait that moves no clock: no clock, settle or scroll behaviour changed.
- Iconography: the browser app's icons, cursors and favicons are untouched.
- The cli surface's `escher-session` output contract (JSON-only stdout, exit statuses `0` · `1` · `2` · `3`, the `mcp` role's ending): no verb, field, exit or carrier changed.
- The mobile-native surface (Android APK, `IS_MOBILE`, urlbar, back button, capture): untouched.
- The web-spa canvas component group (dioxus-native's wasm32 canvas sizing): unchanged.
- The NO RECORDED INTENT and NOT YET MEASURED blocks (Brand Identity, Anti-Patterns, Self-Validation Protocol, Design Decisions Log, the project-wide scales): the session records no intent and no scale.
