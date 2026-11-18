@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages escher-session serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order and SC 1.4.3 Contrast (Minimum).
The 0.3.0 changes it re-reads against: the engine debts (checkbox and radio change, a multiple select's keys and row clicks, size, optgroup), the engine-painted select list, the bundled DejaVu Sans fonts with system fonts off, the upstream syncs and Taffy moves, inert and disabled by presence.
Every re-assertion is a check in the a11y leg of the CI legs script, and a stand-facing one also runs in stand-repeat; the existing keyboard and contrast checks and the a11y fixtures are re-read and extended where a 0.3.0 surface is unasserted.
It pins the carried limit that a focused control no longer shown (hidden, display:none or CSS) keeps focus and takes keys, as a known SC 2.4.3 gap, without removing it.
It widens no engine behaviour, adds no driver verb, carrier, argument, field, log field or span, and no crate dependency.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand keyboard check — SC 2.1.1 and SC 2.4.3 asserted on the stand's screens, Tab and Shift+Tab walking each screen's stated sequence and wrapping
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand contrast check — SC 1.4.3, every text/background pair the stand paints read for 4.5:1, 3:1 for large text
@task_file n=3 path=tests/blitz-tests/tests/inert_subtree.rs mark=edits
what: the inert subtree fixture — focus and keys skip an inert subtree
@task_file n=4 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction fixture — a select's keys, row clicks and choosing
@task_file n=5 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select list fixture — the engine-painted list over the page opened by click, Enter or Space, its key match, Tab choosing and closing before focus moves
@task_file n=6 path=tests/blitz-tests/tests/select_popup_painted.rs mark=edits
what: the painted select list fixture — the list's border, highlight and disabled-text colours as painted
@task_file n=7 path=tests/blitz-tests/tests/accessibility_states.rs mark=edits
what: an accessibility tree fixture — the states a control reports (disabled, checked, expanded)
@task_file n=8 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=edits
what: an accessibility tree fixture — hidden content left out of the tree
@task_file n=9 path=tests/blitz-tests/tests/accessibility_roles.rs mark=edits
what: an accessibility tree fixture — the roles controls report
@task_file n=10 path=tests/blitz-tests/tests/accessibility_names.rs mark=edits
what: an accessibility tree fixture — the accessible names controls report
@task_file n=11 path=tests/blitz-tests/tests/accessibility_label.rs mark=edits
what: an accessibility tree fixture — labels naming their controls
@task_file n=12 path=tests/blitz-tests/tests/accessibility_description.rs mark=edits
what: an accessibility tree fixture — accessible descriptions
@task_file n=13 path=tests/blitz-tests/tests/accessibility_merged_states.rs mark=edits
what: an accessibility tree fixture — states merged onto one node
@task_file n=14 path=tests/blitz-tests/tests/accessibility_active_descendant_unread.rs mark=edits
what: an accessibility tree fixture — an active descendant left unread
@task_file n=15 path=.github/scripts/ci-leg.sh mark=edits
what: the CI legs script, the a11y leg and the stand-repeat leg that runs stand checks three rounds — edits: the a11y leg's --test list, the stand-repeat named list if a new non-stand_ witness joins it
@task_file n=16 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow pins — the legs' test lists pinned
@task_file n=17 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pins — every test target placed in a leg
@task_item n=1 part=needs
SC 1.4.3 contrast on every text/background pair the stand paints, 4.5:1 and 3:1 for large text, the 0.3.0 surfaces included: the open select list's border, highlight and disabled-text colours, which are engine constants hardcoded in the engine, not tokens.
@task_item n=2 part=needs
The select list's chosen row painted with Chromium's inactive selected pair — a text/background pair the contrast re-assertion reads.
@task_item n=3 part=needs
Bundled DejaVu Sans fonts with system fonts off for a run: glyph metrics may have moved, so the large-text threshold and the text the contrast reader samples are read under the bundled fonts, not the system fonts tests were built with; a family the stand asks for that is unavailable falls back to the default font.
@task_item n=4 part=needs
Screenshots the contrast reader samples never paint with text skipped, and the screenshot's fonts-unavailable refusal is what a run with no usable font answers instead.
@task_item n=5 part=needs
SC 2.1.1 keyboard: every control an agent or a person can act on is reached and operated by keys alone, the single select included — Click, Enter or Space opens it, the open list's own key match moves the highlight, Tab chooses and closes before focus moves.
@task_item n=6 part=needs
A single select's list events the keyboard checks read: open, highlight move, choose, choose the chosen option, choose a disabled row, close, close on its select's change; and wheel and key scroll of the open list, which opens and closes with no motion.
@task_item n=7 part=needs
Selects that open no list — a multiple select and a select with size — operated by their own keys and row clicks, with optgroup, as a keyboard and focus-order surface of 0.3.0.
@task_item n=8 part=needs
SC 2.4.3 focus order: Tab and Shift+Tab walk each screen's stated sequence and wrap, and focus moves to a meaningful place after an open select list closes, after a multiple select's keys, and when a control turns disabled (by presence alone), hidden or inert.
@task_item n=9 part=needs
The carried limit pinned, not removed: a focused control that no longer shows, by hidden, display:none or CSS alone, keeps focus and takes keys (the focus fixup reads inertness alone); a check reads it as it stands and names it as a known SC 2.4.3 gap — the same reading as a focused select with no box keeping focus and keys.
@task_item n=10 part=needs
The open select list as an engine overlay: painted after the root render, placed inside the viewport and answering hits first — the place where focus, hit and contrast checks find it over the page.
@task_item n=11 part=needs
The snapshot and accessibility tree of a select — open select, open attribute, highlighted row, chosen option — and the snapshot's bounds of an open list's row, as the a11y fixtures read them.
@task_item n=12 part=needs
The served pages: the escher-session tasks counter, flight-booker, timer, crud, cells and load are the pages the re-assertions cover; which of them the existing keyboard and contrast checks leave out is closed at P3.
@task_item n=13 part=needs
The checks drive the stand only through the existing driver verbs (click, press, type, advance) on the one virtual clock; no driver verb, carrier, argument, answer field, log field or span is added.
@task_item n=14 part=needs
A failing re-assertion is fixed in this chunk only inside an already ratified engine behaviour; a fix changing engine behaviour for every document with no ruling is the founder's fork at P4, never a lean.
@task_item n=15 part=needs
Every re-assertion runs in the a11y leg, and a stand-facing one also in stand-repeat, the stand checks repeated three rounds; no new crate dependency comes with them.
@task_standing n=1
convention: each new stand check moves time only through the driver's advance on the one virtual clock, with settle and load waits moving no clock and driver scroll instant
home: motion-calibrated-to-expression-level-unrecorded · motion
@task_standing n=2
convention: each new check that reads painted text reads it under the test engine's font setup, with a missing font answered by the screenshot's fonts-unavailable refusal, never a screenshot with text skipped
home: typography
@task_item n=1 part=not_needed
WPT runner output and its result colours (PASS, FAIL, CRASH, SKIP, TIMEOUT) and its secondary text — terminal output of the WPT runner, not text the stand paints.
@task_item n=2 part=not_needed
Shell provider window actions (maximize, minimize, window drag, decorations toggle), compositing alpha mode, cursor icons and the favicon — window chrome the re-assertions do not read.
@task_item n=3 part=not_needed
escher-session exit statuses and exit codes, stderr session and usage errors, the mcp role's results and the help answer — the command line's framing, untouched by a re-assertion.
@task_item n=4 part=not_needed
The actions, coverage and UI graph lines and their command arguments and answers — agent read surfaces whose content this chunk does not change.
@task_item n=5 part=not_needed
The JS window object's sizes and devicePixelRatio, the logical viewport from scale, and MountedData calls during a render — no script or viewport sizing is touched.
@task_item n=6 part=not_needed
Presentational hints in layout: body right and bottom margins, table cellspacing and cellpadding, font element face and size, lang setting the text locale, root-relative units.
@task_item n=7 part=not_needed
Color-scheme change recascade, the system color scheme, and the dioxus-native base_color configuration — the stand's scheme is not changed.
@task_item n=8 part=not_needed
The WASM pages in a canvas and the bundled font context on WASM — the re-assertions run on the native stand and served pages, not the web pages.
@task_item n=9 part=not_needed
Overlay scrollbar fade timings, thumb drag, smooth scroll-behavior with wheel cancel, and reduced-motion handling — no scrollbar or animated scroll is re-asserted.
@task_item n=10 part=not_needed
Positioned auto z-index paint levels, stacking-context grouping and hit-testing stopping at a paint clip in page content — the chunk reads the select overlay, not page stacking.
@task_item n=11 part=not_needed
Mobile targets built not tested, and the windowed assistive-technology witness, which stays an open residual out of this chunk.
