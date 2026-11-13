@task_prose part=session
The session took upstream blitz main up to 563d3db73 by 29 no-commit cherry-picks into the engine, the paint crate, the examples and the tests, with no dependency moved.
The take widens engine behaviour, ratified by the founder as upstream's: pages decoded lossily, stylesheets decoded from bytes with BOM and charset, font-face sources skipped by format, the font element's face and size, valign, cellspacing and cellpadding hints, an img with empty alt read as decorative, a default-font fallback when no listed family is available.
It also changes hit-testing of inline roots (content-box offset only for inline children), the link cursor walk (DOM parent, not layout parent), transform resolution on a scale change, and form state for a checkbox or radio with no box.
Escher's own work pinned every action of every CI workflow to a commit SHA and widened the pin check to read every workflow, added one audit ignore, added two frame tests (blend modes and isolation, an inline box's own decorations) and re-recorded the committed Counter checks.
The committed Counter checks moved only in bounds and digests, the first back button from 73x25 to 72.578125x25.
@task_file n=1 path=apps/readme/src/{main,readme_application}.rs mark=merge
what: the readme example application, the markdown and page viewer that drives the engine in a window
@task_file n=2 path=examples/{paint_bench,screenshot}.rs mark=merge
what: the paint benchmark and the screenshot example, which renders a page to an image
@task_file n=3 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet, now with added textarea and select rules
@task_file n=4 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the accessibility tree builder, the roles it gives elements; an img with empty alt is now a generic container
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=merge
what: the base document, URL resolution of hrefs and the document flags that force all transforms to resolve after a scale change
@task_file n=6 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: font metrics queries for font-relative units, now with the default font when no listed family is available
@task_file n=7 path=packages/blitz-dom/src/form.rs mark=merge
what: form control state, checkbox and radio checkedness
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, attribute writes such as checked, cellpadding restyle, rel stylesheet matching and link href removal
@task_file n=9 path=packages/blitz-dom/src/net.rs mark=merge
what: network loading of stylesheets and font-face sources, their decoding and the font format choice
@task_file n=10 path=packages/blitz-dom/src/resolve.rs mark=merge
what: style and layout resolution, the transform resolve pass that now never skips an anonymous box
@task_file n=11 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the style engine bridge and its presentational hints, the font element's face and size, valign, cellspacing and cellpadding
@task_file n=12 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the style device, viewport size and scale as the style engine reads them
@task_file n=13 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the bridge from computed font styles to the text layout engine, the font family list and the default-font fallback
@task_file n=14 path=packages/blitz-dom/src/url.rs mark=merge
what: document URL handling, now resolving Windows absolute paths
@task_file n=15 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: layout damage, what a style change invalidates
@task_file n=16 path=packages/blitz-dom/src/layout/list.rs mark=merge
what: list item layout and its markers
@task_file n=17 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: writing-mode handling in layout
@task_file n=18 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: building an inline formatting context, its text and inline boxes
@task_file n=19 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline layout, placing inline boxes and embedded boxes on lines
@task_file n=20 path=packages/blitz-dom/src/node/element.rs mark=merge
what: element data on a node, attributes and element state
@task_file n=21 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the DOM node, hit-testing of a point and the cursor resolution walk
@task_file n=22 path=packages/blitz-dom/src/node/text.rs mark=merge
what: text node data
@task_file n=23 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint renderer, painting boxes, inline boxes, decorations and stacking contexts into a frame
@task_file n=24 path=packages/blitz/src/lib.rs mark=merge
what: the blitz umbrella crate's entry, the page loading front door
@task_file n=25 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility roles integration test, now with a case for an image with empty alt
@task_file n=26 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests module list, every integration target registered as a mod
@task_file n=27 path=tests/blitz-tests/tests/{anonymous_block_transform,font_element_attributes,font_family_fallback,hidden_checkbox_input,hit_inline_embedded_box,link_cursor,scale_change_overflow}.rs mark=merge
what: seven new upstream integration tests, transforms in an anonymous block, font element attributes, font family fallback, a label toggling a hidden checkbox, hitting an inline embedded box, the link cursor, overflow on a scale change
@task_file n=28 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the WPT runner is pinned to
@task_file n=29 path=.github/workflows/wpt.yml mark=merge
what: the WPT CI workflow, upstream-only and repository-guarded, now installing the wpt CLI by binstall with every action pinned to a SHA
@task_file n=30 path=.github/workflows/publish-browser.yml mark=merge
what: the browser publish workflow, upstream-only and repository-guarded, now taking the latest Android NDK with every action pinned to a SHA
@task_file n=31 path=.github/workflows/wpt-post-results.yml
what: the WPT results posting workflow, upstream-only, its actions pinned to SHAs
@task_file n=32 path=.github/scripts/test_ci_workflows.py
what: the CI workflow checks, now with a workflow pin check that reads every workflow and keeps each pin's ref as a trailing comment
@task_file n=33 path=deny.toml
what: the dependency audit policy, its advisory ignore list, plus one ignore for the browser's sqlite
@task_file n=34 path=tests/blitz-tests/checks/counter-cli.json
what: the committed Counter stand check for the command line role, its node bounds and digests
@task_file n=35 path=tests/blitz-tests/checks/counter-mcp.json
what: the committed Counter stand check for the mcp role, its node bounds and digests
@task_file n=36 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a frame test that blend modes and isolation paint as composited, in both layout modes
@task_file n=37 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a frame test that an inline box of blocks paints no decorations of its own, in both layout modes
@task_item n=1 part=touches
decision/no-generic-family-lays-out-sans: the engine now falls back to the default font when no listed family is available, beside the sans-serif rule for a list with no generic family.
@task_item n=2 part=touches
decision/root-relative-units-track-root-font: font metrics for font-relative units now query with the default-font fallback family list.
@task_item n=3 part=touches
decision/default-ua-sheet-via-default-config: the default user-agent stylesheet gained textarea and select rules.
@task_item n=4 part=touches
decision/single-select-box-shows-chosen-label and decision/select-list-engine-overlay: the default stylesheet's new select rule styles the select box these rows describe.
@task_item n=5 part=touches
decision/cursor-resolution: the cursor walk for a link now climbs DOM parents, not layout parents, so text and an inline-block inside a link take the pointer cursor.
@task_item n=6 part=touches
contract/shell-provider/set-cursor: the cursor it is handed over a link now comes from the DOM-parent walk.
@task_item n=7 part=touches
decision/hit-test-stops-at-paint-clip and decision/hit-testing-stops-at-paint-clip: hit-testing an inline root now applies its content-box offset only to inline children and the self hit; escher's padding-box clip early return still returns border-box coordinates for an inline root while the self hit returns content-box ones.
@task_item n=8 part=touches
decision/stacking-context-groups-paint-and-hit: a new frame test now checks that blend modes and isolation groups paint as composited.
@task_item n=9 part=touches
decision/logical-viewport-from-scale and contract/js-window/device-pixel-ratio: a scale change now sets a document flag that resolves every transform and rescales overflow of text-free boxes.
@task_item n=10 part=touches
contract/escher-session-tasks/counter: the committed Counter checks were re-recorded, the first back button's bounds from 73x25 to 72.578125x25.
@task_item n=11 part=touches
decision/body-right-bottom-margin-ignored: presentational hints widened (valign, cellspacing, cellpadding, the font element's face and size) in the same hint layer this rule sits in.
@task_item n=12 part=touches
decision/screenshot-never-skips-text: the screenshot example and the paint renderer were changed by the take.
@task_item n=13 part=touches
decision/boxless-focused-select-keeps-focus: a checkbox or radio's state is now made even with no box, a parallel no-box form-state rule.
@task_item n=14 part=touches
decision/test-engine-with-system-fonts and decision/tests-built-with-system-fonts: a new font-family fallback test reads the default font under the system-font test build.
@task_item n=15 part=touches
absence/project-type-scale and absence/engine-color-tokens: the default stylesheet's new rules add no type scale, font token or colour token.
@task_fact n=1 slug=typography
fact: When none of an element's listed font families is available, text lays out in the engine's default font.
@task_fact n=2 slug=typography
fact: The font element's face sets the font family when it is a valid family list, and its size maps to a font-size keyword; a size without digits or an invalid face is ignored.
@task_fact n=3 slug=iconography
fact: Text and an inline-block inside a link show the pointer cursor, resolved by walking DOM ancestors; text outside a link shows the text cursor.
@task_fact n=4 slug=spacing
fact: A table's cellspacing and cellpadding attributes and a cell's valign attribute act as presentational hints, and a cellpadding change restyles the table's descendants.
@task_fact n=5 slug=depth-strategy
fact: An inline box of blocks paints no backgrounds, borders or decorations of its own.
@task_item n=1 part=untouched
The motion areas: the virtual clock, instant scrolls, overlay scrollbar timings, select list motion and smooth scroll are not reached.
@task_item n=2 part=untouched
The select list events and the select accessibility contracts: open, close, choose and highlight behaviour and the open select's snapshot are unchanged.
@task_item n=3 part=untouched
The escher-session command, exit status, exit code, stdout, stderr, mcp role and session-error contracts, and the actions, coverage and UI graph lines are unchanged.
@task_item n=4 part=untouched
The WPT runner's output and result colour contracts and its secondary text colour: the runner's terminal output is unchanged.
@task_item n=5 part=untouched
The shell provider's window drag, minimize, maximize and decorations toggle, and the JS window's inner and outer sizes, are unchanged.
@task_item n=6 part=untouched
The dioxus-native configuration contracts (base colour, alpha mode, font context, font context on WASM) and the web-spa surface are unchanged.
@task_item n=7 part=untouched
Brand identity, border radius, the anti-patterns, self-validation and design decisions log areas hold no rows the session reaches.
