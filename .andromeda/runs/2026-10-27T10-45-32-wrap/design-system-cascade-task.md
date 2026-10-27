# Task — design-system · 2026-10-27-upstream-sync-engine-debts

## The session
Took eleven upstream DioxusLabs/blitz commits by no-commit cherry-pick on the founder's word (upstream main read at `ebacd68bd`, merge base `23b9b26a9` unmoved, sixteen left out); no dependency, config, crate or master spec changed, nothing retired.
Engine behaviour for every document: `:open` now matches a `details` or `dialog` carrying `open` (`ElementData::flush_open_state`); every box-generating `::before`/`::after` votes in flow classification; a text input already focused when its editor is built enables IME; the windowed shell routes redraw requests through its event loop.
blitz-vibey-script: a `file:` URL loads only for a `file:` document (`document_may_load`, a narrowing); CSS properties become `CSSStyleDeclaration` accessors; the runtime and the element bindings split into `runtime/` and `dom/element/` modules (a move, nothing removed); the uncaught-JS-error site moves to `runtime/events.rs:21`.
Tests added: blitz-tests target `pseudo_element_flow_classification` (147 → 148 files), `tests/dom.rs` 27 → 30, dioxus-native-dom lib 81 → 82; CI, a11y, audit and `run stand` green with no stand figure moved.
On the founder's word (inputs#I7): F2's earlier move of the nine stand rows to layout-templates is ratified, and the ten further design-system stand lines move to layout-templates' desktop-native screens, values unchanged — eight stand-only lines (five folding into what layout-templates states, three new there), the two rows shared with todomvc keeping todomvc and dropping seven_guis, and the subjects line dropping `desktop-native-stand-load-page`.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- wpt/WPT_COMMIT · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/{css,dom_exception,fetch,fonts,interfaces,node_constants,style,stylesheets,window}.js · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface

## Touches
- design-system.md:218 (web-spa Platform line, seven_guis and todomvc on `#f5f5f5`): a row shared with todomvc — keeps todomvc, drops seven_guis; the fact moves to layout-templates.
- color-palette.md:44 (Surface Scale group, lines 38–46): by elimination the other row shared with todomvc — keeps todomvc, drops seven_guis.
- color-palette.md:59 (Text Hierarchy group, lines 54–61): seven_guis text `#1a1a1a` leaves design-system, one of the three new layout-templates lines (layout-templates :15 and :16 already state `#1a1a1a`, a note for Validate).
- color-palette.md:71 (Semantic Colors, status-states group lines 69–72): the stand's invalid `#e53e3e` / `#fff5f5` / `#c53030` leaves design-system, new in layout-templates.
- typography.md:18 (group lines 8–24): the stand's sans-serif, home title 700, base 14px, title 36px leaves design-system, new in layout-templates; this line carries no seven_guis-style label, so a token search misses it.
- typography.md:19: the stand's select fact leaves, folding into layout-templates' mode-region bullet :15.
- spacing.md:20 (group lines 16–21): the stand's select fact leaves, folding into layout-templates :15; worded without a stand label.
- surface.md:10 (desktop-native-tokens group lines 8–13): the stand's select fact leaves, folding into layout-templates :15.
- surface.md:38–40 (the desktop-native-stand-load-page group, line 40): the load-page facts leave, folding into layout-templates' served-screens bullet :14; the group is emptied.
- motion.md:34 (group lines 33–36): the load-page fact leaves, folding into layout-templates :14; motion.md:33 beside it stays.
- design-system.md:247 subjects comment: drops `desktop-native-stand-load-page`.
- F2 ratified by the founder: the earlier move of the nine stand rows stands; design-system's PROVISIONAL wording lives only in its sidecar (`design-system-amendments.md:209`, `:219`, `:234`), kept as append-only history, with a new sidecar entry carrying the ratification; no gap marker stands in design-system's body or detail (the two drop in layout-templates :20 and architecture's standard-contracts :128).
- The five pointer rows to layout-templates (border-radius.md:12, color-palette.md:27, depth-strategy.md:10, spacing.md:19, typography.md:31): re-read, they now point at ratified rows; wording unmoved.
- Lines outside the founder's ten that still name the stand: surface.md:130–131 (`escher-session` serves six tasks), motion.md:8 (no transitions observed in seven_guis styles), motion.md:33 and design-system.md:129 (the driver's load wait) — they stay; the relay's claim that design-system then holds no stand fact does not hold, a note for Validate.
- Iconography favicon citation `apps/browser/src/tab.rs:280-292` (design-system.md:145 and its iconography detail): tab.rs gained lines at 130–136 and 177–182, so the cited block moves down; one of the browser-tab sweep's citation rows.
- desktop-native Toolkit line citing `packages/blitz-shell/src/window.rs:201-207` (design-system.md:160): window.rs gained lines 193–194, so the cited block moves by two lines.
- desktop-native Platform citation `publish-browser.yml:47-82` and mobile-native `:83-89`: the added lines (126, 153–166) lie below them, so they read unmoved; the upstream-only and repository-guard claims are unchanged.
- mobile-native Toolkit citations `apps/browser/src/main.rs:48` and `:172`: main.rs gained lines 192–193 only, below them; unmoved.
- Any detail line citing the other moved files (blitz-dom `layout/construct.rs`, `mutator.rs`, `node/element.rs`, `stylo.rs`, blitz-shell `lib.rs`, `dioxus_document.rs`, blitz-vibey-script `document.rs`, `fetch.rs`, `lib.rs`, `runtime.rs`, `dom/element.rs`): re-point by the take's line map; code cited in `runtime.rs` or `dom/element.rs` may have left for the new `runtime/` or `dom/element/` modules.
- typography.md:73, a match note naming blitz-vibey-script `tests/dom.rs`: re-read, no change from the file's 27 → 30 tests.
- motion.md:58 (todomvc's `::after` animation): re-read; the pseudo vote changes flow classification of `::before`/`::after` boxes, not the animation, so no change.
- Indirect, through `:open` now matching (stylo.rs, node/element.rs, mutator.rs): any engine-default or UA-styling line about `details` or `dialog` styled by open state.
- Indirect, through the shell's redraw route and the IME-on-construct change (blitz-shell lib.rs and window.rs, construct.rs): any desktop-native-window or desktop-native-stack line describing the winit shell's redraw or IME behaviour.
- Motion body :121 (a programmatic into-view scroll): its blitz-vibey-script caller moved from `dom/element.rs` to `dom/element/scrolling.rs`, calling the same engine `scroll_into_view`, which is unchanged — the rule stands.
- Stand values carried by the move: unchanged — a11y leg 14 · 85 · 0 · 0 and `run stand` 205 · 0 · 7 over 50 files, no stand figure or contrast row moved.

## New facts
- (none)

## Untouched
- Depth Strategy's hit-testing rule (body :85 and its detail): `hit_inner` and the clipping read were re-read unchanged, and `node/node.rs` is not reached.
- Motion's settle and `VirtualClock` lines (body :125, :127): harness and clock unchanged, 0 wall-clock reads outside `clock.rs`.
- Typography Loading (body :44, :46, :48: `fonts-unavailable`, the `system-fonts` feature, the WASM font context): no dependency, feature or driver change; the new `bootstrap/fonts.js` is a script binding moved by the split, not font loading.
- The cli surface's `escher-session` output rule (body :206) and cli-output groups: escher-driver and escher-telemetry are unchanged.
- Iconography's toolbar, menu and tab-strip icons (`apps/browser/src/icons.rs`, `tab_strip.rs`, `browser.css`): none of those files is reached.
- Iconography's cursor rule (body :149): the cursor reader is not among the reached blitz-dom files.
- desktop-native-stack and mobile-native-stack version lines: no dependency or pin moved (Stylo 0.22.0, taffy 0.15, Dioxus 0.7.3, winit `=0.31.0-beta.3`).
- web-spa canvas (body :219, `packages/dioxus-native/src/config.rs`): dioxus-native is not reached; only the platform line :218 is edited.
- The NO RECORDED INTENT and NOT YET MEASURED markers (Brand Identity, Anti-Patterns, Self-Validation Protocol, Design Decisions Log, the project-wide token-scale gaps): the session records no intent and measures no scale.
- Test and count readings this sync moved (blitz-tests files and targets, workspace tallies, `run all`, the `TMPDIR` sites, the script fetcher, the upstream take readings, the lock record, the JS-error site, the silence census): no design-system line carries them.
