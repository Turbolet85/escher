@task_prose part=session
The session merged upstream blitz main 165d698c3 into the fork as a two-parent merge, taking all ten upstream commits as written (the founder's rulings E1 to E6, option A on every fork).
Engine changes taken: float and clear resolve their logical sides by writing direction (stylo_taffy convert float and clear gain an ltr flag), display contents fixes to block containment and box construction, the textarea's deferred text layout, an IME composition underline painted under preedit text in every document, and DOMTokenList replace for page scripts.
Upstream WPT tooling moved: the WPT pin bumped, the diff-to-PR script reports results beside a revision mismatch, the Pages artifact is named per run attempt; one workflow conflict was resolved by hand keeping the pinned deploy action and both repository guards.
The cargo-deny ignore of RUSTSEC-2026-0192 now names its owning residual row; no dependency, crate, workspace member, IPC method, endpoint, env var or CLI flag changed; Dioxus stays held at 0.7.3, re-read at this sync.
The upstream sync position moved 547d7514b to 165d698c3; the left-out upstream set reads 15 commits; blitz-tests gained one target; CI green 17 of 17, stand-repeat 3 of 3 rounds green.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: upstream's WPT workflow, its Pages deploy step and its repository guards
@task_file n=2 path=deny.toml
what: the cargo-deny policy, its advisory ignores with their reasons and its license allow list
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout box construction, inline versus block child flow under display contents, textarea text layout
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage and relayout invalidation, the textarea's deferred text layout
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout building, how inline boxes are gathered into a text layout
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout and float placement inside inline content
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the DOM node, its block containment queries through display contents
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the DOM tree traversal helpers, children and pseudo elements iteration
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: the paint of text runs, including the IME composition underline under preedit text
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the paint of text decorations, underlines and the composition underline
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's DOM interfaces exposed to page scripts, DOMTokenList among them
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM API tests, classList replace among them
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style to layout conversion, float and clear sides resolved by writing direction
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style to layout wrapper, the caller of the float and clear conversion
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate test target, its module list
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine's display contents layout tests, block width inside contents inside inline
@task_file n=17 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new blitz-tests target checking composing text is underlined until committed
@task_file n=18 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision the WPT runs read
@task_file n=19 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT results diff to PR comment script
@task_file n=20 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of upstream's WPT results diff to PR comment script
@task_item n=1 part=touches
The escher-session layout answer: boxes it reports can move on pages using floats with a writing direction, display contents holding a block inside an inline, or a textarea, through the engine's layout changes.
@task_item n=2 part=touches
The escher-session screenshot answer: a screenshot taken while an input holds IME preedit text now shows a composition underline under it.
@task_item n=3 part=touches
Open list rows read their own bounds: the bounds come from the engine's layout, whose block containment through display contents changed; check the decision still holds as worded.
@task_item n=4 part=touches
Visible region through clipping boxes: computed over laid-out boxes the engine now builds differently under display contents; check its wording names no construction rule the merge changed.
@task_item n=5 part=touches
Headless stand pinned viewport and held instance reads as a fresh boot: the stand's headless screenshots re-ran under the merged engine, stand-repeat 3 of 3 rounds green; no cell value moved, check only.
@task_item n=6 part=touches
Session recording as a replayable check: recorded checks replay against the merged engine's layout and paint; any recorded box or pixel expectation a row holds may need a re-read.
@task_item n=7 part=touches
Absence of arbitrary index.html templates for a document: the merge adds no template or document skeleton capability; confirm the absence still stands.
@task_item n=1 part=untouched
The stand stable element ids: no id was added, renamed or freed; the stand pages were not edited.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window chrome method or behaviour changed.
@task_item n=3 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines: no argument, exit code, output path or usage line changed.
@task_item n=4 part=untouched
The escher-session MCP server, exit codes, refusals, recording files, served tasks, actions, coverage, graph, help, lint and host log contracts: no verb, tool, code or field changed.
@task_item n=5 part=untouched
The stand's accent colour, its a11y by attributes, its keyed controls, lint-clean boot, load page network provider, order page and flight type select decisions: no stand source or app UI changed.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the decisions log: not measured and not reached.
