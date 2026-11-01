# Task — design · 2026-11-01-upstream-sync-ui-graph

## The chunk
Takes the upstream delta from DioxusLabs/blitz `upstream/main` at the measured sha `e42ee1f25` into this fork: 33 commits to take or leave, the shape (cherry-pick on the registry pins, that plus winit beta.4, the whole merge, or nothing) chosen by the founder at P4 after P3 measures each.
Resolves the conflicts a take brings, fifteen files predicted for the whole merge, keeping both sides' intent; our engine contracts stay unchanged, and any edit to upstream-owned code is minimal, named and recorded as non-additive.
Carries the entry's freight: fixes or removes the skipped free-disk step of `matrix_test`, adds a font-embolden run to a local gate, removes or restores the orphan comment in blitz-dom `lib.rs`, re-reads the merge surfaces of the engine-debts changes and the select list row by row, and runs a controlled reproduction of the key-press panic at `node.rs:1775`, whose fix is the founder's fork.
Re-runs every standing proof after the take: the fast, doc, a11y and audit legs, the stand checks in both layout modes, the incremental/non-incremental oracle, and fork CI green; re-reads the per-OS tallies and explains each moved count; re-checks the sink allowlist against the taken delta.
Builds nothing of the UI graph, adds no crate, port, env var or listener of ours, and adopts no new upstream API in escher's own code.

## Reached files
- Cargo.toml · merge surface
- Cargo.lock · conflict surface
- examples/seven_guis/src/tasks/cells.rs · conflict surface
- packages/blitz-dom/assets/default.css · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-dom/src/stylo.rs · conflict surface
- packages/blitz-paint/src/text.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · conflict surface
- .github/workflows/ci.yml · edits the free-disk step of the `matrix_test` job (its `if:`, or the step removed)
- packages/blitz-dom/src/lib.rs · edits the orphan comment at lines 96-97
- .github/scripts/ci-leg.sh · edits the local gate the font-embolden run joins
- .github/scripts/test_blitz_tests_targets.py · edits the pinned target list, for upstream test files a take carries
- tests/blitz-tests/Cargo.toml
- tests/blitz-tests/tests/select_popup.rs · edits the one row that skips the key press, under the founder's panic fix
- tests/blitz-tests/tests/select_popup_painted.rs · re-reads the font-embolden witness
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads the keyboard check
- tests/blitz-tests/tests/stand_contrast.rs · re-reads the contrast rows
- tests/blitz-tests/tests/inert_subtree.rs · re-reads the inert subtree check
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads the element id check
- tests/blitz-tests/tests/stand_id_persistence.rs · re-reads the id persistence check
- tests/blitz-tests/tests/stand_id_edits.rs · re-reads the id edits check
- tests/blitz-tests/tests/stand_snapshot.rs · re-reads the snapshot check
- tests/blitz-tests/tests/stand_snapshot_parts.rs · re-reads the snapshot parts check
- tests/blitz-tests/tests/stand_snapshot_text.rs · re-reads the snapshot text check
- packages/blitz-test-harness/src/contrast.rs · re-reads `Harness::text_contrast`'s `None` conditions
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist
- .andromeda/test-plan.md · re-reads §Per-OS test tallies
- .claude/rules/security.md · re-reads §Dependencies, the audit's ignored IDs and the new-crate review
- .andromeda/residuals.md · edits the left-out-delta and merge-surfaces lines (35 and 36)
- .andromeda/architecture.md · edits the `Upstream sync:` line, at the wrap where the take moves the pin

## Needs
- Hit testing through `node/node.rs`: a resolution keeps our hit stops at a clipping node's padding box, under `overflow` or a paint-including `contain`, with the root excepted and an open select list's rows answered before the walk; `0980cce5f`'s `hit_inner` hunk conflicts with the non-invertible answer, and `ebacd68bd` changes hit testing where a node has no computed styles.
- The scrolled-box bounds in `layout/writing_mode.rs` and `absolute_position` in `node/node.rs` stay as they are under a take, `0980cce5f`'s reversed scroll origins included.
- Into-view scrolling through `scrolling.rs`: `scroll_into_view` and `visible_region` keep our behaviour in a resolution, with nested boxes written at once, a requested smooth behaviour reaching the viewport alone, and a select row's scroll moving only its own list; `e42ee1f25` changes off-screen culling bounds next to this.
- The clock invariant over the `scrolling.rs` clock surface: settle, the driver's `advance` and the driver's load wait keep their relation to the harness's `VirtualClock` after a take.
- The select model and open list: `SelectListView`'s engine-constant colours, the highlight pair it shares with a `multiple` select's chosen row, the select and range arms in `mutator.rs`, the `size` hint in `stylo.rs`, and the list's sixteen-file merge surface re-read row by row.
- The engine's default stylesheet `default.css` is a conflict surface, and `dee854c76` adds `<picture>`'s UA rule to it; `7a3dd8918` moves Stylo preferences ahead of the UA sheet; recorded engine default values for colour, type, spacing, depth and radius are read against the resolved sheet.
- Text and decoration painting: `stroke_text_as` in blitz-paint `text.rs` stays through a resolution, while `e5929f9e2` (`-webkit-text-fill-color`), `ac6f9f582` (`background-clip: text`) and `f76777766` (inline box decorations) can change what text paints.
- Compositing: `6fca6308d` adds `mix-blend-mode` and `isolation`, and a stand contrast row may need re-reading where a take carries it.
- Text contrast, through `Harness::text_contrast`'s `None` conditions and `stand_contrast`: added paint properties and `f6edb9701`'s change to what a failed stylesheet does can reach both, in every document.
- Layout construction and inline text: `layout/construct.rs`, `layout/inline/build.rs` (add/add) and `pseudo_element_flow_classification.rs` (add/add) resolve keeping both sides' intent, and recorded engine text mapping values are read against them.
- Measured numbers under a take: `ae774d7f4` (unrounded layout for computed sizes) or moved Taffy, Stylo or winit pins can change exact bounds, snapshot byte sizes and screenshot pixels; a moved number is re-measured and recorded, never loosened to pass.
- Restyle and damage: `fe33b778b` narrows restyle invalidation on DOM mutation and `8b1a0a3ce` adds layout damage for SVG attributes and `<col span>`, against the changed set's contract in `mutator.rs`.
- The a11y target (WCAG SC 2.1.1, 1.4.3, 2.4.3): focus order, Enter and Space activation, contrast and accessibility-tree identity re-proven by `stand_keyboard`, `stand_contrast`, the `accessibility_*` checks, `inert_subtree` and the a11y leg.
- The key-press panic at `node.rs:1775`: a focused control's anonymous block dropped when an ancestor gains `hidden`, then a key press on any control, reproduced at the base, with upstream's `node.rs` read; the fix is the founder's fork.
- dioxus-native-dom's escher surfaces in `dioxus_document.rs`: element ids, the snapshot with its text, parts and diff, unkeyed actionables and the `accessibility_tree` override, against `6ef4b532d`'s mounted-events change.
- The custom widget surface: `a26c6ddeb` replays `CustomWidget` attributes, reaching the desktop-native custom widget pattern where a take carries it.
- Dependency values the plan's surfaces carry: the Dioxus, winit, Stylo and Taffy versions recorded for the desktop-native and mobile-native stacks, read against the chosen take, along with winit beta.4's clipboard API in place of `arboard` where it is taken.
- The font-embolden gate: `blitz-paint/font-embolden` built locally through the blitz-tests witness `select_popup_painted`, which today only CI's macOS leg exercises.
- `escher-session`'s and `mcp`'s stdout, through the sink re-check: the taken delta is read for every new `tracing` or `log` call site, target, print and env read, and the capture checks stay at 0 needles.
- CI's `matrix_test` rows, the android row on `ubuntu-latest` among them, as the free-disk step's `if:` changes; the iOS and Android build legs stay on the mobile-native surface's record.
- The example fixture `examples/seven_guis`, its cells task a conflict surface, resolved so the fixture's recorded values stay accurate.

## Standing conventions
- S1 · A blitz-tests target, upstream's or ours, builds `blitz-dom` with `system-fonts`, so font-dependent assertions do not pass on text measuring 0x0 · Typography, Loading, in the body
- S2 · A test that takes a driver screenshot needs a font context resolving `sans-serif`, since the shot is refused otherwise · Typography, Loading, in the body
- S3 · A test reading animation or settle takes time from the harness's `VirtualClock` · Motion, Animation runtime, in the body
- S4 · A new print or log line keeps the agent-facing command output JSON only, with no colour or ESC byte and no terminal test · Surface: cli, Component Patterns, in the body

## Not needed
- The browser app's toolbar icon assets, icon sizes, favicons and bundle icons: the scope restyles no app chrome.
- The colours, spacing, radii and depth values of `apps/browser/assets/browser.css` and `apps/readme/assets/github-markdown.css`: no file of either app is named by the scope.
- Fixture values of examples other than `examples/seven_guis`, such as todomvc, counter and box_shadow: the scope names no change to them.
- Project-wide token sets and scales recorded as not yet measured (colour, type, spacing, elevation, radius): the chunk defines none.
- The recorded absence of reduced-motion handling: the chunk adds none, and the scope asks for no motion preference.
- The mobile-native urlbar, back button and capture patterns: these are browser app behaviour the scope does not change.
- The desktop-native titlebar and the devtools overlay: the scope names no change to either.
- The web-spa canvas and the todomvc page's WASM hosting: the scope names no wasm surface.
- The WPT runner's command line output and its `owo-colors` values: upstream's WPT conformance numbers and workflows are out of scope.
- The browser's publish targets and packaging (NSIS, dmg, AppImage, APK) and the flake's default package: the scope touches neither `publish-browser.yml` nor `flake.nix`.
