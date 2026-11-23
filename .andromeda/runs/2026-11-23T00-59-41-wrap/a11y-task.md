@task_prose part=session
The session merged upstream Blitz main at 165d698c3 into the fork as a two-parent merge, taking ten upstream commits, with the founder's ruling to take all six upstream changes (E1 to E6) as written.
Taken from upstream: page scripts gain DOMTokenList replace (classList.replace); float and clear resolve logical sides through a left-to-right flag; block containment and box construction through display: contents are fixed; text being composed through an IME is painted with an underline until committed; a textarea's text layout is deferred.
One merge conflict was resolved in upstream's WPT workflow (both repository guards kept) and upstream's agent-instructions files were kept out of the tree by deletion.
The audit config's ignored advisory now names the residual row that owns dropping it; no dependency, crate, manifest or lock changed, Dioxus still held.
The fork's last taken upstream commit moved 547d7514b to 165d698c3, the left-out upstream set reads 15 commits (was 14), and the layout test crate gains one test target, ime_preedit_underline.
Every CI leg ran green, the a11y leg and stand-repeat (3 of 3 rounds) included; CI 17 of 17 green on the merge commit.
@task_file n=1 path=.github/workflows/wpt.yml mark=conflict
what: upstream's WPT conformance workflow and its Pages deploy step, guarded to the upstream repository
@task_file n=2 path=AGENTS.md mark=conflict
what: upstream's agent-instructions file, kept out of the fork's tree; its modify/delete conflict resolved by deletion
@task_file n=3 path=packages/parley_code_editor/AGENTS.md mark=merge
what: upstream's code-editor package agent notes, arriving with the merge and removed, the package not a workspace member
@task_file n=4 path=deny.toml
what: the dependency audit's config, its ignored advisory RUSTSEC-2026-0192 and that ignore's stated reason and owner
@task_file n=5 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: the engine's layout box construction, display: contents children and a textarea's deferred text layout
@task_file n=6 path=packages/blitz-dom/src/layout/damage.rs mark=merge
what: the engine's layout damage and relayout marking, a textarea's deferred text layout
@task_file n=7 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout building of text runs and inline boxes
@task_file n=8 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout and float placement, floats placed by their logical sides
@task_file n=9 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine's DOM node, whether a node is or contains a block, through display: contents
@task_file n=10 path=packages/blitz-dom/src/traversal.rs mark=merge
what: the engine's DOM child traversal helpers, one child-iteration helper retired
@task_file n=11 path=packages/blitz-paint/src/text.rs mark=merge
what: the engine's text painting, an IME composition's text drawn with an underline
@task_file n=12 path=packages/blitz-paint/src/text_decorations.rs mark=merge
what: the engine's text decoration painting, the composition underline decoration
@task_file n=13 path=packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js mark=merge
what: the script runtime's DOM interfaces given to page scripts, DOMTokenList replace added
@task_file n=14 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script runtime's DOM API tests, classList.replace tested
@task_file n=15 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-layout conversion, float and clear taking a left-to-right flag
@task_file n=16 path=packages/stylo_taffy/src/wrapper.rs mark=merge
what: the style-to-layout wrapper, the only caller of the float and clear conversion
@task_file n=17 path=tests/blitz-tests/tests/all.rs mark=merge
what: the layout test crate's aggregate test target and its module list
@task_file n=18 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the layout tests of display: contents, a block inside contents inside an inline filling its width
@task_file n=19 path=tests/blitz-tests/tests/ime_preedit_underline.rs mark=merge
what: a new layout test target, composing IME text drawn underlined in an input until committed
@task_file n=20 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT revision upstream's conformance run reads
@task_file n=21 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: upstream's WPT result-diff reporter, showing results beside a revision mismatch
@task_file n=22 path=.github/scripts/test_wpt_diff_to_pr.py mark=merge
what: the tests of upstream's WPT result-diff reporter
@task_item n=1 part=touches
Text being composed through an IME is now painted with an underline until it is committed, in every document with a preedit, inputs and textareas included; it belongs beside the decision that IME is enabled for an input focused before its editor, which rules over the same text-entry path.
@task_item n=2 part=touches
Indirect: rows that read hidden or focusable content through display: contents (focus kept by a control under display: contents, hidden content kept out of the tree) rule over the engine's box construction and block containment, which the merge changed for display: contents children; the change moves block containment and boxes, not hiddenness or focusability.
@task_item n=3 part=touches
Indirect: rows naming a textarea (a textarea as a disabled form control, a textarea's role or value in the tree and snapshot) rule over the textarea, whose text layout is now deferred until it is needed.
@task_item n=4 part=touches
Indirect: contracts on layout-read bounds (the snapshot's option, row and element bounds, the lint contrast finding's bounds, the driver's element screenshot) read geometry from the layout the merge changed for floats with logical sides and for display: contents.
@task_item n=5 part=touches
Indirect: the rule that a hidpi or zoom change reshapes text sits over the engine's text and inline layout, which the merge changed (inline building, deferred textarea text layout).
@task_item n=6 part=touches
Indirect: the decision that a script's element.style goes through the style attribute stands over the DOM interfaces given to page scripts, which gain DOMTokenList replace (classList.replace) for every script document.
@task_item n=7 part=touches
The a11y CI leg and the stand checks repeated three rounds ran green on the merged tree; the new layout test target is no stand target and is not taken by the repeated stand checks, so the three-round count holds.
@task_fact n=1 slug=visual-design-verification
fact: Text being composed through an IME is drawn with an underline in the engine's own paint until it is committed, in every document, so a composition in progress is shown by more than its caret.
@task_item n=1 part=untouched
Keyboard navigation contracts and decisions (Tab order, select lists open or closed, range input keys, Cells grid, CRUD list, keyboard activation, click focus) are not reached: no key handling or focus rule changed.
@task_item n=2 part=untouched
Role mapping, ARIA state and property reads, accessible name precedence and the accessibility tree's states are not reached: no tree building, role or name rule changed.
@task_item n=3 part=untouched
The snapshot, UI graph, driver actions, actions answer, MCP results and handler coverage contracts are not reached: no driver, snapshot or agent-reader surface changed.
@task_item n=4 part=untouched
Contrast thresholds, the WCAG ratio helper, the lint contrast rule, stand accent colour and select list colours are not reached: no colour, threshold or ratio changed.
@task_item n=5 part=untouched
The accessibility feature defaults, AccessKit and accesskit_xplat choices, and the no-platform-adapter stand decision are not reached: no feature flag, crate or version moved, Dioxus held.
@task_item n=6 part=untouched
Cognitive accessibility rows (invalid state not by colour or class alone, the shared focus order table) are not reached.
@task_item n=7 part=untouched
Password masking and never-revealed password rules are not reached: the composition underline paints no characters and reveals no value.
