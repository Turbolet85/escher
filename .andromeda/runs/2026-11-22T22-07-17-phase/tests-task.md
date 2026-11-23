@task_prose part=chunk
Takes upstream/main at 165d698c3 into the build branch as a real two-parent merge (never a rebase, squash or cherry-pick series): ten upstream commits, 21 files, no manifest or lock change.
What it brings: display: contents layout (block test through contents, pseudo-elements, inline-only wrapping), float keyword sizes and logical float/clear sides in the style-to-layout conversion, a textarea laid out once its width is known, the IME preedit underline paint, DOMTokenList.replace in the vibey-script runtime, upstream's WPT tooling and pin, and two new blitz-tests files.
Every left-out commit, the Dioxus 0.7.3 hold and every boundary widening the merge carries are written as operator forks into one evidence file, none answered by a default; the standing left-out set is re-read against the new upstream tip.
Re-runs every escher proof after the merge (fast and doc legs, stand checks in both layout modes, incremental oracle, a11y leg, dependency audit, embolden witness, screenshot and cross-OS readings, fork CI), re-reads the merge-surface residuals and the sink allowlist, and measures any lock move with a locked workspace build.
Re-points the RUSTSEC-2026-0192 advisory ignore's reason to name its residual row.
@task_file n=1 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom layout tree construction — the display: contents handling and inline-only wrapping upstream changes
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: blitz-dom layout damage — what incremental relayout marks dirty
@task_file n=3 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: blitz-dom inline layout build — the text of a textarea laid out once its width is known
@task_file n=4 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline layout pass — the textarea and inline text layout upstream changes
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=merge
what: blitz-dom node — the block-or-contains-block test read through display: contents
@task_file n=6 path=packages/blitz-dom/src/traversal.rs mark=merge
what: blitz-dom tree traversal helpers upstream trims
@task_file n=7 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting — the IME preedit underline
@task_file n=8 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: blitz-paint text decorations — the underline drawn under preedit text
@task_file n=9 path=packages/stylo_taffy/src/convert.rs mark=merge
what: stylo_taffy style-to-layout conversion — float keyword sizes and logical float/clear sides
@task_file n=10 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: stylo_taffy style wrapper the layout engine reads
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: blitz-vibey-script runtime DOM interfaces — DOMTokenList.replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: blitz-vibey-script DOM integration test — upstream's DOMTokenList.replace test
@task_file n=13 path=tests/blitz-tests/tests/all.rs mark=merge
what: blitz-tests aggregate test entry — upstream's new mod line for its new test files
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: upstream's new blitz-tests test file for display: contents layout
@task_file n=15 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: upstream's new blitz-tests test file for the IME preedit underline paint
@task_file n=16 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT diff-to-PR CI script
@task_file n=17 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: unittest of upstream's WPT diff-to-PR CI script
@task_file n=18 path=.github/workflows/wpt.yml mark=merge
what: upstream's WPT workflow — Pages artifact named per run attempt, an upstream-only workflow guarded by github.repository
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit pin
@task_file n=20 path=AGENTS.md mark=merge
what: upstream's agent guide — excluded from ours by the standing left-out set
@task_file n=21 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent guide for parley_code_editor — a package held out of our workspace by the standing left-out set
@task_file n=22 path=deny.toml mark=edits
what: the dependency audit's advisory and license policy — edits: the RUSTSEC-2026-0192 ignore entry's reason
@task_file n=23 path=.andromeda/cache/chunks/2026-11-22-upstream-sync-ahead-of-the-canvas/evidence/founder-forks.md mark=edits
what: the operator's forks list — every boundary widening, the Dioxus hold and every left-out commit, each with what crosses, the rule it meets, options with measured cost and dependences
@task_file n=24 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests targets check — one file and one target per behaviour
@task_file n=25 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script — the fast and doc legs re-run after the merge
@task_item n=1 part=needs
The sync is a two-parent merge whose parents are this branch's tip and upstream's 165d698c3; conflicts are measured on a fresh trial worktree under the build directory with rerere disabled.
@task_item n=2 part=needs
A left-out commit stays an ancestor and its change is undone in the merge's tree, named with its reason; the standing left-out set (parley_code_editor out of the workspace, upstream's agent guide excluded, Dioxus held) is re-read against the new tip, and a reason that no longer holds puts the commit back in.
@task_item n=3 part=needs
Dioxus stays held at 0.7.3 against upstream's 0.8 alpha pin unless the operator's fork rules otherwise: the buffered render on the Dioxus 0.7 bridge, upstream's buffered-render test kept as written, upstream's mounted-flush test left out and the keyboard_compat coverage exclusion are re-read at this sync.
@task_item n=4 part=needs
Every boundary widening, the Dioxus hold and every left-out commit go into one operator forks file, each with what crosses, the rule of ours it meets, options with their measured cost and dependences on the others, checked against the escalate patterns; none is answered by a default, and technical forks reach the operator as dialogs.
@task_item n=5 part=needs
Any lock move is measured by a locked workspace build beside the dependency audit, never by the audit alone; cargo legs run locked.
@task_item n=6 part=needs
Coupled dependency pins are taken as one set or held, a held pin named as an undo with its reason; no git dependency save Taffy's pinned rev, no bump beyond upstream's own set, MSRV stays 1.91 unless a taken pin requires more, then named.
@task_item n=7 part=needs
Taken layout behaviour — display: contents in box construction and pseudo-elements, the block test through contents, float keyword sizes and logical float/clear sides, a textarea's text laid out once its width is known, the damage marks — must hold in both layout modes and under the incremental/non-incremental differential oracle.
@task_item n=8 part=needs
Taken paint behaviour — the IME preedit underline — reaches pixels: screenshot and cross-OS readings that move are re-measured and recorded, never loosened; the embolden witness re-read.
@task_item n=9 part=needs
DOMTokenList.replace joins the vibey-script runtime with upstream's DOM test.
@task_item n=10 part=needs
Upstream's new display_contents and ime_preedit_underline test files and the aggregate entry's new mod line run under this fork's blitz-tests rule — one file and one target per behaviour — held by the blitz-tests targets check and the blitz-tests manifest's difference from upstream.
@task_item n=11 part=needs
Upstream's WPT tooling (the diff-to-PR script and its unittest, the WPT workflow's per-attempt Pages artifact, the WPT pin) is taken: the workflow keeps every action pinned to a 40-hex SHA and stays guarded by github.repository, ci.yml keeps contents: read, and WPT stays report-only.
@task_item n=12 part=needs
Conflicts keep both sides' intent: the changed-set contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the into-view readings, stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, the buffered render, and the escher crates and seven_guis workspace members; no workaround bought only to keep upstream code untouched.
@task_item n=13 part=needs
Every escher proof passes after the merge: the fast and doc legs (clippy over the workspace and upstream blitz-dom included), the stand checks in both layout modes with stand-repeat's zero-failure budget, the a11y leg, the dependency audit, and fork CI green on the pushed commit with every job settled.
@task_item n=14 part=needs
The merge-surface residuals (the left-out table, the two Dioxus 0.8 move rows, and res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab, res-ipcah32k) are re-read against the merge; one a taken move resolves is re-measured, never assumed.
@task_item n=15 part=needs
The sink allowlist is re-checked: the merged delta read for every new tracing or log call site, target, print and env read; the capture checks stay at 0 needles and third-party targets stay dropped.
@task_item n=16 part=needs
Every advisory ignore in the dependency audit names its residual row: the RUSTSEC-2026-0192 reason re-pointed to res-y4gizreq instead of stating the wait alone.
@task_item n=17 part=needs
The last wrap commit's CI verdict is unread and never folded as green; it is re-read at P3 and before this chunk's push, and new upstream tests moving a tally are attributed by test name.
@task_item n=18 part=needs
No new crate, port, env var or listener of ours.
@task_standing n=1
convention: each workflow edit keeps every action pinned to a commit SHA, read-only permissions and no secrets, and an upstream-only publishing job keeps its repository guard
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: each integration test file is its own test target, one behaviour per file, shared modules no target
home: test-scope-summary · test-strategy
@task_standing n=3
convention: each layout assertion runs in both layout modes
home: integration-test-strategy
@task_standing n=4
convention: each Python CI script test runs under unittest
home: 3-test-harness-contract
@task_standing n=5
convention: test data stated in code, nothing seeded
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: each CI tally move is attributed by test name, job logs read with ANSI stripped
home: ci-integration
@task_item n=1 part=not_needed
The session host, session wire, socket protocol and session state directory rows: the merge changes nothing of escher-session's lifecycle or wire.
@task_item n=2 part=not_needed
The MCP server and escher-session command-line contracts (tools/list, help, exit codes, stdout, stderr lines): no command-line or MCP surface is touched.
@task_item n=3 part=not_needed
The cold-agent pipe rows (events, verdict, exit codes, isolation, prompt): no cold-agent run is owed.
@task_item n=4 part=not_needed
The agent-run verbs, selections, events and exit codes: the agent-run contract is not changed.
@task_item n=5 part=not_needed
The driver refusal-cause set, its printed form, driver argument kinds and lint defects and findings: no driver verb or refusal is added or changed.
@task_item n=6 part=not_needed
Handler coverage rows (bubbling dispatch, canvas handlers, unreached screens, the coverage line): handler coverage is not touched.
@task_item n=7 part=not_needed
Harness contrast and the WCAG ratio rows: no contrast reading is changed.
@task_item n=8 part=not_needed
Session recording and committed recorded-check rows (record fields, check files, byte-equal recordings, recording directory): no check is recorded or re-recorded.
@task_item n=9 part=not_needed
Mutation testing rows (baseline, cargo-mutants filter, dev-host runs): the chunk owes no mutation run over upstream's taken code.
@task_item n=10 part=not_needed
Harness construction, input helper and inspection helper contracts: no harness API is added or changed.
