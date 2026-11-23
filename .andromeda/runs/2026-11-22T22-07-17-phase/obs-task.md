@task_prose part=chunk
Takes upstream/main at 165d698c3 into build/escher-0.4.0 as one real two-parent merge (never a rebase, squash or cherry-pick series): ten upstream commits over 21 files, no manifest and no lock change by the diff stat.
The upstream changes it brings: display: contents layout and pseudo-elements, float keyword sizes and logical float/clear sides, the IME preedit underline paint, a textarea's text laid out once its width is known, DOMTokenList.replace in the script runtime, WPT tooling and pin, and the CI Pages artifact named per run attempt.
Every left-out commit (the standing set re-read: upstream's AGENTS.md files excluded, parley_code_editor out of the workspace, Dioxus held at 0.7.3), every boundary widening and the Dioxus hold are written as operator forks into one evidence file, none answered by a default; it re-points the advisory ignore's reason in the dependency policy to its residual row.
It re-reads the merged delta for every new tracing/log call site, target, print and env read against the sink allowlist, keeps the capture checks at 0 needles, and re-runs every escher proof (fast and doc legs, stand checks, oracle, a11y leg, dependency audit, screenshot and cross-OS readings, fork CI) on the merged tree; any lock move is measured by a --locked workspace build.
@task_file n=1 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom layout tree construction — display: contents boxes and pseudo-elements, inline-only wrapping
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: blitz-dom layout damage and relayout tracking
@task_file n=3 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: blitz-dom inline layout building — inline content through display: contents
@task_file n=4 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline text layout — a textarea's text laid out once its width is known
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the blitz-dom node — is_or_contains_block through display: contents
@task_file n=6 path=packages/blitz-dom/src/traversal.rs mark=merge
what: blitz-dom tree traversal over display: contents children
@task_file n=7 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting — the IME preedit text
@task_file n=8 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: blitz-paint text decorations — the IME preedit underline
@task_file n=9 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's DOM interface bootstrap — DOMTokenList.replace
@task_file n=10 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM integration tests
@task_file n=11 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the stylo-to-taffy style conversion — float keyword sizes, logical float and clear sides
@task_file n=12 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the stylo-to-taffy style wrapper read by layout
@task_file n=13 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test target — its new mod line
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: upstream's new display: contents layout test file, run under the blitz-tests one-file-one-target rule
@task_file n=15 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: upstream's new IME preedit underline test file, run under the blitz-tests one-file-one-target rule
@task_file n=16 path=.github/workflows/wpt.yml mark=merge
what: upstream's WPT workflow — the Pages artifact named per run attempt, guarded by github.repository so WPT results publish upstream only
@task_file n=17 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT diff-to-PR tooling script
@task_file n=18 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: upstream's test of the WPT diff-to-PR tooling
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the WPT pin upstream tracks
@task_file n=20 path=AGENTS.md mark=merge
what: upstream's agent guidance file, standing left out of the take (its change undone in the merge's tree)
@task_file n=21 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent guidance for parley_code_editor, a crate kept out of the workspace; standing left out
@task_file n=22 path=deny.toml mark=edits
what: the dependency and advisory policy — edits: the RUSTSEC-2026-0192 ignore's reason, re-pointed to name its residual row
@task_file n=23 path=chunks/2026-11-22-upstream-sync-ahead-of-the-canvas/evidence/founder-forks.md mark=edits
what: the operator-forks record — each boundary widening, the Dioxus hold and each left-out commit with what crosses, the rule it meets, its options and measured cost, and its dependences
@task_file n=24 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning one file and one target per behaviour for the blitz-tests targets
@task_file n=25 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner, its fast and doc legs re-run on the merged tree
@task_item n=1 part=needs
The sink target allowlist: every new tracing or log call site and target the merged upstream delta brings is read against it; a target outside it is a fork, never silently admitted.
@task_item n=2 part=needs
The capture checks count framework targets at warn and above and must stay at 0 needles after the merge; a new engine log site that fires on the stand or in the headless shot moves them.
@task_item n=3 part=needs
Engine tracing stays behind the tracing cargo feature: any tracing the merged blitz-dom, blitz-paint, stylo_taffy or script-runtime code adds must compile in only with that feature, set by the build's invocation.
@task_item n=4 part=needs
Engine log sites write fixed messages and engine targets print safe fields only: a new upstream event in layout, paint or the script runtime carries no page text, node content or IME preedit text.
@task_item n=5 part=needs
No new print, env read, env var, listener, port or crate of ours arrives with the merge; a new print or env read in the merged delta is found by the sink re-check and named.
@task_item n=6 part=needs
No telemetry backend, OTel export, metrics exporter or error-reporting client may enter through the take; any lock move is measured by a --locked workspace build beside the audit.
@task_item n=7 part=needs
The IME preedit text painted by the new underline path is user-typed content: it stays in pixels and in memory and is never logged by escher's sink or the engine's targets.
@task_item n=8 part=needs
The script runtime's diagnostics drain and script error surface keep their contract across the DOMTokenList.replace change: errors and messages retained between drains, source labels unchanged.
@task_item n=9 part=needs
WPT results and scores stay published from upstream DioxusLabs/blitz only: the merged WPT workflow's Pages artifact step stays guarded by github.repository, and the fork uploads only its own two CI artifacts.
@task_item n=10 part=needs
CI workflow edits the merge brings keep every action pinned to a 40-hex SHA and ci.yml's contents: read; per-leg CI logs stay uploaded on failure and coverage on success.
@task_item n=11 part=needs
Fork CI on the pushed merge commit is read to a settled verdict, every job; the last wrap's unread verdict is re-read, never folded as green.
@task_item n=12 part=needs
Upstream's new test files run as escher's checks do: silent on success, with failures naming a kind and a count; any empty-stderr check unsets RUST_LOG.
@task_item n=13 part=needs
The screenshot and cross-OS readings re-measured on the merged tree; a reading the new paint or layout moves is recorded, never loosened.
@task_item n=14 part=needs
The operator-forks file holds no secret, no local state and no private content: it names commits, rules, options and measured costs only.
@task_item n=15 part=needs
The conflicts keep escher's intent over the snapshot, stable ids, select model and its list, hit stops, accessibility state reads, inert, the stamped sub-resource request and the file: refusal, and the buffered render; no merged change makes any of them log content.
@task_standing n=1
convention: each new or taken test runs silent on success and its failure names a kind and a count, never page content
home: 3-observability-harness-contract
@task_standing n=2
convention: each CI job or workflow step the take adds keeps actions SHA-pinned, uploads its leg log on failure, and publishes nothing from the fork that upstream publishes
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The driver's command and tool_call span fields and their ending values: the merge reaches no driver code and adds no span.
@task_item n=2 part=not_needed
The cold-agent pipe's events, verdict fields, counted verbs and kept checks: no cold-agent run or harness is built or changed.
@task_item n=3 part=not_needed
The agent-run harness events and its cleanup scope: the chunk runs no agent-run harness.
@task_item n=4 part=not_needed
escher-session's exit codes, commands, coverage role, host wire and mcp role output: none of its behaviour changes.
@task_item n=5 part=not_needed
The recording file, recording bounds, session lifecycle and private state directories: no session recording is touched.
@task_item n=6 part=not_needed
The actions, layout, graph and coverage answer shapes and the snapshot node and select-value contracts: the merge changes no answer of the driver.
@task_item n=7 part=not_needed
The build-directory sweep's stdout line and the package-install diagnostic line: neither is changed by this take.
