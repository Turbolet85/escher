@task_prose part=chunk
Syncs this fork with upstream as one real two-parent merge of upstream's main (ten commits: display: contents layout and pseudo-elements, float keyword sizes and logical float sides, the IME preedit underline paint, a textarea's text laid out once its width is known, DOMTokenList.replace in the script runtime, WPT tooling and pin, a CI Pages artifact name, upstream's agent notes).
Each commit left out is named with its reason; the standing left-out set (the code-editor package out of the workspace, upstream's agent notes excluded, Dioxus held at 0.7.3) is re-read, and every left-out commit, the Dioxus hold and every boundary widening the merge brings is written as an operator fork, never answered by a default.
Any lock move is measured by a locked workspace build beside the dependency audit; every escher proof is re-run after the merge (fast and doc CI legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the embolden witness, screenshot and cross-OS readings re-measured, never loosened).
Re-reads the merge-surface residuals and the sink allowlist over the merged delta, and re-points one advisory ignore's reason to name its owning residual row.
@task_file n=1 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout tree construction — display: contents children and pseudo-elements, inline-only wrapping
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage — which changes relayout a node, the changed set's contract
@task_file n=3 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout build — inline text and the IME preedit range in it
@task_file n=4 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout — a textarea's text laid out once its width is known
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's DOM node — the block-or-contains-block test through display: contents
@task_file n=6 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the engine's DOM tree traversal helpers
@task_file n=7 path=packages/blitz-paint/src/text.rs mark=merge
what: the engine's text painting — glyph runs and the IME preedit underline
@task_file n=8 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the engine's text decoration painting — underlines, including the preedit underline
@task_file n=9 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion — float keyword sizes and logical float and clear sides
@task_file n=10 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper the layout engine reads styles through
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's DOM interfaces bootstrap — DOMTokenList.replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM tests
@task_file n=13 path=tests/blitz-tests/tests/all.rs mark=merge
what: the engine test suite's aggregate test target — upstream's new mod line, checked against the one file and one target per behaviour rule
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: upstream's new engine test for display: contents layout
@task_file n=15 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: upstream's new engine test for the IME preedit underline paint
@task_file n=16 path=.github/workflows/wpt.yml mark=merge
what: upstream's WPT CI workflow — guarded by the repository check, its actions pinned to full SHAs
@task_file n=17 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT CI tooling that turns a conformance diff into a pull request
@task_file n=18 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: upstream's tests of that WPT CI tooling
@task_file n=19 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision the conformance runner reads
@task_file n=20 path=AGENTS.md mark=merge
what: upstream's agent notes at the repository root — a standing left-out change, re-read
@task_file n=21 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's agent notes for the code-editor package kept out of the workspace — a standing left-out change, re-read
@task_file n=22 path=Cargo.toml mark=merge
what: the workspace manifest — its members (the escher crates and seven_guis) and the Dioxus pin held at 0.7.3 against upstream's 0.8 alpha
@task_file n=23 path=deny.toml mark=edits
what: the dependency audit's advisory policy — edits: the RUSTSEC-2026-0192 ignore's reason, to name its owning residual row
@task_file n=24 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the engine test suite to one file and one target per behaviour
@task_file n=25 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the merge — fast and doc
@task_item n=1 part=needs
The merge takes display: contents into layout: a display: contents element's children and pseudo-elements lay out in its parent's place, inline-only content wraps, and the block-or-contains-block test sees through it. This is a layout behaviour change the stand checks in both layout modes and the incremental oracle must still hold.
@task_item n=2 part=needs
The merge takes float keyword sizes and logical float and clear sides in the style-to-layout conversion, a layout behaviour change.
@task_item n=3 part=needs
The merge takes a textarea's text being laid out once its width is known, which changes how text-field content is measured. The screenshot and cross-OS readings that cover it are re-measured if they move, never loosened.
@task_item n=4 part=needs
The merge takes the IME preedit underline paint, so text being composed now paints an underline. Its colour and thickness are engine paint constants. The screenshot verb must still never paint with text skipped.
@task_item n=5 part=needs
The merge takes DOMTokenList.replace into the script runtime DOM, a new method page script can call.
@task_item n=6 part=needs
Conflicts keep both sides' intent: the changed set's contract, the clock invariant (one virtual clock, a load wait moves no clock, settle does not wait on animation), the select model and its list, the hit stops at paint clips and the scrolled-box bounds, the into-view readings, the stable ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, and the buffered render replayed under one borrow. No workaround is bought only to keep upstream code untouched.
@task_item n=7 part=needs
Fonts in the merged tests: upstream's new engine tests run as this fork's engine tests, built with system fonts like the rest.
@task_item n=8 part=needs
Dioxus stays held at 0.7.3 against upstream's 0.8 alpha as a fork for the operator. Holding a pin upstream moved undoes upstream's change and is named with its reason. The dioxus-native configuration the fork's surfaces read (base colour, font context, alpha mode) keeps its 0.7.3 shape unless the operator takes the move.
@task_item n=9 part=needs
Coupled dependency pins: a take carries upstream's pins as one coupled set or holds the whole set. The workspace takes no git dependency except Taffy's pinned rev, no dependency is bumped beyond upstream's own set, and MSRV stays 1.91 unless a taken pin requires more, in which case that is named.
@task_item n=10 part=needs
Lock moves are measured by a locked workspace build next to the dependency audit, never by the audit alone. The merge diff itself carries no manifest or lock change.
@task_item n=11 part=needs
Every advisory ignore in the audit policy names the residual row that owns it. The RUSTSEC-2026-0192 reason is re-pointed to its owner row.
@task_item n=12 part=needs
Each boundary widening, the Dioxus hold and each left-out commit (the standing ones re-read included) is written as one operator fork in the chunk's founder-forks evidence file. Each fork states what crosses, the rule it meets, its options with their measured cost and its dependences on the others, and is checked against the escalate patterns. Technical forks go to the operator as dialogs.
@task_item n=13 part=needs
CI workflow edits keep every action pinned to a 40-hex SHA and the main workflow's read-only contents permission. Upstream's workflow files stay guarded by the repository check.
@task_item n=14 part=needs
Sink allowlist: the merged delta is read for every new tracing or log call site, target, print and env read, and the standing capture checks stay at zero needles.
@task_item n=15 part=needs
No new crate, port, env var or listener of ours. The escher crates and the seven_guis members stay in the workspace.
@task_item n=16 part=needs
The escher-session driver's observable answers (snapshot, screenshot, layout, graph, actions, coverage, exit statuses) must read the same after the merge, and its screenshot and snapshot readings are re-measured where a merged layout or paint change moves them.
@task_item n=17 part=needs
The a11y leg and the accessibility tree reads, including the select's chosen option, open state and highlighted row, still pass after the merge.
@task_item n=18 part=needs
Fork CI must be green on the pushed merge commit with every job settled, and the last wrap commit's unread verdict is re-read before the push.
@task_standing n=1
convention: each engine test target the merge adds is built with system fonts, like every other engine test target
home: typography
@task_standing n=2
convention: each screenshot reading the merge moves is re-measured and recorded with text painted, never with text skipped and never loosened
home: typography
@task_item n=1 part=not_needed
Brand identity, border radius and spacing scales: the chunk adds no visual token, radius or spacing value of its own.
@task_item n=2 part=not_needed
Cursor icon resolution and the favicon read: no merged commit touches cursor or icon handling.
@task_item n=3 part=not_needed
The shell provider's window controls (minimize, maximize, decorations, window drag): the merge reaches no window-shell code.
@task_item n=4 part=not_needed
The WPT runner's terminal output, its result colours and test-name hyperlinks: the merge changes WPT CI tooling and the pin, not the runner's own output, and upstream's conformance numbers are out of scope.
@task_item n=5 part=not_needed
The web-spa surface, where WASM pages render into a canvas and the WASM bundled font context lives: the merge carries no WASM page or canvas change, and the canvas entries come later.
@task_item n=6 part=not_needed
Mobile targets: they are built, not tested, and no merged commit touches them.
@task_item n=7 part=not_needed
The escher-session tasks table (counter, timer, crud, cells, flight-booker and the rest) and the command arguments and help text: the chunk adds no command or task.
@task_item n=8 part=not_needed
Overlay scrollbar fade timings and smooth scroll with wheel cancel: no merged commit touches scrolling or scrollbar motion.
@task_item n=9 part=not_needed
Font element hints, the lang attribute setting the text locale, root-relative units, table cellspacing hints and body margin hints: no merged commit touches these presentational hints or units.
