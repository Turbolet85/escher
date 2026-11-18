@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages the escher-session binary serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order and SC 1.4.3 Contrast (Minimum).
The changes re-read are the engine debts (checkbox and radio change, multiple select keys and row clicks, size on a multiple select, optgroup), the engine-painted select list, the bundled DejaVu Sans fonts, inert and disabled by presence, and the upstream syncs and Taffy moves.
The existing stand keyboard and stand contrast checks and the a11y fixtures are re-read and extended where a 0.3.0 surface is unasserted; every re-assertion runs in the a11y CI leg, and a stand-facing one also in stand-repeat.
It widens no engine behaviour and adds no driver verb, carrier, argument, field, log field, span or crate dependency; a failure found at HEAD is fixed only inside an already ratified engine behaviour.
The known SC 2.4.3 gap carried in (a focused control hidden by hidden, display none or CSS keeps focus) is pinned as it stands and named, not removed.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard check, the SC 2.1.1 and SC 2.4.3 witness on the 7GUIs stand screens — edits: its Tab order, keyboard operation and focus assertions, extended to the 0.3.0 surfaces and served pages left unasserted
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand's contrast check, the SC 1.4.3 witness reading every text/background pair the stand paints through the harness contrast readers — edits: its pairs and thresholds, extended to the 0.3.0 surfaces read with the bundled fonts
@task_file n=3 path=tests/blitz-tests/tests/inert_subtree.rs mark=re-reads
what: the a11y fixture check of focus fixup and focusability under an inert subtree
@task_file n=4 path=tests/blitz-tests/tests/select_interaction.rs mark=re-reads
what: the a11y fixture check of select keyboard and pointer interaction, single and multiple
@task_file n=5 path=tests/blitz-tests/tests/select_popup.rs mark=re-reads
what: the a11y fixture check of the open select list's default actions, its open keys and Tab choosing and closing
@task_file n=6 path=tests/blitz-tests/tests/select_popup_painted.rs mark=re-reads
what: the a11y fixture check of the open select list painted over the page, its border, highlight and disabled-text colours
@task_file n=7 path=tests/blitz-tests/tests/accessibility_states.rs mark=re-reads
what: the a11y fixture check of the accessibility tree's ARIA and native states
@task_file n=8 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=re-reads
what: the a11y fixture check of hidden content in the accessibility tree
@task_file n=9 path=tests/blitz-tests/tests/accessibility_roles.rs mark=re-reads
what: the a11y fixture check of accessibility tree roles
@task_file n=10 path=tests/blitz-tests/tests/accessibility_names.rs mark=re-reads
what: the a11y fixture check of accessible names
@task_file n=11 path=tests/blitz-tests/tests/accessibility_label.rs mark=re-reads
what: the a11y fixture check of labels naming controls
@task_file n=12 path=tests/blitz-tests/tests/accessibility_description.rs mark=re-reads
what: the a11y fixture check of aria-describedby and aria-description
@task_file n=13 path=tests/blitz-tests/tests/accessibility_merged_states.rs mark=re-reads
what: the a11y fixture check of merged ARIA and native states
@task_file n=14 path=tests/blitz-tests/tests/accessibility_active_descendant_unread.rs mark=re-reads
what: the a11y fixture check that aria-activedescendant stays unread
@task_file n=15 path=.github/scripts/ci-leg.sh mark=edits
what: the one CI leg runner, holding the a11y leg and the stand-repeat leg — edits: the a11y leg's --test list, and the stand-repeat named list if a new non-stand_ witness joins it
@task_file n=16 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow contract tests pinning the legs of the CI leg runner — edits: the a11y leg's and stand-repeat's pins
@task_file n=17 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the tests pinning the blitz-tests targets each CI leg runs — edits: the a11y leg's and stand-repeat's target pins
@task_item n=1 part=needs
SC 2.1.1 Keyboard re-asserted: every actionable control on the stand screens and served pages is reached and operated by the keyboard alone, through the harness's key types and the Enter and Space keyboard activation of the focused control.
@task_item n=2 part=needs
SC 2.4.3 Focus Order re-asserted: Tab and Shift+Tab walk each screen's stated sequence and wrap, the Cells grid read as one Tab stop with arrow and Enter editing.
@task_item n=3 part=needs
Focus after the open select list: opening by click, Enter or Space and its own key match, Tab on an open list choosing and closing it before focus moves, and the list closing with no event, focus left in a meaningful place.
@task_item n=4 part=needs
Focus and keys on a multiple select and optgroup: the multiple select row keys (choose alone, span, move) and row clicks, size as a height hint, options including optgroup children and disabled by themselves or their group.
@task_item n=5 part=needs
Focus when a control turns disabled or inert: focusability reads disabled by presence on a form control alone, inert is honoured by presence, and the focus fixup under inert moves focus without events.
@task_item n=6 part=needs
The carried SC 2.4.3 limit: a focused control that stops showing by hidden, display none or CSS alone keeps focus and takes keys, the focus fixup reading inert alone; the check reads it as it stands and names it as a known gap, never removing it.
@task_item n=7 part=needs
Checkbox and radio change fired by the engine, operated from the keyboard on the stand, re-read for SC 2.1.1.
@task_item n=8 part=needs
SC 1.4.3 Contrast re-asserted: every text/background pair the stand paints meets 4.5:1, or 3:1 for large text, read through the test harness's text contrast and contrast ratio readers over screenshots composited over white on the CPU renderer.
@task_item n=9 part=needs
The select list's border, highlight and disabled-text colours and the multiple select chosen-row colours are the engine's own constants; they are contrast pairs the re-assertion reads, never retuned.
@task_item n=10 part=needs
Bundled fonts: the stand's typography is DejaVu Sans shared from the bundled font with system fonts off; glyph metrics, the large-text threshold and the sampled text the contrast reader reads are measured under it.
@task_item n=11 part=needs
Served pages: the six tasks the escher-session binary serves from its one session-apps table (Counter, Flight Booker, Timer, CRUD, Cells and the load page, with its delayed in-process stylesheet) are the pages the re-assertion covers; which of them the existing checks leave out is P3's to close.
@task_item n=12 part=needs
The stand's pinned environment: it boots offline, in the light scheme, its readings identical on every CI OS, so a new assertion holds on each.
@task_item n=13 part=needs
Every re-assertion runs in the a11y leg of the CI leg runner, the leg widening in place with no new job, its --test list and the legs' pins kept in step by the CI contract tests.
@task_item n=14 part=needs
A stand-facing re-assertion also runs in stand-repeat, the stand checks repeated three rounds under the zero-failure budget, a non-stand_ witness joining its named list.
@task_item n=15 part=needs
Re-asserts only: a failure found at HEAD is fixed only inside an already ratified engine behaviour; a fix changing engine behaviour for every document with no ruling is the founder's fork, never taken here.
@task_item n=16 part=needs
No driver verb, carrier, argument, answer field, log field or span is added: the checks read the stand through the harness and the existing readers alone.
@task_item n=17 part=needs
No new crate dependency is added for the checks.
@task_standing n=1
convention: each new blitz-tests test file is one test target of its own, declared in the one test table, testing one behaviour
home: conventions
@task_standing n=2
convention: each new test is named descriptively, asserts its fixture's condition first, and reaches helpers shared by stand checks through non-target modules with track-caller assertion helpers
home: conventions
@task_standing n=3
convention: each new keyboard test takes its key types through the harness's re-exports
home: standard-contracts
@task_item n=1 part=not_needed
WPT runner, WPT suites, results, reftests and the WPT CI artifacts: the chunk runs no web-platform test.
@task_item n=2 part=not_needed
The reference browser app, its history, persistence, about pages and toolbar: the chunk reaches no browser chrome.
@task_item n=3 part=not_needed
The MCP server protocol, the session socket's wire and the cold-agent run: the checks drive the stand in process, never over a carrier.
@task_item n=4 part=not_needed
Networking, the HTTP cache, script fetching and the Boa JavaScript runtime: the stand boots offline and runs no page script.
@task_item n=5 part=not_needed
CSS layout algorithms and their absences (grid, tables, writing modes, transforms, anchor positioning, container queries): the chunk asserts focus and contrast, never layout.
@task_item n=6 part=not_needed
The windowed assistive-technology witness, the AccessKit platform adapter and its window events: they stay with the founder's residual, out of scope.
@task_item n=7 part=not_needed
Release, versioning, publishing, the Nix flake and the host-run mutation tier: the chunk ships nothing and changes no package.
@task_item n=8 part=not_needed
The driver's exploration, recording, kept checks and their replay: the re-assertion adds no kept check and no recorded session.
