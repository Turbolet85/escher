@task_prose part=session
The session synced the fork with upstream Blitz main by a two-parent merge of ten upstream commits, the fork's last taken upstream commit moving to 165d698c3, with one upstream commit left out whole.
It took the founder-ratified upstream behaviour changes E1 to E6 as written, with no upstream hunk edited: DOMTokenList replace for page scripts, float and clear logical sides resolved by direction, display contents fixes to block containment and box construction, the IME composition underline, and the textarea's deferred text layout.
It resolved one conflict in the upstream-only WPT workflow, keeping the SHA-pinned Pages deploy action and both repository guards, and deleted upstream's arriving AGENTS.md files again so none enters the tree.
It hand-edited the dependency audit config so the RUSTSEC-2026-0192 ignore names the residual row that owns its removal; no dependency, crate, workspace member, port, socket or env var changed, and Dioxus stays held at 0.7.
It re-read the Dioxus 0.8 hold at this sync and kept it, carrying the next re-read to the next upstream sync or the first sync after a Dioxus 0.8 release; upstream's iter_children macro is retired, no caller of escher's.
CI went green 17 of 17 and blitz-tests gained one integration test target for the IME preedit underline.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: the upstream-only WPT workflow, its Pages deploy step and its repository guards
@task_file n=2 path=deny.toml
what: the cargo-deny dependency and licence audit config, its advisory ignores and their reasons
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom box construction, child flow classification and anonymous blocks, including display contents and the textarea
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: blitz-dom damage classes and the layout caches cleared by them
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: blitz-dom inline layout construction
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline layout, the placing of floats beside inline content
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the blitz-dom Node, its structure and its block containment test
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: blitz-dom tree traversal helpers over children and pseudo-elements
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting, including text inputs
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: blitz-paint explicit text-decoration geometry, kept out of Parley
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the blitz-vibey-script JS bootstrap, the DOM interfaces page scripts see
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the blitz-vibey-script DOM tests for page-script APIs
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: stylo_taffy's conversion of Stylo computed styles to Taffy styles, its exported float and clear converters
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: stylo_taffy's style wrapper that Taffy reads, the caller of the float and clear converters
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test target and its module list
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests display contents integration test
@task_file n=17 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new blitz-tests integration test target, the IME composition underline painted until commit
@task_file n=18 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision the WPT runner tests against
@task_file n=19 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: the upstream CI script that writes the WPT diff into the PR body and step summary
@task_file n=20 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the unittest suite of the WPT diff to PR script
@task_item n=1 part=touches
contract/layout/stylo-taffy-exports: stylo_taffy's convert float and convert clear now take a second ltr bool argument, the logical sides resolved by direction; the exported signatures moved.
@task_item n=2 part=touches
decision/dioxus-0-8-move-held: re-read at this upstream sync and held at Dioxus 0.7; its next re-read named as the next upstream sync or the first sync after a Dioxus 0.8 release.
@task_item n=3 part=touches
decision/keyboard-compat-coverage-exclusion: the coverage exclusion of upstream keyboard_compat rides the Dioxus hold and is carried to the same next re-read.
@task_item n=4 part=touches
decision/upstream-agents-md-excluded: applied again; upstream's AGENTS.md and the parley_code_editor AGENTS.md arrived with this take and were removed, the conflict resolved by deletion.
@task_item n=5 part=touches
decision/upstream-behaviour-changes-ratified-per-take and decision/upstream-take-engine-changes: this take's six engine behaviour changes E1 to E6 ratified by the founder and merged as written.
@task_item n=6 part=touches
decision/upstream-sync-by-two-parent-merge and decision/upstream-sync-merge-base: this sync was a two-parent merge with one upstream commit left out whole; the left-out set reads 15, a count no row holds.
@task_item n=7 part=touches
decision/fork-additions-in-own-functions and decision/direct-engine-changes-flagged-upstreamable: no upstream hunk was edited in the take; escher's additions stay out of upstream bodies.
@task_item n=8 part=touches
decision/upstream-only-workflows, decision/upstream-ci-resources-upstream-only and decision/actions-pinned-to-commit-sha: the WPT workflow conflict kept both repository guards and the SHA-pinned Pages deploy action, taking upstream's per-run-attempt artifact name.
@task_item n=9 part=touches
decision/cargo-deny-advisory-gate, decision/dependency-and-licence-audit and decision/cargo-deny-audit: an advisory ignore's reason now names the residual row that drops it; the audit leg still green with the NCSA licence-not-encountered warning.
@task_item n=10 part=touches
decision/deferred-inline-layout: the textarea's text layout is now deferred like other inline layout construction (E6).
@task_item n=11 part=touches
decision/damage-scoped-cache-clearing and decision/damage-bits-drive-relayout: the damage path changed for the textarea's deferred text layout.
@task_item n=12 part=touches
decision/pseudo-elements-vote-in-flow-classification and decision/anonymous-blocks-in-layout-children-only: box construction now classifies children by a child flow and treats a display contents element holding only inlines as inline (E4).
@task_item n=13 part=touches
decision/text-decoration-outside-parley and decision/explicit-text-decoration-geometry: the composition underline is a new explicit decoration painted by blitz-paint, outside Parley (E5).
@task_item n=14 part=touches
decision/ime-event-copied, decision/ime-event-copied-from-winit and decision/ime-enabled-at-editor-construction: the IME preedit text a focused input receives is now painted underlined until committed.
@task_item n=15 part=touches
decision/blitz-tests-one-target-per-file, decision/blitz-tests-target-per-file and decision/blitz-tests-single-test-target: a new blitz-tests file became its own target by cargo's discovery with no manifest edit.
@task_item n=16 part=touches
contract/ci/wpt-diff-to-pr, contract/wpt-diff-to-pr-output/pr-body and contract/wpt-diff-to-pr-output/step-summary: the WPT diff script now shows results beside a revision mismatch, with or without a diff.
@task_item n=17 part=touches
contract/wpt-ci-artifacts/wpt-diff and contract/wpt-ci-artifacts/wpt-report-json-zst: upstream's WPT workflow now names its Pages artifact per run attempt.
@task_item n=18 part=touches
decision/wpt-mirrors-upstream-tooling and decision/wpt-default-suites: the pinned WPT revision moved with the take.
@task_item n=19 part=touches
contract/script/bootstrap and decision/shared-element-prototype: the JS bootstrap gives page scripts DOMTokenList replace, so classList.replace exists (E1).
@task_item n=20 part=touches
contract/answer-json/layout-placing: indirect, through the float converters; a float placed by a logical inline-start or inline-end side now lands on the physical side its direction gives.
@task_item n=21 part=touches
decision/layout-modes-identical and decision/one-incremental-pipeline: indirect, the display contents and textarea changes are held identical in both layout modes by the incremental oracle and display contents tests.
@task_fact n=1 slug=established-decisions
fact: While an IME composes, the preedit text in a text input is painted with a composition underline in the text colour, in every document, until the composition is committed.
@task_fact n=2 slug=established-decisions
fact: Logical float and clear values, inline-start and inline-end, resolve to left or right by the box's direction, left-to-right or right-to-left.
@task_fact n=3 slug=established-decisions
fact: Block containment looks through display contents, so an element whose display contents child holds a block counts as containing a block, and a display contents element holding only inlines does not break its parent's inline flow.
@task_fact n=4 slug=standard-contracts
fact: Page scripts get DOMTokenList replace, so element.classList.replace swaps one class token for another as the DOM standard says.
@task_item n=1 part=untouched
Driver, session, MCP role, session socket and kept-check contracts and decisions: no verb, argument, wire form, refusal cause or answer shape moved.
@task_item n=2 part=untouched
Occupied resources and names: no crate, binary, server, port, socket, listener or env var was added, taken or freed.
@task_item n=3 part=untouched
Stack rows on Dioxus, winit, Taffy, Parley, Stylo, Boa and the other crates: Cargo manifests and the lockfile are unchanged and no version moved.
@task_item n=4 part=untouched
Accessibility tree, snapshot, stable element id, UI graph and lint contracts: none of their readers or written forms changed.
@task_item n=5 part=untouched
Select list, range input, focus, hit testing and scrolling decisions: the take changed none of them.
@task_item n=6 part=untouched
Telemetry, tracing and scrubbing rows: no log line, span or target changed, and no new sink site appeared.
@task_item n=7 part=untouched
escher's own CI legs, coverage floors, stand-repeat budget and its named stand targets, and the cold-agent and agent-run tools: the new test target is no stand target and no leg changed.
@task_item n=8 part=untouched
Browser app, history, persistence and networking rows: blitz-net, the browser and its storage were not reached.
@task_item n=9 part=untouched
Project intent rows on the capability statement and the root README: no capability of the driver was added or removed.
