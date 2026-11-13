@task_prose part=session
The session took upstream main up to 563d3db73 as 29 no-commit cherry-picks (the founder ratified the take's behaviour changes as upstream's), and left out 99 upstream commits, each with its reason.
The accessibility builder now reads an img with alt="" as decorative, the generic container role, and the a11y leg runs 115 tests where it ran 114, over the same fourteen files.
The document mutator keeps a checkbox's or radio's checked state even when it has no box, so a label click toggles a hidden checkbox. Hit testing applies the inline root's content-box offset only to inline children, and a scale change now resolves every transform.
The default stylesheet gains textarea and select rules. Pages decode lossily, stylesheets decode from bytes, the font element's face and size are now presentational hints, and a default font is used when no listed family is available.
Escher added two frame tests (blend modes, an inline box's own decorations) and re-recorded the committed Counter checks, where only bounds moved. It pinned every action of every workflow to a commit SHA and moved the pin check to a new WorkflowPinTest. The audit gained one advisory ignore.
Retired nothing; no dependency, crate, port, env var, endpoint or public item was added.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the engine's accessibility tree builder, which maps elements to AccessKit roles, names and states
@task_file n=2 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, which writes attributes, keeps a native checkbox's or radio's checked state and restyles on attribute writes
@task_file n=3 path=packages/blitz-dom/src/form.rs mark=merge
what: the engine's form control state, a native checkbox's or radio's checked reading
@task_file n=4 path=packages/blitz-dom/src/document.rs mark=merge
what: the base document, which holds the node tree, focus, hit-testing entry points, URL resolution and the document flags
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the node, its hit-testing walk and its layout and DOM parents
@task_file n=6 path=packages/blitz-dom/src/node/element.rs mark=merge
what: the element data of a node, its attributes and form state
@task_file n=7 path=packages/blitz-dom/src/node/text.rs mark=merge
what: the text node data that text runs are shaped from
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet, the computed styles of textarea and select controls
@task_file n=9 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the style engine binding and its presentational hints, the font element's face and size, valign, cellspacing and cellpadding
@task_file n=10 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the style device, the viewport's scale and scheme as the style engine reads them
@task_file n=11 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the bridge from computed font styles to text shaping, and the default-font fallback
@task_file n=12 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: the font metrics query that resolves font families for ch and ex units
@task_file n=13 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the resolve pass that applies styles, layout and transforms after a change
@task_file n=14 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's resource loading, stylesheets decoded from bytes and font-face sources
@task_file n=15 path=packages/blitz-dom/src/url.rs mark=merge
what: the engine's URL resolution, including Windows absolute paths
@task_file n=16 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the layout damage tracking that decides what is relaid out after a change
@task_file n=17 path=packages/blitz-dom/src/layout/list.rs mark=merge
what: the list item layout, its markers
@task_file n=18 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: the writing mode handling of layout
@task_file n=19 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the inline layout builder that gathers text runs and inline boxes
@task_file n=20 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the inline layout pass that places text runs and inline boxes
@task_file n=21 path=packages/blitz-paint/src/render.rs mark=merge
what: the painter, which renders the document's boxes, decorations and blend modes to a frame
@task_file n=22 path=packages/blitz/src/lib.rs mark=merge
what: the engine's top-level launch crate
@task_file n=23 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility role integration tests, one of the fourteen named accessibility test files the a11y leg runs
@task_file n=24 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests target list, a mod line per integration test file
@task_file n=25 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the WPT run is pinned to
@task_file n=26 path=.github/workflows/publish-browser.yml mark=merge
what: the upstream-only browser publish workflow, guarded to the upstream repository
@task_file n=27 path=.github/workflows/wpt.yml mark=merge
what: the upstream-only WPT workflow, guarded to the upstream repository
@task_file n=28 path=tests/blitz-tests/tests/anonymous_block_transform.rs mark=merge
what: an upstream integration test, a transform inside an anonymous block resolved on hover and on a style write
@task_file n=29 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an upstream integration test, the font element's size and face mapped to font size and family
@task_file n=30 path=tests/blitz-tests/tests/font_family_fallback.rs mark=merge
what: an upstream integration test, ch units of unavailable families read with the default font
@task_file n=31 path=tests/blitz-tests/tests/hidden_checkbox_input.rs mark=merge
what: an upstream integration test, a label click toggling a hidden checkbox
@task_file n=32 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an upstream integration test, hit-testing an inline embedded box where it is painted
@task_file n=33 path=tests/blitz-tests/tests/link_cursor.rs mark=merge
what: an upstream integration test, the pointer cursor over text and an inline block inside a link
@task_file n=34 path=tests/blitz-tests/tests/scale_change_overflow.rs mark=merge
what: an upstream integration test, a scale change rescaling the overflow of boxes
@task_file n=35 path=.github/workflows/wpt-post-results.yml
what: the upstream-only WPT results workflow, guarded to the upstream repository
@task_file n=36 path=.github/scripts/test_ci_workflows.py
what: the CI workflow checks, among them the action pin check now held by WorkflowPinTest over every workflow
@task_file n=37 path=tests/blitz-tests/checks/counter-cli.json
what: a committed stand check of the Counter example, read through the CLI, its recorded bounds and digests
@task_file n=38 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed stand check of the Counter example, read through MCP, its recorded bounds and digests
@task_file n=39 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a frame test of blend modes and isolation painted as composited, in both layout modes
@task_file n=40 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a frame test that an inline box of blocks paints no decorations of its own, in both layout modes
@task_item n=1 part=touches
Role mapping by HTML-AAM and native element roles: an img with alt="" now maps to the generic container role, not image. Rows on the role of an img or an element role table may need the decorative case.
@task_item n=2 part=touches
Role from role attribute, element, else Unknown: the element arm now has a case keyed on an attribute value (an empty alt), which this rule may need to state.
@task_item n=3 part=touches
Interactive nodes carry a name, and the accessible name precedence: a decorative img is now a generic container with no image role, so rules that read an img's name from alt meet a role change.
@task_item n=4 part=touches
The a11y CI leg, the accessibility test leg in the slow tier, and accessibility checks gating CI: the leg runs 115 tests where it ran 114, over the same fourteen files. Where a row's cell states 114, it moves to 115; the fourteen named files stand.
@task_item n=5 part=touches
The native checkbox or radio checked state (the accessibility tree state, the tree's native checked contract, the snapshot's checked, the source of the checked reading, the native checkbox toggle): a checked write is read back into an existing checkbox state, and the state exists even for a checkbox or radio with no box.
@task_item n=6 part=touches
A checkbox or radio click toggles, fires input and focuses, and a label names its bound labelable element: a label click now toggles a hidden (display none) checkbox. Rows on label clicks and on hidden content kept out of the tree meet a hidden control whose state still changes.
@task_item n=7 part=touches
An inert element takes no hit, hit-testing of inert nodes, the driver's synthetic click at the node's centre, and driver click refusals for a covered node: the hit-testing walk now applies an inline root's content-box offset only to inline children. A hit inside an inline embedded box lands where it is painted.
@task_item n=8 part=touches
A hidpi or zoom change reshapes text, the viewport carries scheme and zoom, and a scheme change counts as a device change: a scale change now sets a document flag that resolves every transform and rescales overflow.
@task_item n=9 part=touches
Stand contrast measured from computed styles, the stand reads no lint finding at boot, and the lint's contrast finding bounds: the default stylesheet gains textarea and select rules, and text metrics moved. The committed Counter checks were re-recorded with only bounds and digests moved; the first back-btn went from 73x25 to 72.578125x25.
@task_item n=10 part=touches
Text contrast required ratio per SC 1.4.3, which reads font size and weight: the font element's size now maps to a font-size keyword and its face sets the family, and an unavailable family falls back to the default font. A page's large-text reading can move.
@task_item n=11 part=touches
What the engine's changed set marks: the never-list of changed-set write sites takes the select-list qualifier (a scroll that closes an open list marks its select). This lands in architecture by the operator's direction, and an a11y row stating the never-list without it reads stale.
@task_item n=12 part=touches
Accessibility tree built through the document's own method, and the adapter tree equals the harness tree: the builder they rule over changed (the empty alt arm), and both read the same changed tree.
@task_item n=13 part=touches
Rows that hold a both-layout-modes convention for the engine's integration tests: six of the seven upstream test targets run one layout mode, recorded as exceptions. The two new frame tests run both modes, and the stand checks still run both.
@task_fact n=1 slug=7-screen-reader-support
fact: An img whose alt is the empty string is decorative: the accessibility tree gives it the generic container role, not image. This was ratified by the founder as upstream's behaviour.
@task_fact n=2 slug=keyboard-navigation
fact: A native checkbox or radio keeps its checked state with no box, and a click on its label toggles it even when it is hidden (display none). This was ratified by the founder as upstream's behaviour.
@task_item n=1 part=untouched
Keyboard contracts of the range input, the single-choice select with its list open or closed, the multiple select, the seven_guis Cells grid and the CRUD list: no key handling, typeahead or list navigation changed.
@task_item n=2 part=untouched
Focus rules: Tab and Shift+Tab, the stated tab order, what is focusable, the focus fixup on inertness, focus-visible, and focus on a removed node. No focus path changed.
@task_item n=3 part=untouched
The platform adapter and assistive technology reach: accesskit_xplat, the accessibility feature defaults, no adapter on the stand binaries, and the session library enabling accessibility. No dependency or feature moved.
@task_item n=4 part=untouched
The snapshot text form, the actions answer, handler coverage, MCP tool results, served node ids, the snapshot diff and the UI graph state registration: no driver, MCP or snapshot shape changed.
@task_item n=5 part=untouched
Password masking and the password caret: no password value path changed.
@task_item n=6 part=untouched
Cognitive accessibility rows (the invalid state stated beyond colour or class, the shared stand tab-order table): nothing in them moved.
@task_item n=7 part=untouched
The obs log format named in the assertion harness overview: escher's sink has no new site, only two feature-gated upstream warn lines folded from three.
