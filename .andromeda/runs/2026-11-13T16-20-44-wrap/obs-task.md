@task_prose part=session
The session took upstream DioxusLabs/blitz main up to 563d3db73 as 29 no-commit cherry-picks into the engine crates, the readme app, the examples, the WPT pin and the blitz-tests targets, with no dependency moved.
The take widens engine behaviour, ratified by the founder as upstream's: pages decoded lossily with no panic on non-UTF-8, stylesheets decoded from bytes with BOM and charset, font-face sources skipped by format and extension, the font element's face and size, valign, cellspacing and cellpadding hints, an empty alt read as decorative, a default-font fallback, Windows absolute paths resolved.
The engine's resolve_url now returns nothing where it panicked on an unresolvable href, and removing a link's href with no stylesheet held returns early where it was unreachable.
The sink census moved 2 feature-gated tracing warn lines in the font-face source skip, folding 3 removed, 0 new; the smoke logged service.name=seven_guis.
Every live uses line of every workflow is now pinned to a 40-hex SHA, and the pin check moved from CiWorkflowTest to a new WorkflowPinTest that reads every workflow; ci-scripts Ran 110 to Ran 112.
The two committed Counter kept checks were re-recorded by the script, only bounds and digests moving; one advisory ignore, RUSTSEC-2026-0459, joined the audit; two frame tests and seven upstream targets were added.
@task_file n=1 path=apps/readme/src/main.rs mark=merge
what: the readme browser app's entry, behind the log-times features' frame and phase timings
@task_file n=2 path=apps/readme/src/readme_application.rs mark=merge
what: the readme browser app's application loop, its in-app overlay and timings
@task_file n=3 path=examples/paint_bench.rs mark=merge
what: the bench example that prints in-process paint timings
@task_file n=4 path=examples/screenshot.rs mark=merge
what: the screenshot example that prints in-process timings
@task_file n=5 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet
@task_file n=6 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the engine's accessibility tree builder, the roles it reads
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine's base document, its URL resolution and document flags
@task_file n=8 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: the engine's font metrics query
@task_file n=9 path=packages/blitz-dom/src/form.rs mark=merge
what: the engine's form handling, checkbox and radio state
@task_file n=10 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's document mutator, the stylesheet link and checked writes
@task_file n=11 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's network loading of stylesheets and font-face sources, holding feature-gated tracing warn events under a blitz target
@task_file n=12 path=packages/blitz-dom/src/resolve.rs mark=merge
what: the engine's resolve phases, the transform resolution
@task_file n=13 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the engine's style bridge, its presentational hints
@task_file n=14 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: the engine's style to text-layout conversion, the font family fallback
@task_file n=15 path=packages/blitz-dom/src/url.rs mark=merge
what: the engine's URL resolution of document paths
@task_file n=16 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage tracking
@task_file n=17 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout
@task_file n=18 path=packages/blitz-dom/src/node/element.rs mark=merge
what: the engine's element node data
@task_file n=19 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's node, its hit testing and cursor walk
@task_file n=20 path=packages/blitz-dom/src/node/text.rs mark=merge
what: the engine's text node data
@task_file n=21 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint crate's renderer, behind the layer counters
@task_file n=22 path=packages/blitz/src/lib.rs mark=merge
what: the blitz umbrella crate's entry
@task_file n=23 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility role integration test
@task_file n=24 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests list of test modules
@task_file n=25 path=wpt/WPT_COMMIT mark=merge
what: the WPT pinned commit the WPT runner reads
@task_file n=26 path=.github/workflows/publish-browser.yml mark=merge
what: the upstream-only browser publishing workflow, repository-guarded
@task_file n=27 path=.github/workflows/wpt.yml mark=merge
what: the upstream-only WPT workflow that runs WPT and publishes its scores, repository-guarded
@task_file n=28 path=tests/blitz-tests/tests/anonymous_block_transform.rs mark=merge
what: an upstream integration test of transforms inside an anonymous block
@task_file n=29 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an upstream integration test of the font element's face and size
@task_file n=30 path=tests/blitz-tests/tests/font_family_fallback.rs mark=merge
what: an upstream integration test of the default-font fallback
@task_file n=31 path=tests/blitz-tests/tests/hidden_checkbox_input.rs mark=merge
what: an upstream integration test of a label toggling a hidden checkbox
@task_file n=32 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an upstream integration test of hit testing an inline embedded box
@task_file n=33 path=tests/blitz-tests/tests/link_cursor.rs mark=merge
what: an upstream integration test of the cursor over a link
@task_file n=34 path=tests/blitz-tests/tests/scale_change_overflow.rs mark=merge
what: an upstream integration test of overflow after a scale change
@task_file n=35 path=deny.toml
what: the dependency audit configuration, its advisory ignore set
@task_file n=36 path=.github/workflows/wpt-post-results.yml
what: the upstream-only workflow that posts WPT results, repository-guarded
@task_file n=37 path=.github/scripts/test_ci_workflows.py
what: the ci-scripts suite that checks the CI workflows, its CiWorkflowTest and new WorkflowPinTest guards
@task_file n=38 path=tests/blitz-tests/checks/counter-cli.json
what: a committed kept check file of the Counter stand, its accepted answer and answer digest
@task_file n=39 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed kept check file of the Counter stand over the mcp role, its accepted answer and answer digest
@task_file n=40 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a frame test of blend modes and isolation, both layout modes
@task_item n=1 part=touches
The engine's tracing behind the cargo feature and set by the build's invocation: 2 tracing warn lines in the font-face source skip moved, still feature-gated, folding 3 removed and adding 0 new sites.
@task_item n=2 part=touches
Engine targets print safe fields only, and the sink target allowlist: the moved warn lines sit under a blitz target with the URL redacted, which the rule over engine targets reads.
@task_item n=3 part=touches
escher-telemetry's scrub set url and error items: the moved font-face warn lines carry a source URL the sink scrubs.
@task_item n=4 part=touches
A failed stylesheet leaves the pending critical set: stylesheet loading changed, decoded from bytes with BOM and charset, rel matched case-insensitively, and an href removed from a link with no stylesheet held now returns early where it was unreachable.
@task_item n=5 part=touches
No file: loads from non-file: documents: the engine's URL resolution changed, resolve_url returning nothing on an unresolvable href (skipped by its callers) and Windows absolute paths now resolved.
@task_item n=6 part=touches
Error capture of engine panics: an unresolvable href, a non-UTF-8 page and a link's href removal no longer panic, so fewer paths reach the panic hook.
@task_item n=7 part=touches
WPT scores published from upstream only, and WPT results published upstream only: the WPT, WPT-post-results and browser-publishing workflows were pinned and wpt installs its CLI by cargo-binstall; every repository guard is byte for byte.
@task_item n=8 part=touches
The WPT report message and subtest message: the WPT pin moved, so the WPT runner reads a new WPT commit.
@task_item n=9 part=touches
CI on GitHub workflows: every workflow's actions are now pinned to a 40-hex SHA with the ref as a trailing comment, checked by the new WorkflowPinTest over every workflow.
@task_item n=10 part=touches
A guard moved: test_j and test_k left CiWorkflowTest for WorkflowPinTest; any row whose guard cell names CiWorkflowTest for the pin or toolchain check now names a stale guard, and rows guarded by the other CiWorkflowTest assertions, such as the per-leg log artifact on failure and the coverage artifact on success, stand.
@task_item n=11 part=touches
Committed kept checks hold no secrets and no private data: both Counter checks were re-recorded; a secret-shaped probe matched only the stand's task element ids, a false positive.
@task_item n=12 part=touches
The kept check file's accepted, answer and answer digest items: in the re-recorded Counter checks only bounds and digests moved, the first back-btn bound from 73x25 to 72.578125x25.
@task_item n=13 part=touches
Metric coverage's in-process timings: the bench and screenshot examples, the readme app and the paint renderer each took upstream lines; still printed or shown in process, no metric emission or exporter added.
@task_item n=14 part=touches
escher's sink log line service.name and the sink's resource keys: the windowed smoke under RUST_LOG=info logged service.name=seven_guis and stayed up 10 s.
@task_item n=15 part=touches
The fork CI's two artifacts and local state never uploaded: the workflows changed are upstream-only and repository-guarded, so no new upload or artifact entered the fork's CI.
@task_fact n=1 slug=7-error-capture-reporting
fact: The engine no longer panics on an unresolvable href, a non-UTF-8 page or the removal of a link's href with no stylesheet held: the href is skipped, the page is decoded lossily and the removal returns early, each with no event.
@task_item n=1 part=untouched
The driver's command and tool_call spans, their fields, the refusal cause names and the tool_call endings: no driver code or span moved.
@task_item n=2 part=untouched
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, transcripts and cleanup; no agent-run, status or log-shape change.
@task_item n=3 part=untouched
escher-session's roles, commands, exit codes, error object, host wire and stdout, and the coverage line: no escher-session code moved.
@task_item n=4 part=untouched
escher's sink log line format, its event and span line layout, escaping, scrub and RUST_LOG configuration, and the telemetry-installed and panic events: no telemetry crate code moved.
@task_item n=5 part=untouched
The select list, snapshot, graph, actions answer and page script event rows of PII scrubbing: the changed-set qualifier is an architecture row and no select or snapshot logging moved.
@task_item n=6 part=untouched
Script diagnostics drain, script error source labels, the window error event and window.onerror: no script runtime code moved.
@task_item n=7 part=untouched
The recording file, recording bounds, recording and session state directories and the session lifecycle: unmoved.
@task_item n=8 part=untouched
The absences of a telemetry backend, metrics exporters, metric emission and an error-reporting service, and the no-OTel-export decision: no dependency entered and none of these was added.
