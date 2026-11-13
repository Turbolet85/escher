@task_prose part=session
Took upstream blitz main up to 563d3db73 as 29 no-commit cherry-picks; no dependency, crate, port, endpoint or public item entered, and the lockfile and manifests are unchanged.
The take brings engine behaviour widenings ratified by the founder as upstream's: lossy page decoding, stylesheets decoded from bytes, font element face and size, valign, cellspacing and cellpadding hints, an empty alt read as decorative, a default-font fallback, a non-panicking URL resolve, hit and link-cursor fixes, and anonymous-box transforms resolved.
Seven upstream integration targets and two escher frame tests (blend modes and isolation, an inline box of blocks painting no decorations of its own) were added; six of the seven upstream targets run one layout mode, recorded as exceptions.
The action pin check widened from the main CI workflow to every workflow, its two pin tests moved into a new WorkflowPinTest with two more cases (ci-scripts Ran 110 to 112), and all four workflows are now pinned to 40-hex SHAs.
The two committed Counter checks were re-recorded by the recording script (only bounds and digests moved), the dependency audit gained one advisory ignore, RUSTSEC-2026-0459, and clippy under rustc 1.100 went green.
Measured: run all 768 to 785 passed, run stand 262 unmoved, a11y leg 114 to 115 tests over the same fourteen files, per-OS tallies moved, stand-repeat 1248 unmoved, CI green at attempt 3 after a runner-capacity cancel.
@task_file n=1 path=apps/readme/src mark=merge
what: the readme browser app, an upstream example app
@task_file n=2 path=examples/paint_bench.rs mark=merge
what: the paint benchmark example, ignored by default
@task_file n=3 path=examples/screenshot.rs mark=merge
what: the headless screenshot example, an example fixture read by a human
@task_file n=4 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's user-agent stylesheet for form controls such as textarea and select
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the accessibility tree builder, the roles the a11y leg reads (an img with empty alt now a generic container)
@task_file n=6 path=packages/blitz-dom/src/document.rs mark=merge
what: the base document, its URL resolve (no longer panics) and its document flags set on a scale change
@task_file n=7 path=packages/blitz-dom/src/font_metrics.rs mark=merge
what: font metrics, the default font used when no listed family is available
@task_file n=8 path=packages/blitz-dom/src/form.rs mark=merge
what: form controls, checkbox and radio state
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, the engine's write path for attributes, checked state, link rel and href
@task_file n=10 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's network loading of stylesheets and font-face sources
@task_file n=11 path=packages/blitz-dom/src/resolve.rs mark=merge
what: style and layout resolve, transforms resolved for anonymous boxes
@task_file n=12 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the style engine bridge, presentational hints of HTML attributes
@task_file n=13 path=packages/blitz-dom/src/stylo_device.rs mark=merge
what: the style device, the viewport scale the style engine reads
@task_file n=14 path=packages/blitz-dom/src/stylo_to_parley.rs mark=merge
what: style to text-layout conversion, the font family list and its fallback
@task_file n=15 path=packages/blitz-dom/src/url.rs mark=merge
what: URL resolution of document and file paths, Windows absolute paths
@task_file n=16 path=packages/blitz-dom/src/layout mark=merge
what: the layout engine, damage, list items, writing mode and inline layout
@task_file n=17 path=packages/blitz-dom/src/node mark=merge
what: the DOM node, hit testing, element and text nodes, the link cursor walk
@task_file n=18 path=packages/blitz-paint/src/render.rs mark=merge
what: the scene painter, the frame render the pixel tests read
@task_file n=19 path=packages/blitz/src/lib.rs mark=merge
what: the blitz shell crate, the windowed entry point
@task_file n=20 path=tests/blitz-tests/tests/accessibility_roles.rs mark=merge
what: the accessibility roles integration test, a member of the a11y leg
@task_file n=21 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests shared test entry listing every target module, test false kept
@task_file n=22 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the conformance runner is pinned to
@task_file n=23 path=.github/workflows/publish-browser.yml mark=merge
what: the upstream-only browser publishing workflow behind the repository guard
@task_file n=24 path=.github/workflows/wpt.yml mark=merge
what: the upstream-only WPT conformance workflow behind the repository guard, installing the wpt cli
@task_file n=25 path=tests/blitz-tests/tests/anonymous_block_transform.rs mark=merge
what: an upstream integration target, a transform inside an anonymous block
@task_file n=26 path=tests/blitz-tests/tests/font_element_attributes.rs mark=merge
what: an upstream integration target, the font element's face and size
@task_file n=27 path=tests/blitz-tests/tests/font_family_fallback.rs mark=merge
what: an upstream integration target, the default font for unavailable families
@task_file n=28 path=tests/blitz-tests/tests/hidden_checkbox_input.rs mark=merge
what: an upstream integration target, a label click toggling a hidden checkbox
@task_file n=29 path=tests/blitz-tests/tests/hit_inline_embedded_box.rs mark=merge
what: an upstream integration target, hit testing an inline embedded box
@task_file n=30 path=tests/blitz-tests/tests/link_cursor.rs mark=merge
what: an upstream integration target, the pointer cursor over a link
@task_file n=31 path=tests/blitz-tests/tests/scale_change_overflow.rs mark=merge
what: an upstream integration target, overflow rescaled on a scale change
@task_file n=32 path=deny.toml
what: the dependency audit configuration, the advisory ignore set
@task_file n=33 path=.github/workflows/wpt-post-results.yml
what: the upstream-only WPT results posting workflow, its action pins
@task_file n=34 path=.github/scripts/test_ci_workflows.py
what: the CI Python scripts unit tests, the guard of the CI workflow and action-pin rules
@task_file n=35 path=tests/blitz-tests/checks/counter-cli.json
what: a committed recorded check of the Counter app, command-line surface
@task_file n=36 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed recorded check of the Counter app, MCP surface
@task_file n=37 path=tests/blitz-tests/tests/blend_mode_frame.rs
what: a pixel frame test of mix-blend-mode and isolation, both layout modes
@task_file n=38 path=tests/blitz-tests/tests/inline_box_decorations_frame.rs
what: a pixel frame test that an inline box of blocks paints no decorations of its own, both layout modes
@task_item n=1 part=touches
decision ci-actions-pinned-to-sha is stale: the pin rule and its check now read every workflow file, not the main CI workflow alone, and all four workflows are pinned.
@task_item n=2 part=touches
decision actions-pinned-to-commit-sha has a stale guard: the pin tests test_j and test_k moved from CiWorkflowTest to the new WorkflowPinTest, which also checks each pin keeps its ref as a trailing comment.
@task_item n=3 part=touches
decision ci-rust-cache: the WPT workflow's Rust cache and checkout are now SHA-pinned to their own commits, which differ from the main CI workflow's pins.
@task_item n=4 part=touches
decisions wpt-upstream-only and upstream-repository-guard: the upstream-only workflows changed (wpt cli installed by cargo-binstall, latest Android NDK) with every repository guard kept byte for byte.
@task_item n=5 part=touches
contract wpt-cli and decision wpt-conformance-runner: the WPT workflow now installs the wpt cli by cargo-binstall.
@task_item n=6 part=touches
decision ci-scripts-unit-tested and the CI workflow guard rows: the ci-scripts suite gained two tests, Ran 110 to 112; no row states the count.
@task_item n=7 part=touches
decision layout-tests-in-both-layout-modes: six of the seven upstream targets taken run one layout mode, kept as written and recorded as exceptions; stand-checks-in-both-layout-modes binds the stand checks only and holds.
@task_item n=8 part=touches
decisions one-test-target-per-behaviour, blitz-tests-one-target-per-file, one-test-target-per-file, blitz-tests-single-test-target and blitz-tests-manifest-difference: nine blitz-tests targets added, the shared entry gains nine module lines with test false kept.
@task_item n=9 part=touches
decisions recorded-checks-kept-fresh, checks-byte-equal-to-recording and contract record-checks-script: the two committed Counter checks re-recorded by the script, only bounds and digests moved.
@task_item n=10 part=touches
contract committed-checks excluded-content and decision checks-hold-no-private-data: a secret-shaped probe over the committed checks matches the stand's task element ids, a false positive.
@task_item n=11 part=touches
decision a11y-leg-membership: the a11y leg runs 115 tests over the same fourteen files, an empty-alt image case added to the accessibility roles test.
@task_item n=12 part=touches
decision wpt-pinned-commit: the recorded WPT commit moved; the row names no sha.
@task_item n=13 part=touches
decision dependency-audit-advisories-and-licenses: the audit ignore set gains RUSTSEC-2026-0459 (libsqlite3-sys via rusqlite 0.32 in the browser) with its written reason.
@task_item n=14 part=touches
decision clippy-legs: clippy under rustc 1.100 on the CI image went red then green by an upstream pick; the plan's clippy entries run under 1.100.0 on the host.
@task_item n=15 part=touches
decisions tally-move-attributed-by-test-name and ci-log-ansi-strip: per-OS tallies re-read by test name with the ANSI strip; fetching a job log holding escape sequences needs the allow-escape-sequences flag.
@task_item n=16 part=touches
contract harness-contrast blend-mode and decisions frames-compared-whole, pixel-tests-on-cpu-buffer, contrast-reader-reads-only: blend modes and isolation now have a pixel frame test beside the contrast reader's mix-blend-mode skip.
@task_item n=17 part=touches
decision mutation-controlled-tests: both frame tests read red under their control first; the pin check's control could not run in the control tool's clone and was measured by the operator outside the tree.
@task_item n=18 part=touches
decisions font-tests-skip-without-font, font-metric-tests-never-skip and font-assertions-on-system-fonts: new font targets read the default font used when no listed family is available, and the font element's face and size.
@task_item n=19 part=touches
contracts harness-input pointer-coordinates and clipped-click: inline-root hit testing now applies the content-box offset only to inline children; escher's clip early return still returns border-box coordinates, read by no check.
@task_item n=20 part=touches
decision key-after-dropped-anonymous-block: transform resolve now never skips an anonymous box.
@task_item n=21 part=touches
decision upstream-mounted-flush-test-left-out: the upstream test left out until the Dioxus 0.8 move re-read as standing; Dioxus stays 0.7.3.
@task_item n=22 part=touches
decision file-urls-only-for-file-documents: URL resolution now resolves Windows absolute paths and returns nothing where it panicked on an unresolvable href.
@task_item n=23 part=touches
decision ci-on-github-actions and the CI verdict: a job cancelled for runner capacity reads red; the operator re-ran it and the latest attempt is read.
@task_fact n=1 slug=integration-test-strategy
fact: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions to layout assertions in both modes.
@task_fact n=2 slug=test-strategy
fact: Blend modes with isolation, and an inline box of blocks painting no decorations of its own, are pinned by pixel frame tests in both layout modes.
@task_fact n=3 slug=ci-integration
fact: A CI job cancelled for runner capacity with no step run reads red; it is re-run and the latest attempt's verdict is the one read.
@task_fact n=4 slug=unit-test-strategy
fact: A red-before-green control of a CI scripts change cannot run through the fast leg in the control tool's clone, whose socket path passes the 94-byte state-dir bound; it takes a narrower entry or a hand control.
@task_item n=1 part=untouched
Session lifecycle, session wire, driver session and session host contracts: no protocol, socket, verb or error changed.
@task_item n=2 part=untouched
escher-session command-line, CLI and MCP role contracts and exit codes: no command, output or exit code changed.
@task_item n=3 part=untouched
agent-run and cold-agent verbs, events, exit codes, selections and verdict: no agent-run, status or log-shape change; run all and run stand counts are held by no row.
@task_item n=4 part=untouched
Refusal causes, lint defects, snapshot and handler coverage rows: none changed.
@task_item n=5 part=untouched
Virtual clock, advance, timer ticks and mocking discipline rows: none changed.
@task_item n=6 part=untouched
Coverage floors, mutation baseline, complexity ceiling and the stand-repeat three rounds with zero-failure budget: floors unmoved and stand-repeat 1248 unmoved.
@task_item n=7 part=untouched
Telemetry sink and log-content rows: two feature-gated warn lines moved, none new; sink capture checks passed unchanged.
@task_item n=8 part=untouched
MSRV, locked cargo legs and the shared build cache: lockfile and manifests byte-identical, MSRV leg green.
@task_item n=9 part=untouched
Bounded and retried package installs and apt-install exit codes: unchanged.
