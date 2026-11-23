@task_prose part=session
The session took ten upstream engine commits in one two-parent merge, moving the fork's last taken upstream commit to 165d698c3; one upstream commit was left out, so the left-out set now reads 15.
It took the founder's six upstream changes as written: page scripts gain classList.replace; the IME composition (preedit) text is painted with an underline until it is committed, in every document; floats and clear resolve their left and right sides from the text direction; display: contents children are built and tested as block or inline correctly; and a textarea's text layout is deferred.
It resolved one conflict, in upstream's WPT pages workflow, and hand-edited the advisory ignore list so RUSTSEC-2026-0192's reason names the residual that drops it. Upstream's agent instruction files arrived in the merge and were removed, so the committed tree is unchanged by them.
No dependency moved; Dioxus is held at 0.7.3 and was re-read at this sync. blitz-tests gains one test target for the composition underline, and CI was green on 17 of 17 checks.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: upstream's WPT workflow that runs the WPT suite and deploys its results to Pages, the Pages artifact now named per run attempt, and the repository guards that keep it upstream-only
@task_file n=2 path=deny.toml
what: the dependency audit's advisory and licence policy, the list of ignored advisories, each with the reason it is ignored
@task_file n=3 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout box construction, deciding whether children flow as blocks or inline, including under display: contents, and the textarea's text layout
@task_file n=4 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage tracking, which decides what is laid out again after a change, now deferring a textarea's text layout
@task_file n=5 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout builder, which assembles inline content and the boxes placed inside it
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, placing floats inside inline content by their logical sides
@task_file n=7 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's DOM node, including whether a node is or contains a block, now looking through display: contents
@task_file n=8 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the engine's tree traversal helpers over a node's children and pseudo-elements
@task_file n=9 path=packages/blitz-paint/src/text.rs mark=merge
what: the engine's text painting, now painting the IME composition underline under preedit text
@task_file n=10 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the engine's text decoration painting (underline, overline, line-through), now with a composition underline decoration taking a colour
@task_file n=11 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's DOM interfaces exposed to page scripts, now with DOMTokenList replace (classList.replace)
@task_file n=12 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM tests, now with a classList.replace test
@task_file n=13 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the conversion of computed styles into layout styles, whose float and clear conversions now take the text direction
@task_file n=14 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper that hands an element's converted styles to layout, the caller of the float and clear conversions
@task_file n=15 path=tests/blitz-tests/tests/all.rs mark=merge
what: the engine integration tests' aggregate target, the list of test modules it includes
@task_file n=16 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine integration tests for display: contents layout, now with a block inside contents inside an inline filling the width
@task_file n=17 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision the WPT suite runs against
@task_file n=18 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT tooling that posts the WPT results diff to a pull request, now showing results beside a revision mismatch
@task_file n=19 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of upstream's WPT results diff tooling
@task_file n=20 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new engine integration test target that renders a text input with composing IME text and checks it is underlined until committed
@task_item n=1 part=touches
Engine colours hardcoded: the composition underline is a new engine-painted decoration whose colour is given in engine code; check whether the row's list of hardcoded engine colours must take it.
@task_item n=2 part=touches
Colour tokens absent in the engine crates: the composition underline's colour adds another engine colour with no token; the absence still holds but gains an instance.
@task_item n=3 part=touches
Screenshot never paints with text skipped: a textarea's text layout is now deferred, and screenshots of a document with composing IME text now carry the underline; check the rule still holds over a deferred textarea.
@task_item n=4 part=touches
driver screenshot and its refusals: what a screenshot paints moved, as an input with IME preedit text now shows an underline under it.
@task_item n=5 part=touches
script runtime DOM contract: page scripts gain classList.replace, which belongs beside the script runtime DOM items such as document.fonts.
@task_item n=6 part=touches
Tests built with system fonts and test engine built with system fonts: a new engine integration test target renders text through the CPU renderer; check it follows the same font rule.
@task_item n=7 part=touches
WPT runner output and result colours: upstream's WPT tooling moved (the results diff shows results beside a revision mismatch, the WPT pin moved, the Pages artifact is named per run attempt); check whether any WPT runner row reaches that tooling.
@task_item n=8 part=touches
dioxus-native configuration contracts (base colour, font context, alpha mode, WASM font context): Dioxus stayed held at 0.7.3 and was re-read at this sync; these hold, but any row naming the Dioxus version or its hold is touched.
@task_item n=9 part=touches
Stacking-context grouping and hit-testing at a paint clip, indirectly: box construction under display: contents changed which children are built as blocks, which feeds the boxes that paint and hit-testing walk.
@task_item n=10 part=touches
lang attribute sets the text locale, indirectly: floats and clear now resolve left and right from the text direction, so a rule tying text direction or locale to layout may sit beside it.
@task_fact n=1 slug=typography
fact: The engine paints an underline under IME composition (preedit) text in every document with a preedit, until the text is committed; the underline is an engine decoration, not page CSS.
@task_fact n=2 slug=typography
fact: float and clear left and right sides are resolved from the text direction, so inline-start and inline-end swap in right-to-left text.
@task_item n=1 part=untouched
Select list colours, the chosen row's Chromium pair and the open select list's events, accessibility, overlay, motion and placement: no select code moved.
@task_item n=2 part=untouched
Motion and the virtual clock: driver advance, settle, load wait, scroll behaviour and overlay scrollbar fades are unchanged.
@task_item n=3 part=untouched
Iconography: cursor resolution, the cursor-icon crate and the favicon are unchanged.
@task_item n=4 part=untouched
escher-session commands, exit statuses and codes, stdout and stderr lines, the mcp role, tasks, and the actions, coverage and graph lines: no part of the session driver moved.
@task_item n=5 part=untouched
The shell provider's window controls, the JS window sizes and device pixel ratio, and the logical viewport: unchanged.
@task_item n=6 part=untouched
Spacing hints (body margins, table cellspacing and cellpadding) and font element presentational hints: no presentational hint moved.
@task_item n=7 part=untouched
The web-spa canvas surface and mobile targets built but not tested: no platform target or surface changed.
@task_item n=8 part=untouched
Colour scheme recascade and the viewport's system colour scheme: unchanged.
