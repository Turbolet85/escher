@task_prose part=chunk
Syncs upstream into escher's build branch as a real two-parent merge of upstream main at 165d698c3 (10 commits past the last sync's base 547d7514b, 21 files, no manifest or lock change), never a rebase, squash or cherry-pick series.
Names every commit left out with its reason, re-reading the standing left-out set (parley_code_editor out of the workspace, upstream AGENTS.md excluded, Dioxus held at 0.7.3), and writes every boundary widening, the Dioxus hold and every left-out commit as an operator fork into one evidence file, none answered by a default.
Re-points the dependency audit's RUSTSEC-2026-0192 ignore reason to name its residual row res-y4gizreq.
Re-measures every escher proof after the merge (ci-leg fast and doc, stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg, the audit, the embolden witness, screenshot and cross-OS readings, fork CI green) and re-reads the merge-surface residuals and the telemetry sink allowlist against the merged delta.
@task_file n=1 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom layout box construction — the display: contents block test, display: contents pseudo-elements and inline-only wrapping, anonymous blocks in layout children
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: blitz-dom damage propagation and damage-scoped layout cache clearing
@task_file n=3 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: blitz-dom deferred inline layout construction — a textarea's text laid out once its width is known
@task_file n=4 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline layout of text and inline boxes through Parley
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=merge
what: blitz-dom Node — structure, its recorded facts and its layout data readers
@task_file n=6 path=packages/blitz-dom/src/traversal.rs mark=merge
what: blitz-dom tree traversal helpers over the layout children
@task_file n=7 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting from the container's style, including an editing text input's IME preedit
@task_file n=8 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: blitz-paint explicit text-decoration geometry, gaining the IME preedit underline
@task_file n=9 path=packages/stylo_taffy/src/convert.rs mark=merge
what: stylo_taffy conversion of Stylo computed styles to Taffy styles — float keyword sizes and logical float and clear sides
@task_file n=10 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: stylo_taffy style wrapper Taffy reads through
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: blitz-vibey-script JS bootstrap DOM interfaces — DOMTokenList gaining replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: blitz-vibey-script DOM interface tests
@task_file n=13 path=tests/blitz-tests/tests/all.rs mark=merge
what: blitz-tests aggregate test target and its mod list, gaining upstream's new mod line
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: upstream's new blitz-tests integration test file for display: contents layout
@task_file n=15 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: upstream's new blitz-tests integration test file for the painted IME preedit underline
@task_file n=16 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: the WPT diff-to-PR CI helper script — its command line and its PR body and step summary output
@task_file n=17 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the unittest tests of the WPT diff-to-PR CI helper script
@task_file n=18 path=.github/workflows/wpt.yml mark=merge
what: upstream's WPT CI workflow, guarded to the upstream repository, naming its Pages artifact per run attempt
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the WPT tooling pin, the WPT checkout commit the runner mirrors
@task_file n=20 path=AGENTS.md mark=merge
what: upstream's root agent guidance file, kept out of the tree by the standing left-out ruling
@task_file n=21 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent guidance file for parley_code_editor, a package kept out of the workspace by the standing left-out ruling
@task_file n=22 path=deny.toml mark=edits
what: the workspace dependency and licence audit configuration, its advisory ignores by id — edits: the RUSTSEC-2026-0192 ignore entry's reason
@task_file n=23 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the blitz-tests rule of one file and one test target per behaviour
@task_file n=24 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner — its fast, doc, a11y, audit and embolden legs re-run after the merge
@task_item n=1 part=needs
The sync is a real two-parent merge whose parents are the build branch's tip and the upstream commit taken; never a rebase, a squash or a cherry-pick series, and the merge base is the last sync's upstream commit.
@task_item n=2 part=needs
Each commit left out is named with its reason; in a two-parent merge a left-out commit stays an ancestor, so leaving it out means undoing its change in the merge's tree, and a reason that no longer holds puts the commit back in.
@task_item n=3 part=needs
The standing left-out set is re-read against the upstream commit taken: parley_code_editor kept out of the workspace, upstream's AGENTS.md files kept out of the tree, and the Dioxus 0.8 move held at 0.7.3 while upstream pins 0.8.0-alpha.3.
@task_item n=4 part=needs
The Dioxus hold is re-read with its residuals res-xpkmxnvl and res-xchmbfjr and the keyboard_compat coverage exclusion; whether Dioxus 0.8 is released and the hold's cost both ways are measured, and the hold is an operator fork.
@task_item n=5 part=needs
Every boundary widening the merge brings, the Dioxus hold and every left-out commit go into one founder-forks evidence file, each with what crosses, the rule of ours it meets, its options with their measured cost and its dependences on the others, checked against the escalate patterns; none is answered by a default, and technical forks go to the operator as dialogs.
@task_item n=6 part=needs
The take changes layout behaviour: display: contents is seen through when testing whether a box is or contains a block, display: contents pseudo-elements are generated and wrapping is inline-only; it is ratified as an upstream engine behaviour change of this take, and both layout modes still give identical layouts.
@task_item n=7 part=needs
The take changes style-to-layout conversion: float keyword sizes and logical float and clear sides are converted for Taffy, ratified as an upstream behaviour change of this take.
@task_item n=8 part=needs
The take changes inline layout of a textarea: its text is laid out once its width is known, a change to deferred inline construction ratified per take.
@task_item n=9 part=needs
The take adds painting of the IME preedit underline in text decoration, beside escher's explicit text-decoration geometry and the embolden witness, ratified per take.
@task_item n=10 part=needs
The take adds DOMTokenList.replace to the JS bootstrap's DOM interfaces in the script runtime, ratified per take.
@task_item n=11 part=needs
The take changes upstream's WPT tooling — the diff-to-PR helper and its tests, the WPT workflow's Pages artifact named per run attempt, and the WPT commit pin — tooling the WPT runner mirrors from upstream and that stays upstream-only.
@task_item n=12 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, the buffered render, and the escher crates and seven_guis workspace members; no workaround is bought only to keep upstream code untouched.
@task_item n=13 part=needs
Where a resolution keeps an escher addition inside a merged upstream body, the fork's additions stay in functions of escher's own.
@task_item n=14 part=needs
A lock move the merge or a fork's option makes is measured by a locked workspace build beside the dependency audit, never by the audit alone; cargo legs run locked.
@task_item n=15 part=needs
A take carries upstream's coupled dependency pins as one set or holds the set; holding a pin upstream moved is an undo named with its reason; no dependency is bumped beyond upstream's own set.
@task_item n=16 part=needs
The workspace takes no git dependency save Taffy's pinned revision.
@task_item n=17 part=needs
CI workflow edits keep every action pinned to a 40-hex commit SHA and the CI workflow's read-only contents permission; upstream's workflow files stay guarded by the repository check so upstream CI resources are reached only from upstream; the CI contract tests pin this.
@task_item n=18 part=needs
MSRV stays 1.91 unless a taken pin requires more, then named.
@task_item n=19 part=needs
The chunk adds no crate, port, environment variable or listener of escher's own.
@task_item n=20 part=needs
Upstream's new test files run under the blitz-tests rule of one file and one target per behaviour, the new mod line in the aggregate target included, held by the targets check.
@task_item n=21 part=needs
Every escher proof passes after the merge: the fast and doc legs, the stand checks in both layout modes, the incremental and non-incremental oracle, the a11y leg, the dependency audit, the embolden witness, and the screenshot and cross-OS readings, a moved reading re-measured and recorded, never loosened; fork CI green on the pushed commit with every job settled.
@task_item n=22 part=needs
The merged delta is read for every new tracing or log call site, target, print and environment read against the telemetry sink allowlist; engine tracing stays gated per call site and the standing capture checks stay at zero needles.
@task_item n=23 part=needs
Every advisory ignore in the dependency audit names its residual row: the RUSTSEC-2026-0192 reason is re-pointed to name res-y4gizreq, not the wait alone.
@task_item n=24 part=needs
The merge-surface residuals res-7qgqygtj, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab and res-ipcah32k are re-read against the merge; one a taken move resolves is re-measured, never assumed.
@task_item n=25 part=needs
The last wrap commit's CI verdict, still in progress, is never folded as green and is re-read before this chunk's push.
@task_standing n=1
convention: Each integration test file in blitz-tests holds one behaviour and is its own test target, pinned by the targets check.
home: conventions · standard-contracts
@task_standing n=2
convention: Each CI workflow change keeps actions pinned to commit SHAs, read-only permissions and upstream-only guards on upstream workflows.
home: stack-and-technologies · infrastructure-patterns
@task_standing n=3
convention: Each CI helper script test uses unittest on the Python stdlib.
home: conventions · stack-and-technologies
@task_item n=1 part=not_needed
The driver's verbs, arguments, refusal causes and faults, answer JSON, session wire and socket, MCP server and kept-check file contracts: the merge reaches no driver or session code.
@task_item n=2 part=not_needed
The cold-agent and agent-run scripts' events, exit codes, state and verdict: no cold-agent run is part of this sync.
@task_item n=3 part=not_needed
The browser app's about URLs, history store and migrations, persistence, toolbar and URL suggestions: no upstream commit taken touches the browser app.
@task_item n=4 part=not_needed
Absences of unsupported CSS features such as container queries, anchor positioning, subgrid, masonry, shadow DOM and 3D transforms: the take adds none of them.
@task_item n=5 part=not_needed
The canvas entries' work and every later entry, and sending escher's changes upstream: out of this chunk's scope.
@task_item n=6 part=not_needed
Stack choices for crates the merge leaves unpinned, such as nucleo, rfd, rdme's engines, mimalloc and png: no dependency is bumped beyond upstream's own set.
@task_item n=7 part=not_needed
Android, iOS and macOS entry points and shell platform quirks: no commit taken touches the shells.
@task_item n=8 part=not_needed
The package-install script, the skia archive check and the target-sweep script's arguments and exit codes: the sync changes none of these helpers.
@task_item n=9 part=not_needed
The UI graph, its registry and the seven_guis graph-signal hooks: the take reaches no Dioxus bridge or graph code.
@task_item n=10 part=not_needed
The lint verb's defect set and the layout reason contracts read by the layout verb: the take adds no lint or layout-reason reader.
