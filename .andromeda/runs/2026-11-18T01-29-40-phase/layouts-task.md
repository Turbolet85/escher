@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages the escher-session binary serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order, SC 1.4.3 Contrast (Minimum).
Re-reads the existing stand keyboard and stand contrast checks and the a11y fixtures, and extends them where a 0.3.0 surface is unasserted: the engine-painted select list, the multiple select's keys and row clicks, checkbox and radio change, optgroup, inert and disabled controls, and text read with the bundled DejaVu Sans fonts.
Every re-assertion runs in the a11y CI leg, and a stand-facing one also in stand-repeat; it pins as a known SC 2.4.3 gap that a focused control hidden by hidden, display:none or CSS alone keeps focus and takes keys.
It widens no engine behaviour and adds no driver verb, carrier, argument, field, log field, span or crate dependency.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand keyboard check — every stand control reached and operated by keys alone, Tab and Shift+Tab walking each screen's stated sequence and wrapping — edits: its assertions, extended where a 0.3.0 surface is unasserted
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand contrast check — every text/background pair the stand paints meets 4.5:1 (3:1 large text) — edits: its assertions, extended to the 0.3.0 surfaces and the bundled fonts
@task_file n=3 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select list fixture — the engine-painted open list, opened by click, Enter or Space, Tab choosing and closing before focus moves
@task_file n=4 path=tests/blitz-tests/tests/select_popup_painted.rs mark=edits
what: the painted select list fixture — the open list's border, highlight and disabled-text colours
@task_file n=5 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction fixture — the multiple select's keys and row clicks, size and optgroup
@task_file n=6 path=tests/blitz-tests/tests/inert_subtree.rs mark=edits
what: the inert subtree fixture — inert and disabled controls leaving the focus order
@task_file n=7 path=tests/blitz-tests/tests/accessibility_states.rs mark=re-reads
what: the a11y states fixture — names, roles and states read from attributes
@task_file n=8 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=re-reads
what: the a11y hidden fixture — hidden content left out of the accessibility tree
@task_file n=9 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner — the a11y leg and the stand-repeat leg — edits: the a11y leg's --test list, the stand-repeat named list if a new non-stand_ witness joins it
@task_file n=10 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow pins — the legs' pins over the a11y leg's test list
@task_file n=11 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pins — every test target placed in a leg
@task_item n=1 part=needs
Every stand control keyed: each control an agent or a person can act on is reached and operated by the keyboard alone, the 0.3.0 controls included (SC 2.1.1).
@task_item n=2 part=needs
The CRUD list is one keyboard region: the focus-order walk of the CRUD screen and the multiple select's keys read it as one stop.
@task_item n=3 part=needs
The open select list: an engine-painted list over the page whose rows read their own bounds; opened by click, Enter or Space; Tab chooses and closes it before focus moves; its border, highlight and disabled-text colours are the engine's own constants and are read for contrast.
@task_item n=4 part=needs
The Flight type select reads its two options at boot; its open list and keys are part of the Flight Booker screen's keyboard and focus-order re-assertion.
@task_item n=5 part=needs
Focus order after 0.3.0 interactions: focus moves to a meaningful place when an open list closes, after a multiple select's keys, and when a control turns disabled, hidden or inert (SC 2.4.3).
@task_item n=6 part=needs
The known SC 2.4.3 gap is pinned as it stands, not removed: a focused control that no longer shows by hidden, display:none or CSS alone keeps focus and takes keys, the focus fixup reading inert alone; removing it would change engine behaviour for every document.
@task_item n=7 part=needs
Stand contrast: every text/background pair the stand paints, the stand accent colour among them, meets 4.5:1, or 3:1 for large text, read with the bundled DejaVu Sans fonts and system fonts off (SC 1.4.3).
@task_item n=8 part=needs
A run's fonts come from the stand's bundled fonts; a run without them is refused as fonts-unavailable, so the contrast reader's sampled text and the large-text threshold are measured on those glyphs.
@task_item n=9 part=needs
The headless stand runs at its pinned viewport; contrast sampling and focus walks read the stand at that size.
@task_item n=10 part=needs
The stand names its controls, roles and states by attributes alone, and its ids are attributes alone; the checks reach each control by its stable element id.
@task_item n=11 part=needs
The six served tasks of escher-session — Counter, Flight Booker, Timer, CRUD, Cells and the load page — are the served pages the re-assertion covers; which of them the existing checks leave out is closed at P3.
@task_item n=12 part=needs
Session roles share the stand boot, and a held instance reads as a fresh boot: a served page's re-assertion reads the same stand a fresh boot would.
@task_item n=13 part=needs
Into-view scroll moves every holding scroller and the visible region is read through clipping boxes: a control reached by Tab is brought into view before it is read.
@task_item n=14 part=needs
A failing re-assertion at HEAD is fixed only inside an already ratified engine behaviour; a fix that changes engine behaviour for every document with no ruling behind it goes to the founder at P4.
@task_item n=15 part=needs
No driver verb, carrier, argument, field, log field or span is added: the checks read the stand through the escher-session answers that already exist.
@task_item n=16 part=needs
The stand lints clean at boot; a fixture or stand edit the re-assertion makes keeps it so.
@task_standing n=1
convention: a new stand-facing witness reaches stand controls only by their stable element ids, never by position or text
home: surface-desktop-native
@task_standing n=2
convention: a new stand check boots the headless stand at its pinned viewport and reads a held instance as a fresh boot
home: surface-desktop-native
@task_standing n=3
convention: a new served-page check drives the stand through the existing escher-session actions and answers, under its exit codes and refusals
home: surface-cli
@task_item n=1 part=not_needed
The window chrome controls of the shell provider (drag, maximize, minimize, decorations, close): the re-assertion runs headless and touches no window chrome.
@task_item n=2 part=not_needed
The cold-agent and agent-run test pipes and their usage, exit codes, stdout and record directories: the chunk adds no agent run.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines and their arguments and environment knobs: benches, not accessibility checks.
@task_item n=4 part=not_needed
The MCP server's transport, protocol revisions, tools list and result bounds: no tool or verb is added.
@task_item n=5 part=not_needed
The session recording and its check files, and the coverage role's answer: no recording or coverage behaviour changes.
@task_item n=6 part=not_needed
Arbitrary index.html templates for a document, and the load page's network provider: the re-assertion adds no template and no network path.
@task_item n=7 part=not_needed
The windowed assistive-technology witness: it stays the founder's residual, out of scope.
