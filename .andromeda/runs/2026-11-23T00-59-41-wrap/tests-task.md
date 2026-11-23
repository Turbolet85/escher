@task_prose part=session
The session took upstream blitz main at 165d698c3 into the fork as one two-parent merge onto 31c8155fc: ten upstream commits, nine taken and ca0b97b05 left out whole, so the left-out set now reads 15.
The founder's six rulings landed as written. Page scripts gain DOMTokenList replace. Float and clear take a left-to-right flag for their logical sides. Block containment and box construction now handle display contents correctly. An IME composition underline is painted while a preedit stands. The textarea's text layout is deferred.
It resolved one conflict, in the upstream-only WPT workflow, and kept the SHA-pinned deploy-pages action and both repository guards. It hand-edited one line of the dependency audit config: the RUSTSEC-2026-0192 ignore reason now names its owning residual row. Upstream AGENTS.md files were removed again.
No dependency moved (Cargo.toml and Cargo.lock unchanged, Dioxus held at 0.7.3, the hold re-read at this sync), and no crate was added or removed. blitz-tests gains one upstream target by cargo's discovery, with no manifest edit.
build, fast, doc, a11y and audit legs green; stand-repeat 3 of 3 rounds green; CI 17 of 17 green on the merge commit.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: the upstream-only WPT conformance workflow and its Pages deploy job; resolved a conflict in it, keeping the SHA-pinned deploy-pages action with upstream's per-run-attempt artifact name and both github.repository guards
@task_file n=2 path=deny.toml
what: the dependency audit config (advisories and licenses); edited the RUSTSEC-2026-0192 ignore reason to name the residual row that owns it, re-checked by the audit leg on every push
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: blitz-dom box construction; merged upstream's display-contents child flow and the textarea's deferred text layout
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: blitz-dom incremental layout damage; merged upstream's textarea deferred text layout
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: blitz-dom inline layout building; merged upstream's float and display-contents fixes
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline layout; merged upstream's logical float and clear sides for inline floats
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the blitz-dom node; merged upstream's block-containment read that recurses through display contents
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: blitz-dom tree traversal macros; merged upstream's removal of its plain child-iteration macro
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: the blitz-paint text painter; merged upstream's IME composition underline paint
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: blitz-paint text decorations; merged upstream's composition underline decoration
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the vibey-script page-script DOM interfaces; merged upstream's DOMTokenList replace
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the vibey-script DOM tests; merged upstream's classList replace test
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the stylo_taffy style conversion exports; merged upstream's float and clear conversions taking a left-to-right flag
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the stylo_taffy style wrapper, the conversions' one caller; merged upstream's pass of the direction flag
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests aggregate target; merged upstream's one added line
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests display-contents layout target; merged upstream's test of a block inside contents inside an inline filling the width
@task_file n=17 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new blitz-tests integration target from upstream; a pixel check on the CPU buffer that composing text stays underlined until committed
@task_file n=18 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT commit the conformance runner reads; moved by upstream to 9986bb3
@task_file n=19 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: the upstream-only WPT diff-to-PR CI script; merged upstream's change that shows results beside a revision mismatch
@task_file n=20 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the unittest tests of the WPT diff-to-PR CI script; merged upstream's tests for a revision mismatch with and without a diff
@task_item n=1 part=touches
WPT workflow on upstream only and upstream-only publishing jobs: the conflict resolution kept both github.repository guards, and the deploy step now names its artifact per run attempt.
@task_item n=2 part=touches
CI actions pinned to commit SHAs: the conflict resolution kept deploy-pages pinned to its SHA (v5) against upstream's text.
@task_item n=3 part=touches
WPT pinned to a recorded commit: the pin moved to 9986bb3 with upstream's commit.
@task_item n=4 part=touches
CI Python scripts unit-tested and Python script tests under unittest: upstream's WPT diff-to-PR script and its unittest file changed together, with new revision-mismatch cases.
@task_item n=5 part=touches
Fork WPT reading is report-only and WPT conformance is report-only: the upstream WPT diff tooling now reports results beside a revision mismatch instead of only failing to diff.
@task_item n=6 part=touches
Dependency audit (advisories and licenses): the RUSTSEC-2026-0192 ignore reason now names its owning residual row and says the audit leg re-checks it on every push. The audit stays green with one license-not-encountered warning, NCSA, as before.
@task_item n=7 part=touches
One test target per file, one blitz-tests target per file, and one integration test target per behaviour: blitz-tests gains the upstream IME preedit underline target as its own file, found by cargo's discovery.
@task_item n=8 part=touches
blitz-tests manifest holds one test target, and the blitz-tests manifest's difference from upstream: a new target arrived without any manifest edit. A row that counts the targets or states the manifest's difference should be read against that.
@task_item n=9 part=touches
Stand checks repeated three rounds and named stand-repeat targets: the new target is no stand_ target and is not named in stand-repeat. stand-repeat read 3 of 3 green, so the zero-failure budget held.
@task_item n=10 part=touches
Workspace tested in the fast CI leg: the new upstream tests (the classList replace unit test, the display-contents case, the IME underline target) run in fast, which read 268 test results ok and 0 failed.
@task_item n=11 part=touches
Pixel tests on a CPU buffer: the new IME underline check renders through the CPU image renderer and counts pixels.
@task_item n=12 part=touches
Layout assertions in both layout modes, stand checks in both layout modes, and the incremental layout differential oracle: the logical float sides, the display-contents fixes and the textarea's deferred layout change layout for every document. Their coverage was read through the oracle in both modes and through the stand checks.
@task_item n=13 part=touches
Upstream tests kept as written (buffered-render kept, mounted-flush left out): this take brings upstream's tests unedited. ca0b97b05 is left out whole, and the left-out set reads 15.
@task_item n=14 part=touches
Upstream-authored oversize files kept and upstream's duplicated test pairs kept: upstream's layout, paint, conversion and test files merged clean and unedited.
@task_item n=15 part=touches
keyboard_compat.rs left out of coverage: the Dioxus hold was re-read at this sync and held. The exclusion's re-read is now set for the next sync or the first sync after a Dioxus 0.8 release.
@task_item n=16 part=touches
Buffered render on the Dioxus 0.7 bridge: Dioxus is held at 0.7.3 (lock 0.7.10), and Cargo.toml and Cargo.lock are unchanged.
@task_item n=17 part=touches
Cargo legs run locked: the direction-flag signature change was checked across every caller by the locked build.
@task_item n=18 part=touches
WPT suites run in CI and WPT run on the host on the fork: the WPT pin and the WPT tooling moved underneath them, upstream's change.
@task_fact n=1 slug=test-strategy
fact: The IME composition underline is checked headless by upstream's pixel target, which counts underline pixels on the CPU buffer while a preedit stands and after commit, in the fast leg. No windowed IME run checks it.
@task_fact n=2 slug=unit-test-strategy
fact: Page-script DOMTokenList replace is covered by upstream's vibey-script DOM unit test, taken as written and run in the fast leg.
@task_item n=1 part=untouched
Session lifecycle, session wire, session host, session errors and session state directory contracts: no session, socket or wire code moved.
@task_item n=2 part=untouched
escher-session command line, CLI and MCP role contracts, exit codes and stderr lines: no command, tool, exit code or log line changed.
@task_item n=3 part=untouched
Driver verbs, driver session, refusal causes and lint defect contracts: no driver verb, cause or order moved.
@task_item n=4 part=untouched
Cold-agent and agent-run pipes, their events, verdicts, exit codes and selections: untouched.
@task_item n=5 part=untouched
Harness construction, settle, virtual clock, input, inspection, contrast and screenshot contracts: the harness API is unchanged. The new upstream test only calls it.
@task_item n=6 part=untouched
Mocking discipline, hand-written stand-ins, timer ticks and network stand-ins: no stand-in moved.
@task_item n=7 part=untouched
Per-crate coverage floors, mutation testing rules and the complexity ceiling: no floor, threshold or mutation run moved. No coverage row is owned by this chunk.
@task_item n=8 part=untouched
Recorded checks, session recording and handler coverage: no recording, check file or coverage line changed.
@task_item n=9 part=untouched
CI leg script, its exit codes, cache, trigger branches, package installs, MSRV and the fast/slow split: the fork's own CI legs and gates are unchanged. Only upstream-only WPT tooling moved.
@task_item n=10 part=untouched
Telemetry target allowlist, sink capture and no ids or names in logs: the take added no log, sink or env var site (0 over 214 added product lines).
