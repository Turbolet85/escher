# Task — design · 2026-10-31-engine-debts

## The chunk
Takes six recorded engine debts to the founder as forks, each to end fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual: the transformed scrolling box, the `inert` focus fixup, native checkedness, the select model's limits, the select-option overlap, and the served pages' keyboard and contrast rows.
Every fork is unanswered at planning: all of them, with their dependences, go in one file at P4 with no provisional answer, so any fix below is conditional on the founder's word.
Adds checks of what already runs, none an engine change: `:open` on `details` and `dialog`, the windowed shell's redraw route through its event loop, and `pseudo_element_flow_classification` in both layout modes.
Re-asserts SC 2.1.1, 2.4.3 and 1.4.3 through `stand_keyboard`, `stand_contrast`, `inert_subtree` and `select_interaction`, and adds the served pages' Tab-order, keyboard-activation and contrast rows as the ground of the route's A11y re-read.
Carries one stale name for the wrap to amend: `DocumentMutator::remove_attribute` becomes `clear_attribute`, in its contract row and sidecar entries.

## Reached files
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/events/
- packages/blitz-dom/src/mutator.rs · re-reads `clear_attribute`, the remover's name
- tests/blitz-tests/tests/transformed_client_rect.rs · edits `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`
- tests/blitz-tests/tests/inert_subtree.rs · edits `removing_inert_restores_focus_hit_and_tree`
- .andromeda/residuals.md · edits lines 31, 32, 33, 34, 39 and 40
- .andromeda/architecture-detail/standard-contracts.md · edits the `ElementData::classes` row

## Needs
- `visible_region` for a target inside a scrolling box under a CSS transform: a translated box, and whether a scale is in the fix's reach, which P3 measures.
- A pan/zoom container fixture like the node graph's, a CSS `transform` on a container holding absolutely positioned nodes, as the proof of any transformed-box fix.
- Paint and hit-test behaviour of a scrolled or transformed box, because a fix must agree with escher's scrolled-box bounds and hit and upstream `0980cce5f` touches the same code.
- The driver refusal `off-screen`: its schema text states the transformed-box limit, so a fix moves that text and its capability-statement pin.
- The driver refusal `covered`: a `click` on a select option is refused it today, and its schema text moves with any fix to that limit.
- Focus state when a node or an ancestor becomes `inert`: where focus goes when the fixup runs, and what a focused node shows until then, under SC 2.4.3.
- Checked state of `<input type="checkbox">` and `type="radio"`: the `toggled` value in the accessibility tree, the snapshot's `checked` and `Reading`'s `pressed`, and whether a fix moves any reading.
- A checked state crossing to the platform adapter as a closed value, the state already ratified on that crossing, which a fix must not widen.
- The select control's rendering: a `multiple` select laid out as an empty bordered box, and how its options and an `optgroup` would show if modelled.
- A painted select popup if the founder takes it: engine-painted colours, surface and elevation, spacing, radius and option text, plus paint order and hit-testing above other content.
- An option's click and key handling in a select, and the engine `change` event a checkbox, radio or text input does not fire.
- The bounds a `ListBoxOption` reads: its select's client rect today, what a painted popup would change, and the dependence that asks that fork with the popup's.
- The `lint` `overlaps` check in the driver's `lint.rs`, and `stand_lint`'s pinned `[0, 1, 0, 0]` from `flight-type-one-way` against `flight-type-return` on the flight booker at boot.
- The load page's `load-fetch` Button and the Cells page: Tab order and keyboard activation under SC 2.1.1, with the focusable controls these rows walk.
- Foreground and background colours of the served pages' text and controls, and of the stand's controls, read for contrast under SC 1.4.3; a red row is a further fork, not a silent re-pin.
- Engine-painted default colours and engine text mapping, wherever a contrast row or the select rendering reads them rather than an authored value.
- `stand_keyboard`, `stand_contrast`, `inert_subtree` and `select_interaction` as the standing witnesses of the a11y target, and the OPEN gap in a11y-plan §5 for the served pages.
- `:open` matching on `details` and `dialog`, read by the presence of the `open` attribute, and what a page shows in each state the new check asserts.
- The windowed shell's redraw route through its event loop, on the desktop window surface, read today only by the boot smoke and `focus_redraw`.
- `pseudo_element_flow_classification` run `for incremental in [false, true]`, an edit to an upstream test that the additivity note records.
- Both layout modes on every pipeline check: incremental and non-incremental layout stay identical under `incremental_oracle`, including any fixture this chunk adds.
- `NODE_FIELDS` thirteen, `VERBS` twelve and eight refusal causes stay as they are: a debt whose fix needs a new one is brought as a fork, so a design value may not rest on one.
- The engine-change rule: each engine fix applies to every document, so a default it changes reaches every page and stand app, and is ratified by the founder's word at this chunk's P4.

## Standing conventions
- S1 · A new test that reads text size or position builds `blitz-dom` with `system-fonts`, since without it text measures 0x0 and the assertion passes vacuously · `## Typography`
- S2 · A proof by driver `screenshot` takes `fonts-unavailable` as a refusal with nothing painted, never a shot with text skipped · `## Typography`
- S3 · A new test that waits or animates takes time from the harness's `VirtualClock`: settle never advances it, the load wait leaves it standing, and only `advance` moves it · `## Motion (calibrated to expression level `unrecorded`)`
- S4 · A new answer or refusal line on `escher-session` or its `mcp` role holds only JSON on stdout, with no colour or ESC byte, and keeps the exit-status classes · `## Surface: cli`
- S5 · A new fixture cites no project-wide colour, type, spacing or radius token, since no such set is measured · `## Color Palette`, `## Typography`, `## Spacing`, `## Border Radius`

## Not needed
- Browser toolbar icons, menu icon sizes, bundle logos and favicons: no debt touches the browser app's chrome.
- The browser app's stylesheet and the rdme GitHub-markdown stylesheet values: neither is a stand app nor a served page of this chunk.
- The desktop titlebar and the devtools overlay: the debts sit in the engine, the driver and the served pages.
- The custom-widget pattern: the canvas renderer and the CPU renderer's custom-paint gap are 0.4.0 capabilities, out of this entry.
- The mobile urlbar, back button and capture patterns: no debt reaches those browser-app controls.
- The WASM page's canvas sizing and its host page colour: the chunk adds nothing to the web canvas surface.
- The WPT runner's and the example binaries' terminal colours: the chunk adds no output to those binaries.
- Authored transition and animation values, the expression level and the absence of reduced-motion handling: no debt adds or changes an animation.
- Cells' selected-cell state: residual line 24 is out of this entry, though Cells' keyboard and contrast rows are in it.
- Brand identity, anti-patterns, the self-validation protocol and the design decisions log: each holds no recorded intent to bind a choice.
