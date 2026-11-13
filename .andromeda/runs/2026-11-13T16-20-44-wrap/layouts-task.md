@task_prose part=session
The session took upstream blitz main up to 563d3db73 as 29 no-commit cherry-picks into the engine (DOM, layout, paint, net, url), with no dependency, crate, public item, port, env var or endpoint added.
The take widens engine behaviour: lossy page decoding, stylesheets decoded from bytes, the font element's face and size, valign, cellspacing and cellpadding hints, an empty-alt image read as decorative, a default-font fallback, resolve_url returning None instead of panicking, checkbox state kept with no box, link cursor walked by DOM parent, hit testing of inline roots and transforms of anonymous boxes; the founder ratified these as upstream's.
Escher pinned every live action of every CI workflow to a 40-hex SHA and widened the pin check from the CI workflow to every workflow, its two pin tests moved into a new WorkflowPinTest with two cases added (ci-scripts Ran 110 to 112).
It added two frame tests (blend modes and isolation, an inline box of blocks painting no decorations of its own) in both layout modes, nine blitz-tests targets in all, six of the seven upstream ones running one layout mode.
It re-recorded the two committed Counter checks, where only bounds and digests moved (the first back-btn 73x25 to 72.578125x25), and added one cargo-deny advisory ignore, RUSTSEC-2026-0459.
Not listed as files below, also picked from upstream unchanged in kind: the new upstream test targets anonymous_block_transform, font_family_fallback, hidden_checkbox_input, link_cursor and scale_change_overflow.
@task_file n=1 path=apps/readme/src/main.rs mark=merge
what: the readme browser app's entry, the desktop window that opens a document
@task_file n=2 path=apps/readme/src/readme_application.rs mark=merge
what: the readme browser app's application and window handling
@task_file n=3 path=examples/paint_bench.rs mark=merge
what: the paint_bench command line, its arguments and backend
@task_file n=4 path=examples/screenshot.rs mark=merge
what: the screenshot example that renders a page to an image
@task_file n=5 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet, now with textarea and select defaults
@task_file n=6 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the accessibility tree builder that gives each element its role, name and state
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: the document, its url resolution, document flags and transform resolution
@task_file n=8 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: font metrics queries used for ch and font sizes, now with the default-font fallback
@task_file n=9 path=packages/blitz-dom/src/form.rs mark=merge
what: form controls, checkbox and radio state
@task_file n=10 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator that applies attribute writes, checked state, stylesheet links and restyles
@task_file n=11 path=packages/blitz-dom/src/net.rs mark=merge
what: the network provider's stylesheet and font-face loading
@task_file n=12 path=packages/blitz-dom/src/resolve.rs mark=merge
what: style and layout resolution, transforms resolved on a scale change
@task_file n=13 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the style engine glue and its presentational hints from HTML attributes
@task_file n=14 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the style device, viewport and scale for media queries
@task_file n=15 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the bridge from computed style to text layout, font family lists and fallback
@task_file n=16 path=packages/blitz-dom/src/url.rs mark=merge
what: document url parsing, now resolving Windows absolute paths
@task_file n=17 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: layout damage, what a style change re-lays out
@task_file n=18 path=packages/blitz-dom/src/layout/list.rs mark=merge
what: list item layout and markers
@task_file n=19 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: writing mode handling in layout
@task_file n=20 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: building inline layout from text and inline boxes
@task_file n=21 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline layout, positions of inline boxes and text runs
@task_file n=22 path=packages/blitz-dom/src/node/element.rs mark=merge
what: element data, attributes and per-element state
@task_file n=23 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the node, its hit testing (hit_inner) and the cursor over a link
@task_file n=24 path=packages/blitz-dom/src/node/text.rs mark=merge
what: text node data and decoding
@task_file n=25 path=packages/blitz-paint/src/render.rs mark=merge
what: the painter that renders boxes, blend modes, decorations and clips to a frame
@task_file n=26 path=packages/blitz/src/lib.rs mark=merge
what: the blitz top-level crate that loads and opens a document in a window
@task_file n=27 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility role integration tests, now with an empty-alt image as decorative
@task_file n=28 path=tests/blitz-tests/tests/all.rs mark=merge
what: the list of blitz-tests targets, nine mod lines added
@task_file n=29 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the WPT run is pinned to
@task_file n=30 path=.github/workflows/wpt.yml mark=merge
what: the upstream-only WPT workflow, repository-guarded, now with every action pinned to a SHA
@task_file n=31 path=.github/workflows/publish-browser.yml mark=merge
what: the upstream-only browser publish workflow, repository-guarded, now with every action pinned to a SHA
@task_file n=32 path=.github/workflows/wpt-post-results.yml
what: the upstream-only WPT results workflow, its actions pinned to a SHA
@task_file n=33 path=.github/scripts/test_ci_workflows.py
what: the ci-scripts suite that checks the CI workflows, now with WorkflowPinTest over every workflow
@task_file n=34 path=deny.toml
what: the dependency audit configuration and its advisory ignore list
@task_file n=35 path=tests/blitz-tests/checks/counter-cli.json
what: the committed Counter stand check recorded through the session command line
@task_file n=36 path=tests/blitz-tests/checks/counter-mcp.json
what: the committed Counter stand check recorded through the MCP server
@task_file n=37 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a frame test of blend modes and isolation, in both layout modes
@task_file n=38 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a frame test that an inline box of blocks paints no decorations of its own, in both layout modes
@task_file n=39 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an upstream test of the font element's face and size attributes
@task_file n=40 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an upstream test that an embedded inline box is hit where it is painted
@task_item n=1 part=touches
The committed Counter checks were re-recorded: a row holding a Counter element's bounds or a check file's digest (the first back-btn now 72.578125x25, not 73x25) holds a moved value; rows on recorded checks as replayable and on the check file they write are touched by the re-recording.
@task_item n=2 part=touches
Hit testing changed in the engine: an inline root's content-box offset now applies only to inline children and the self hit; escher's own clips_to_padding_box early return now returns border-box x and y where the self hit returns content-box ones; rows on a visible region through clipping boxes, open list rows reading their own bounds, and into-view scroll read through this hit path.
@task_item n=3 part=touches
An image with an empty alt now reads as a generic container (decorative): a row on how the stand gets its names, roles and states from attributes alone sits beside this role change.
@task_item n=4 part=touches
Checkbox and radio state is now made even when the control has no box, and a checked write is read back into an existing state: rows on stand controls and their boot reading (the flight type select, held instance as fresh boot) rule over the mutator this changed.
@task_item n=5 part=touches
Fonts: a default font is now used when no listed family is available: a row on the session's fonts-unavailable refusal or on fonts the stand needs sits beside this fallback.
@task_item n=6 part=touches
The network provider now decodes stylesheets from bytes with BOM and charset, skips font-face sources by format and extension, decodes pages lossily, and resolve_url returns None instead of panicking: a row on the load page alone having a network provider rules over this provider.
@task_item n=7 part=touches
The default user-agent stylesheet gains textarea and select defaults: rows on the stand's look, its accent colour, the flight type select and the fixed document skeleton read through those defaults.
@task_item n=8 part=touches
A scale change now sets a flag to resolve every transform, and anonymous boxes are never skipped in transform resolution: a row on the headless stand's pinned viewport and scale reads through this.
@task_item n=9 part=touches
paint_bench's command line was touched by the take (one line): rows on its arguments and backend should be read against it.
@task_item n=10 part=touches
Six of the seven new upstream test targets run one layout mode, recorded as exceptions in test-plan: a row that says checks or tests run in both layout modes meets these exceptions (the stand checks themselves stay in both modes).
@task_item n=11 part=touches
The pin check widened from the CI workflow to every workflow, its tests moved from CiWorkflowTest to WorkflowPinTest: any row whose guard cell names CiWorkflowTest for action pinning holds a moved guard.
@task_item n=12 part=touches
The readme browser app and the blitz top-level crate that open a document in a desktop window were touched by the take: rows on the desktop window and the document skeleton read through them.
@task_item n=1 part=untouched
The stand's stable element ids: no id was added, renamed or removed; the checks' sweep for secret-shaped strings matching task- ids is a false positive, not a change.
@task_item n=2 part=untouched
The agent-run and cold-agent command lines, their usage, exit codes, stdout, run log, record directory and Windows pass-through: no change to either script.
@task_item n=3 part=untouched
The escher-session command line, its actions, exit codes, help, lint, graph, snapshot, screenshot, served tasks and refusals other than fonts, and the MCP server's tools, protocol revisions, stdio and result bound: no new verb, port, socket, env var, IPC method or endpoint.
@task_item n=4 part=untouched
The multi_doc command line and its environment variables: unchanged.
@task_item n=5 part=untouched
The ShellProvider window chrome controls (drag, maximize, minimize, decorations, close): no window chrome API moved.
@task_item n=6 part=untouched
The cold-agent rules (pipe builds nothing, prompt names no id, drives through the MCP server) and caller input never written back: unchanged.
@task_item n=7 part=untouched
Rows that say a JSON key is left out when absent share the words left out with the 99 upstream commits this take left out; they are true claims, no change.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces and the decisions log: no fact of this session reaches them; the absence of arbitrary index.html templates still holds.
