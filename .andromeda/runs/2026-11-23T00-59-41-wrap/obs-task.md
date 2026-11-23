@task_prose part=session
The session merged upstream DioxusLabs/blitz main at 165d698c3 into the fork as one two-parent merge: ten upstream commits, nine taken and one left out, so the left-out set now reads 15. The founder took all six forks as upstream wrote them.
What came in from upstream: float and clear take logical sides through a new ltr flag, display: contents fixes for block containment and box construction, the textarea's text layout deferred, an IME composition underline painted in every document with a preedit, and DOMTokenList.replace for page scripts.
Two files were edited by hand. The WPT workflow's conflict was resolved by keeping the fork's deploy-pages pin, adding upstream's per-run-attempt Pages artifact name and keeping both repository guards. The ignore for RUSTSEC-2026-0192 in the dependency audit policy now names the residual row that owns it.
Upstream's AGENTS.md files arrived and were removed again. No dependency, crate, workspace member, IPC method, endpoint, port, env var or log site was added (0 new sink sites over 214 added product lines).
The blitz-tests crate gains one test target for the IME underline. Every leg passed locally and CI passed 17 of 17 on the merge commit.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: the WPT workflow, upstream-only, that runs the WPT suite and publishes WPT scores to Pages from upstream DioxusLabs/blitz only, behind its github.repository guards; resolved a conflict in it (the Pages deploy step's artifact name)
@task_file n=2 path=AGENTS.md mark=conflict
what: upstream's agent instructions file, kept out of the fork's tree; resolved a modify/delete conflict in it by deleting it, so the tree is unchanged
@task_file n=3 path=deny.toml
what: the dependency audit policy read by the audit leg (advisories and licences); edited the reason on its RUSTSEC-2026-0192 ignore
@task_file n=4 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout box construction (display: contents children, textarea text layout); merged into it
@task_file n=5 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage and incremental relayout; merged into it
@task_file n=6 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout builder; merged into it
@task_file n=7 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout and float placement; merged into it
@task_file n=8 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's DOM node (block containment through display: contents); merged into it
@task_file n=9 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the engine's tree-traversal macros; merged into it (one upstream macro removed)
@task_file n=10 path=packages/blitz-paint/src/text.rs mark=merge
what: the engine's text painting (the IME composition underline); merged into it
@task_file n=11 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the engine's text decoration painting; merged into it
@task_file n=12 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the page-script runtime's DOM interface bootstrap (DOMTokenList, classList); merged into it
@task_file n=13 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the page-script runtime's DOM tests, run in the fast leg; merged into it
@task_file n=14 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion (float and clear now take the writing direction); merged into it
@task_file n=15 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper, the only caller of the float and clear conversions; merged into it
@task_file n=16 path=tests/blitz-tests/tests/all.rs mark=merge
what: the engine test crate's aggregate test target; merged into it
@task_file n=17 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine test crate's display: contents layout tests; merged into it
@task_file n=18 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new engine test target for the IME composition underline paint, run in the fast leg; added by the merge
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the WPT runner's pinned web-platform-tests revision; merged into it
@task_file n=20 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream-only WPT tooling that reports WPT result differences on a pull request (results now shown beside a revision mismatch); merged into it
@task_file n=21 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of the upstream-only WPT diff tooling; merged into it
@task_file n=22 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent instructions for a crate that is not a workspace member and is kept out of the fork; arrived with the merge and was deleted, so the tree is unchanged
@task_item n=1 part=touches
The rows saying WPT scores and results are published to Pages from upstream only. The WPT workflow changed in the merge: the Pages artifact is now named per run attempt, both github.repository guards were kept, and publishing is still upstream-only.
@task_item n=2 part=touches
The Metric Coverage overview's sentence on the WPT runner's figures and Pages publishing on upstream only. The WPT pin and the upstream WPT diff tooling moved, and the publishing reach did not change.
@task_item n=3 part=touches
The WPT report contract items (message, subtest message). The upstream WPT diff tooling now shows results beside a revision mismatch. Check that no item of the report's shape moved with it.
@task_item n=4 part=touches
The fork CI artifact rules (two artifacts uploaded by fork CI, per-leg log on failure, coverage on success). A workflow that uploads an artifact changed, but its Pages artifact stays behind the upstream-only guards, so the fork's artifact set holds.
@task_item n=5 part=touches
The CI-on-GitHub-workflows decision. A workflow file changed through a resolved merge conflict, with the deploy-pages action still pinned by SHA. CI passed 17 of 17 on the merge commit.
@task_item n=6 part=touches
Engine tracing behind a cargo feature, and engine tracing set by the build's invocation. These rules cover the engine crates (blitz-dom, blitz-paint, stylo_taffy, the page-script runtime) that the merge changed. No tracing site, feature or dependency was added.
@task_item n=7 part=touches
The rules that engine log sites write fixed messages and that engine targets print only safe fields. These rules cover the layout, paint and conversion code that changed. The sink measure found 0 new log sites.
@task_item n=8 part=touches
The sink target allowlist, and the capture that counts framework targets at warn and above. The engine code changed but adds no target and no warn-level site, and the capture checks inside the fast leg passed.
@task_item n=9 part=touches
The rule that the layout read opens no span. Layout construction, damage, inline layout and float placement changed underneath it, and no span was added.
@task_item n=10 part=touches
The rules that session logs carry no page content and that the shot image leaves only in its answer. The new IME composition underline paints preedit text into every document with an IME. It is a paint only and logs nothing.
@task_item n=11 part=touches
The script error surface and the script-diagnostics drain contracts (window error event, take_js_errors, take_messages). A new page-script DOM method, DOMTokenList.replace, was added to the runtime bootstrap whose errors these contracts carry.
@task_item n=12 part=touches
The rule that the headless stand runs with no sink. The stand checks ran three rounds green in stand-repeat over the changed layout code. The new IME test target is not a stand target.
@task_item n=13 part=touches
The absence rows for a telemetry backend, metrics export and an error-reporting service. The merge added no dependency (Cargo.toml and Cargo.lock unchanged), so each absence still holds.
@task_item n=1 part=untouched
escher's own telemetry sink and log line contracts (stream, RUST_LOG, line format, escaping, scrub, span lines, resource keys service.name and service.version). The session touched no escher crate.
@task_item n=2 part=untouched
The escher-driver command and tool_call spans, their fields, the refusal cause names and the tool_call endings. No driver code or span changed.
@task_item n=3 part=untouched
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, kept checks, transcripts and cleanup. Nothing in either harness moved.
@task_item n=4 part=untouched
escher-session's roles, exit codes, stdout, stderr, host wire, coverage line, recording file, recording directory, session lifecycle and state directory. The binary and its contracts are unchanged.
@task_item n=5 part=untouched
The PII scrubbing rows of escher's sink and the driver answers: the content-named field set, the snapshot, graph, layout and actions answers, select and optgroup labels, and the mutation buffer. No answer shape or scrub rule changed.
@task_item n=6 part=untouched
escher-telemetry's panic hook, its startup events, its scrub set, and the rule that the sink is installed first in each native main. No native main or telemetry crate changed.
@task_item n=7 part=untouched
The build-directory sweep line and the package-install diagnostic line contracts. Neither tool changed.
@task_item n=8 part=untouched
The rows for the no-OTel-export decision and the tracing-as-telemetry-crate choice. Their reach is unchanged.
