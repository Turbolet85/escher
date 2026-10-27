# Task — design · 2026-10-27-upstream-sync-engine-debts

## The chunk
Takes upstream blitz's new work (DioxusLabs/blitz `upstream/main` at the measured sha `ebacd68bd`, 27 commits open to take or leave) onto the fork's build branch, by the take shape chosen at P4: cherry-picks onto the registry pins, that plus winit beta.4 and the winit clipboard, a whole merge with git Taffy, git Stylo and Dioxus 0.8 alpha, or nothing.
Resolves the predicted conflicts so both sides' intent holds: the hit stops, the scrolled-box bounds, the changed set, the select and range models and the document clock stay unchanged.
Carries upstream's pins as one coupled set or none, and runs upstream's new and edited test files as one-file-one-target blitz-tests targets under a take that carries them.
Re-reads the merge surfaces of the fork's engine changes per row, the clock invariant, the sink allowlist against upstream's new log sites, and the per-OS test tallies of the take's green CI run.
Re-proves every standing check: the fast and doc legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit and fork CI.

## Reached files
- Cargo.toml
- Cargo.lock · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-vibey-script/src/runtime.rs · conflict surface
- wpt/WPT_COMMIT · conflict surface
- packages/blitz-dom/src/node/element.rs
- packages/blitz-dom/src/stylo.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/events/driver.rs
- packages/blitz-dom/src/node/text.rs
- packages/blitz-shell/src/lib.rs
- packages/blitz-shell/src/window.rs
- packages/blitz-shell/src/convert_events.rs
- packages/blitz-shell/Cargo.toml
- packages/dioxus-native/Cargo.toml
- packages/blitz-traits/src/events.rs
- packages/blitz-traits/src/shell.rs
- packages/dioxus-native-dom/src/dioxus_document.rs
- packages/blitz-vibey-script/src/dom/element.rs
- packages/blitz-vibey-script/tests/dom.rs
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs
- tests/blitz-tests/tests/clipboard.rs
- tests/blitz-tests/tests/inline_box_scrollable_overflow.rs
- tests/blitz-tests/tests/display_contents.rs
- tests/blitz-tests/tests/pre_overflow_scroll.rs
- tests/blitz-tests/tests/all.rs · edits its mod lines
- .github/scripts/test_blitz_tests_targets.py · edits the pinned target list
- apps/browser
- .github/workflows/publish-browser.yml
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist (engine and escher targets admitted, every other target dropped)
- .github/scripts/ci-leg.sh · re-reads the fast, doc, a11y and audit legs
- tests/blitz-tests/tests/stand_contrast.rs · re-reads stand_contrast
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads stand_keyboard
- .andromeda/test-plan.md · re-reads §Per-OS test tallies

## Needs
- The engine's hit test at a clipping box: a hit stops at the padding box of a box paint clips by overflow or by a computed contain that includes paint, the root element excepted, the node itself still answered inside its border box — kept unchanged through the node/node.rs conflict, beside upstream's hit test of an element with no computed styles yet (ebacd68bd).
- The contain: paint hit stop's getter, re-written under Stylo 0.23 if the take carries git Stylo (06ecec886 and the two commits that need it).
- The programmatic into-view scroll: every scrolling box holding its target written at once, innermost first, a requested Smooth applying to the viewport alone, the driver's scroll instant everywhere — kept through the scrolling.rs conflict and upstream's reversed scroll origins through Taffy's ScrollableOverflow (0980cce5f).
- The engine's time reads (overlay scrollbar fade, fling, smooth scroll, double-click window) through the document's clock; no Instant::now or SystemTime::now in blitz-dom outside clock.rs, re-read on the take's tree.
- The driver's advance and load wait as recorded in motion: advance moves the harness's clock by exactly its ms, click, type and press leave it, and the load wait moves no clock — re-proven by the stand checks after the take.
- Scrolled-box bounds: a scrolled box reads its own client rect where it stands (writing_mode.rs's second geometry body), kept through that file's conflict and 0980cce5f's rewrite.
- The recorded reading that a scrolling box under a transform is read where it stands untransformed: whether 0980cce5f moves it is recorded for the next entry, never fixed here.
- Text layout and measured bounds under a moved Taffy, Stylo or winit: a stand check asserting exact bounds, a snapshot's byte size or a screenshot's pixels may read a new value, re-measured and recorded, never loosened.
- The fonts-unavailable refusal of a screenshot or a lint when no sans-serif family resolves, and the bundled stand font, unchanged by the take.
- Text inputs: IME enabled when a focused text input is constructed (7fc9ee3af, dioxus_document.rs and construct.rs) and upstream's node/text.rs edit, beside the password mask's paint and click caret.
- Generated content: ::before and ::after pseudos voting in flow classification (2963b65a2), a layout change the incremental and non-incremental oracle reads in both modes.
- The details element's :open pseudo-class (53e9d81c6, mutator.rs, node/element.rs, stylo.rs), beside the fork's summary toggling by Enter and Space.
- The stand's control colours and text/background pairs, re-asserted row for row by stand_contrast in the a11y leg (white on the accent #3a5ce5 at 5.47), the computed styles it reads coming through a possibly moved Stylo.
- The engine-painted defaults (the user-agent stylesheet's colours and focus indication) read through Stylo, re-read where the take moves Stylo or the details styling.
- The cursor: the CSS cursor keyword, else Text for text inputs, Pointer in links, Text over selectable text, Default otherwise, a pointer over no box included — reached where the take touches document.rs, events/pointer.rs and the shell's winit events.
- The desktop-native window: winit windows in blitz-shell's window.rs, shell redraw requests routed through the event loop (6587e7e79), winit's exact beta moved to =0.31.0-beta.4 only as part of upstream's coupled set, and the shell's changed-set drain and AccessKit rebuild in View::poll kept.
- The desktop-native stack and its clipboard: winit's clipboard API in place of arboard (527394724), the clipboard feature leaving blitz-shell's and dioxus-native's default features.
- The desktop-native and mobile-native platform lines that cite publish-browser.yml line ranges, edited upstream by a7ea9631b and b5b163450 — citations re-read where the take carries them.
- The browser app's toolbar icons, URL bar and favicon citations in apps/browser, moved by d63ed2c8c where the take carries it.
- The Dioxus bridge's surface in dioxus_document.rs (element ids, the snapshot, the accessibility_tree override), and under a Dioxus 0.8 take the port of the stable-id derivation onto its vdom, decided by stand_element_ids, stand_id_persistence, stand_id_edits and the snapshot checks.
- The script fetcher narrowed so only a file: document may load a file: URL (cee6b8731), and CSS properties as accessors on CSSStyleDeclaration (efdc5472a), read against the untrusted-input rule.
- Upstream's two new tracing::error sites (blitz-shell's clipboard, blitz-vibey-script's uncaught JS error) read against the sink allowlist's engine-target rule, the capture checks staying at 0 needles on stderr.
- The a11y surface the take reaches (IME on a focused input, :open, hit testing): focus order, Enter and Space activation and accessibility-tree identity re-proven by stand_keyboard, accessibility checks, inert_subtree and the a11y leg.
- Per-OS tallies: the take's green CI run's test-bearing job logs re-read by the test plan's method, each moved count explained by upstream's new targets.

## Standing conventions
- S1 · every blitz-tests target that reads text builds blitz-dom with the system-fonts feature, or text measures 0x0 and its assertions pass vacuously · Typography (body, Loading)
- S2 · a test that reads animation or scroll time drives it from the harness's VirtualClock, and a settle answers the animating set the same way, never waiting on or advancing it · Motion (body, Animation runtime)

## Not needed
- Brand Identity, Anti-Patterns, Self-Validation Protocol and the Design Decisions Log: each records no intent.
- The spacing values in the apps and examples and the engine's spacing defaults: no commit of the delta changes a margin, padding or gap value.
- The border-radius values and the engine's radius handling: the scope names no paint or radius change.
- The observed shadow and elevation values in the apps and examples: no example's styles are in the delta.
- The motion expression level and reduced-motion handling: one records no intent, the other is observed absent, and the take adds neither.
- The cli surface's WPT runner terminal colours and its tokens: the delta touches no runner source or its colour library.
- The driver command line's and its mcp role's stdout form (JSON only, no colour, the exit-status classes): the take reaches no escher-driver or session-host code.
- The desktop-native custom widget pattern: its example is not in the delta.
- The stand load page's component pattern: the take changes no stand page or its stylesheet.
