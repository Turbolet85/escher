@task_prose part=chunk
Syncs upstream/main at 165d698c3 into build/escher-0.4.0 as one real two-parent merge (merge base 547d7514b, 10 upstream commits, 21 files, no manifest or lock change by the diff stat) — never a rebase, a squash or a cherry-pick series.
Takes upstream's display: contents, float keyword size, IME preedit underline, textarea text layout, DOMTokenList.replace, WPT tooling and pin, and CI Pages artifact changes, with upstream's new blitz-tests files.
Re-reads the standing left-out set (parley_code_editor out of the workspace, upstream's AGENTS.md excluded, Dioxus held at 0.7.3) against the new pin; every left-out commit, the Dioxus hold and every boundary widening goes to the operator as a fork with its measured options, none answered by a default.
Re-points the RUSTSEC-2026-0192 advisory ignore's reason to its residual row, re-reads the merge-surface residuals, re-checks the sink allowlist on the merged delta, and re-measures every escher proof after the merge.
@task_file n=1 path=AGENTS.md mark=merge
what: upstream's repository agent-instructions file, excluded from the take by the standing left-out set
@task_file n=2 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent-instructions file for the code-editor package, which stands out of this fork's workspace
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the layout tree construction — display: contents pseudo-elements, inline-only wrapping and block containment read through display: contents
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the layout damage tracking that upstream's display: contents and textarea changes reach
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the inline layout builder — inline-only wrapping under display: contents and textarea text
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the inline layout pass — a textarea's text laid out once its width is known
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the DOM node model — whether a node is or contains a block through display: contents
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the DOM tree traversal that display: contents children are walked through
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: the text painter — the IME preedit text painted with its underline
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the text decoration painter — the IME preedit underline
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's bootstrap DOM interfaces — DOMTokenList.replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM interface tests — DOMTokenList.replace
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion — float keyword sizes and logical float and clear sides
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper the float and clear conversions are read through
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test target — upstream's new mod line
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: upstream's new display: contents test file, under the one-file-one-target-per-behaviour blitz-tests rule
@task_file n=17 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: upstream's new IME preedit underline test file, under the one-file-one-target-per-behaviour blitz-tests rule
@task_file n=18 path=.github/workflows/wpt.yml mark=merge
what: upstream's WPT CI workflow, guarded by github.repository, its actions pinned to 40-hex SHAs
@task_file n=19 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT tooling script that turns a WPT diff into a pull request
@task_file n=20 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of upstream's WPT diff-to-PR tooling script
@task_file n=21 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision upstream's tooling reads
@task_file n=22 path=deny.toml mark=edits
what: the dependency audit's advisory ignore list, each ignore naming its residual row — edits: the RUSTSEC-2026-0192 ignore's reason
@task_file n=23 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests targets pin — one file and one target per behaviour
@task_file n=24 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner's fast and doc legs, re-run after the merge
@task_item n=1 part=needs
The sync itself: one two-parent merge whose parents are this branch's tip and upstream's 165d698c3; a left-out commit stays an ancestor, so leaving it out means undoing its change in the merge's tree, each named with its reason; a standing reason that no longer holds puts the commit back in.
@task_item n=2 part=needs
Taken layout behaviour: display: contents pseudo-elements, inline-only wrapping, and block containment read through display: contents — a change the merge brings to how the documents this fork renders are laid out.
@task_item n=3 part=needs
Taken style behaviour: float keyword sizes and logical float and clear sides converted to layout.
@task_item n=4 part=needs
Taken paint and text behaviour: the IME preedit underline painted, and a textarea's text laid out once its width is known.
@task_item n=5 part=needs
Taken script behaviour: DOMTokenList.replace in the script runtime's DOM interfaces.
@task_item n=6 part=needs
Taken CI and tooling: the WPT tooling and pin, and the CI Pages artifact named per run attempt; every action stays pinned to a 40-hex SHA, ci.yml keeps contents: read, and upstream's workflow files stay guarded by github.repository.
@task_item n=7 part=needs
Upstream's new test files and the aggregate target's new mod line run under this fork's blitz-tests rule: one file and one target per behaviour, held by the targets pin.
@task_item n=8 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, the buffered render, and the escher crates and seven_guis members of the workspace; no workaround bought only to keep upstream code untouched.
@task_item n=9 part=needs
Lock moves: any lock move the merge or a fork's option makes is measured by a --locked workspace build with the dependency audit beside it, never by the audit alone.
@task_item n=10 part=needs
Dependency boundary: the workspace takes no git dependency save Taffy's pinned rev; upstream's pins are carried as one coupled set or held, a held pin named as an undo with its reason; no bump beyond upstream's own set; MSRV stays 1.91 unless a taken pin requires more, then named.
@task_item n=11 part=needs
The Dioxus hold: Dioxus stays at 0.7.3 against upstream's 0.8.0-alpha.3 pin as an operator fork; whether 0.8 is released and the hold's cost both ways are measured, and its residuals and the keyboard_compat coverage exclusion re-read.
@task_item n=12 part=needs
The advisory ignore for RUSTSEC-2026-0192 names its owning residual row, res-y4gizreq, as every ignore must name its residual row.
@task_item n=13 part=needs
The forks file: every boundary widening the merge carries, the Dioxus hold and every left-out commit, the standing ones re-read included, collected in one file, each with what crosses, the rule of ours it meets, its options and their measured cost, and its dependences on the others, checked against the escalate patterns; none answered by a default; technical forks go to the operator as dialogs.
@task_item n=14 part=needs
Every escher proof passes after the merge: the fast and doc CI legs, the stand checks in both layout modes, the incremental and non-incremental oracle, the a11y leg, the dependency audit, the embolden witness, and fork CI green on the pushed commit with every job settled; the last wrap's unread CI verdict is re-read before the push.
@task_item n=15 part=needs
Screenshot and cross-OS readings of the stand: a reading the taken layout and paint changes move is re-measured and recorded, never loosened.
@task_item n=16 part=needs
Sink allowlist: the merged delta read for every new tracing or log call site, target, print and env read; the standing capture checks stay at 0 needles.
@task_item n=17 part=needs
Merge-surface residuals re-read against the merge — the left-out table, the Dioxus 0.8 move's residuals and the others the scope names; one a taken move resolves is re-measured, never assumed.
@task_item n=18 part=needs
No new crate, port, env var or listener of ours enters with the merge.
@task_item n=1 part=not_needed
The escher-session driver's command-line contracts — its command form, exit codes, help, graph, lint, layout, coverage role, actions, recording and refusals: no upstream change reaches the driver binary and the chunk adds no verb, answer or exit code.
@task_item n=2 part=not_needed
The escher-session MCP server contracts — stdio transport, tools list, protocol revisions, result bound, screenshot result and end: the merge brings no change to the driver's server.
@task_item n=3 part=not_needed
The agent-run and cold-agent pipe contracts and the cold-agent decisions (prompt names no id, pipe builds nothing, drives through the MCP server): no script of those pipes is touched.
@task_item n=4 part=not_needed
The multi_doc and paint_bench command-line contracts — arguments, environment knobs, stdout, backend: the chunk changes no example's command line.
@task_item n=5 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the upstream delta does not reach the shell provider.
@task_item n=6 part=not_needed
Stand page-building decisions that the merge does not reach — the stand accent colour, the fixed document skeleton with its main mount, the order page controls built to cause, session roles sharing the stand boot, a held instance reading as a fresh boot: the chunk builds nothing on the stand.
@task_item n=7 part=not_needed
The absence of arbitrary index.html templates for a document: the chunk adds no document template.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces and the decisions log: no row stands there and the chunk reaches no mobile or web layout.
