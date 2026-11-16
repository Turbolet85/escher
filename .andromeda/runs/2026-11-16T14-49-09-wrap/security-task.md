@task_prose part=session
The session took seven upstream blitz picks past #1372: reversed scroll origins measured against Taffy's ScrollableOverflow, hit testing of elements with no computed styles yet, and five Taffy bumps.
The engine dependency Taffy moves from git rev e90751fd to git rev a3997d55, version 0.15.0 and features unchanged; it stays the one git source in the lockfile and every leg builds locked.
For every document, remote pages included, a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) now scrolls to negative offsets; the programmatic scroll clamps to the signed range (min, max); ratified by the founder for this take.
blitz-dom's public API widens by one read-only accessor, Node::try_layout_data, which answers none before layout; a node with no computed styles reads as no scroll container, closing a select popup panic.
The hit test's clip stop now answers in the content-box frame (only an inline root's answer moves); the take resolves the textual conflict between upstream's hit_inner hunk and the non-invertible-transform answer, which stands.
No IPC method, socket, verb, argument, result field, node key, refusal cause, session error, exit, env var, log field or span was added; audit (advisories ok, licenses ok), msrv and CI 17/17 green.
@task_file n=1 path=Cargo.toml
what: the workspace root manifest, holding the engine dependency pins, among them the Taffy git rev — edited, the Taffy rev line alone
@task_file n=2 path=Cargo.lock
what: the lockfile that locked builds and the dependency audit read; the one git source — edited, the Taffy package's source line alone
@task_file n=3 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine DOM's layout debug printing — edited by the upstream picks
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, its touch-action pan axes reading the scroll size — edited
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, inline-box sizes and scrollable overflow — edited
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout pass over Taffy, its layout cache — edited, the cache read now takes a shared reference
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's replaced-element sizing (images, canvases), their min-content contributions — edited
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: the engine's writing-mode geometry, overflow mirroring and its scroll-container test, false for a node with no computed styles — edited
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's DOM mutator and its unit tests, among them hit testing before the first style resolve and the clipping inline root border hit — edited
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine DOM node, its hit test (clip stop, non-invertible transform answer, own-scroll offset), its layout data accessors and its CSSOM scroll width and height — edited, the hit test's textual conflict with upstream resolved
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's scrollbar geometry, the thumb over the scroll range — edited, now the signed range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scrolling, the programmatic scroll clamp, scroll into view and the scroll animation — edited, the scroll state now a triple (current, min, max)
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests integration tests of display contents — edited
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: the blitz-tests integration tests of inline-box scrollable overflow — edited, six reversed-origin tests added
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the blitz-tests integration test of pre-element overflow scrolling — edited
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test checks, its scroll size checks now reading the node's CSSOM size — edited
@task_item n=1 part=touches
decision/engine-dependency-pins: the Taffy pin it holds moves from git rev e90751fd to a3997d55 (upstream DioxusLabs/taffy), version 0.15.0 and features unchanged — the row is stale.
@task_item n=2 part=touches
decision/no-git-dependencies and decision/no-git-dependency: Taffy is still the one git source, now at rev a3997d55; any rev or exception those rows name moves with it.
@task_item n=3 part=touches
decision/upstream-sync-dependency-set: the upstream take carries five Taffy bumps whole, the dependency list otherwise unchanged.
@task_item n=4 part=touches
decision/locked-builds, decision/dependency-audit-ci-gate, decision/cargo-deny-audit-tool and decision/licence-gate: the lockfile's Taffy source line moved, built locked by every leg and audited green (advisories ok, licenses ok).
@task_item n=5 part=touches
decision/programmatic-scroll-clamped: the programmatic scroll now clamps to the signed range (min, max), so a reversed box scrolls to negative offsets; a missing node is still ignored.
@task_item n=6 part=touches
decision/select-list-scroll-own-offset: scroll into view now reads the (current, min, max) triple and skips a box whose min and max are both zero; the select list row's own-offset scroll is over that component.
@task_item n=7 part=touches
decision/non-invertible-transform-no-hit: upstream's hit test hunk conflicted textually with this answer; the take resolves it and the non-invertible transform still takes no hit.
@task_item n=8 part=touches
decision/visible-region-through-transforms and decision/visible-region-open-list-row: the hit test's content test now reads the overflow rect's four sides plus the client offset, and the clip stop answers content-box coordinates for an inline root; whether a hit happens and which node is unchanged.
@task_item n=9 part=touches
decision/skip-snapshot-of-unstyled-node and decision/select-row-layout-none-without-styles: the hit test now reaches elements with no computed styles yet, and a node with no computed styles reads as no scroll container.
@task_item n=10 part=touches
decision/stale-node-ids-never-panic and decision/no-panic-on-call-input: the select popup panic on an unstyled node under the workspace's features is closed by the scroll-container test answering false.
@task_item n=11 part=touches
decision/text-extent-read-only-none-when-unresolved: a sibling read-only accessor, Node::try_layout_data, now public, answers none where no layout data stands.
@task_item n=12 part=touches
decision/scrollbar-fade-on-document-clock: the scrollbar component changed — its thumb runs over the signed scroll range; the fade clock is untouched.
@task_item n=13 part=touches
decision/inline-box-heights-finite: inline layout and its scrollable overflow changed with the Taffy bumps; the finite-height rule is over that component.
@task_item n=14 part=touches
decision/untrusted-bytes-decoded-without-panic and decision/select-list-hostile-markup-finite: the reversed-origin behaviour change reaches every document, remote pages included.
@task_item n=15 part=touches
decision/session-layout-read-in-process: scroll width and height now answer the CSSOM scroll size, reversed origins included, to every in-process reader, a page's scrollWidth and scrollHeight among them.
@task_fact n=1 slug=input-validation
fact: A reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets on every document, remote pages included; scroll width and height answer the CSSOM scroll size, ratified by the founder for the upstream take past #1372.
@task_fact n=2 slug=input-validation
fact: blitz-dom's public API carries one read-only layout accessor, Node::try_layout_data, which answers none for a node with no layout data and writes nothing.
@task_fact n=3 slug=error-handling
fact: A node with no computed styles yet is hit-testable and reads as no scroll container, so geometry and hit reads on it never panic.
@task_item n=1 part=untouched
The driver session socket, the MCP server, their wire grammar, bounds, replies and protocol errors: no method, request, reply, socket or port was added or changed.
@task_item n=2 part=untouched
Driver verbs, argument kinds, refusal causes, refusal and session error JSON, exit codes and CLI flags: none added, removed or moved.
@task_item n=3 part=untouched
Logging and telemetry: the sink, its target allowlists, the command and tool_call span lines and their fields; no log site, span or field was added.
@task_item n=4 part=untouched
Secret management and CI: secrets, signing material, publish jobs, CI tokens and action SHA pins; no workflow or harness surface changed.
@task_item n=5 part=untouched
Data protection: history, cookies, the HTTP cache, kept and recorded checks, recording directories and their owner-only rules.
@task_item n=6 part=untouched
Outbound requests and fetchers: blitz-net, script and font fetch limits, schemes, per-host limits, and the CORS, CSP and size-limit absences.
@task_item n=7 part=untouched
Stable element ids, snapshot text, graph and actions answers, masked control values and the accessibility adapter crossing: no carrier or reading changed.
@task_item n=8 part=untouched
Authentication and authorization absences, and the threat model's crossing decisions for the MCP, actions, graph, caused and coverage answers.
