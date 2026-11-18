@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria on the 7GUIs stand and the pages the escher-session binary serves, at HEAD, after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order, SC 1.4.3 Contrast (Minimum).
Re-reads the existing stand keyboard and stand contrast checks and the a11y fixtures, and extends them where a 0.3.0 surface (engine debts, the engine-painted select list, bundled DejaVu Sans fonts, inert, disabled by presence) is unasserted.
Every re-assertion runs in the a11y leg of the CI leg script, and a stand-facing one also inside stand-repeat; the leg pins in the CI script tests follow any new member.
Pins the known SC 2.4.3 gap (a focused control hidden by hidden, display:none or CSS alone keeps focus and takes keys, the focus fixup reading inert alone) as it stands, not removed; widens no engine behaviour and adds no driver verb, field, log field, span or crate.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand keyboard check — SC 2.1.1 and SC 2.4.3 on the stand's screens: every control reached and operated by keyboard alone, Tab and Shift+Tab walking each screen's stated sequence and wrapping — edits: its assertions, extended for the 0.3.0 surfaces and the served pages it leaves out, and a check pinning the hidden-control focus gap
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand contrast check — SC 1.4.3, every text/background pair the stand paints read by the harness's text contrast reading against the required ratio (4.5:1, 3:1 for large text) — edits: its assertions, extended for the 0.3.0 surfaces under the bundled font
@task_file n=3 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: an a11y fixture — the engine-painted select list: opened by click, Enter or Space, its own key match, Tab choosing and closing before focus moves
@task_file n=4 path=tests/blitz-tests/tests/select_popup_painted.rs mark=edits
what: an a11y fixture — the select list painted by the scene painter: its border, highlight and disabled-text colours
@task_file n=5 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: an a11y fixture — select interaction: the multiple select's keys and row clicks, size on a multiple select, optgroup
@task_file n=6 path=tests/blitz-tests/tests/inert_subtree.rs mark=edits
what: an a11y fixture — an inert subtree: focus and keys kept out of inert content
@task_file n=7 path=tests/blitz-tests/tests/accessibility_states.rs mark=edits
what: an a11y fixture — accessibility states of controls (checked, disabled, expanded)
@task_file n=8 path=tests/blitz-tests/tests/accessibility_merged_states.rs mark=edits
what: an a11y fixture — merged accessibility states
@task_file n=9 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=edits
what: an a11y fixture — hidden content left out of the accessibility tree
@task_file n=10 path=tests/blitz-tests/tests/accessibility_roles.rs mark=edits
what: an a11y fixture — accessibility roles of controls
@task_file n=11 path=tests/blitz-tests/tests/accessibility_names.rs mark=edits
what: an a11y fixture — accessible names
@task_file n=12 path=tests/blitz-tests/tests/accessibility_label.rs mark=edits
what: an a11y fixture — label association
@task_file n=13 path=tests/blitz-tests/tests/accessibility_description.rs mark=edits
what: an a11y fixture — accessible descriptions
@task_file n=14 path=tests/blitz-tests/tests/accessibility_active_descendant_unread.rs mark=edits
what: an a11y fixture — active descendant left unread
@task_file n=15 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script, every CI leg run through it — edits: the a11y leg's --test list
@task_file n=16 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI Python script tests pinning the legs and the workflow jobs — edits: the a11y leg's pinned membership and the stand-repeat named list's pin
@task_file n=17 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the CI Python script tests pinning the blitz-tests targets to their legs — edits: the legs' pins for a joining or extended target
@task_file n=18 path=.github/workflows/ci.yml mark=edits
what: the CI workflow — edits: the stand-repeat job's named list, only if a new non-stand_ witness joins it
@task_item n=1 part=needs
SC 2.1.1 Keyboard re-asserted: every control an agent or a person can act on on the stand and its served pages is reached and operated by the keyboard alone, the 0.3.0 controls included (checkbox and radio change, the multiple select, the engine-painted select list).
@task_item n=2 part=needs
SC 2.4.3 Focus Order re-asserted: Tab and Shift+Tab walk each screen's stated sequence and wrap, and focus moves to a meaningful place after an open select list closes, a multiple select's keys, and a control turning disabled, hidden or inert.
@task_item n=3 part=needs
SC 1.4.3 Contrast (Minimum) re-asserted: every text/background pair the stand paints meets 4.5:1, 3:1 for large text, read under the bundled DejaVu Sans fonts with system fonts off, the select list's border, highlight and disabled-text colours and the chosen-row colour pair included.
@task_item n=4 part=needs
The contrast reading: one contrast formula, the WCAG 2.x ratio, the harness's text_contrast and required_ratio, the large-text threshold read from the bundled font's metrics; the contrast reader changes nothing it reads, and a pair with no reading is handled as the reading's contract states.
@task_item n=5 part=needs
The engine-painted select list: opened by click, Enter or Space, its own key match, Tab choosing and closing before focus moves, painted by the scene painter alone, its open pseudo-class following the open attribute.
@task_item n=6 part=needs
The engine debts' surfaces: checkbox and radio change events coming from the engine alone, the multiple select's keys and row clicks, size on a multiple select, optgroup.
@task_item n=7 part=needs
inert and disabled by presence on a form control: focus and keys kept out of inert content, a disabled control left out of the keyboard walk.
@task_item n=8 part=needs
The known SC 2.4.3 gap pinned as it stands: a focused control that no longer shows by hidden, display:none or CSS alone keeps focus and takes keys, the focus fixup reading inert alone; the check reads it and names it as a known gap, never removes it.
@task_item n=9 part=needs
The served pages: the six tasks the escher-session binary serves (Counter, Flight Booker, Timer, CRUD, Cells and the load page) booted through the stand's boot functions (stand::boot, boot_cells, boot_load, boot_timer) and their options; which of them the keyboard and contrast checks leave out is closed at P3.
@task_item n=10 part=needs
The harness surfaces the checks drive: key events and input helpers, the focused-element inspection, layout rect and center_of, settle and the virtual clock advanced by ticks for the timer.
@task_item n=11 part=needs
Every re-assertion runs in the a11y leg of the CI leg script, joining the leg's membership; a file that joins keeps its other legs, and the leg pins in the CI Python script tests are updated to match.
@task_item n=12 part=needs
Every stand-facing re-assertion is witnessed inside stand-repeat, run three rounds with a zero-failure budget; a new non-stand_ witness joins its named list and the pin that holds that list.
@task_item n=13 part=needs
Stand checks run on the bundled font with no skip, and in both layout modes.
@task_item n=14 part=needs
A failing re-assertion at HEAD is fixed only inside an already ratified engine behaviour; a fix that changes engine behaviour for every document is left to the founder at P4, never taken as a lean.
@task_item n=15 part=needs
No driver verb, carrier, argument, field, log field or span is added, and no crate dependency.
@task_item n=16 part=needs
The extended checks are controlled red-before-green: each new assertion shown to fail on a broken control before it passes.
@task_item n=17 part=needs
The windowed assistive-technology witness stays out of scope, held by its residual; no check launches a window or reads a platform accessibility API.
@task_standing n=1
convention: each new integration test file is one test target of its own in the blitz-tests manifest, shared stand tables and fixtures in their shared module, no shared module a target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new test pins its expected values as literals in code, its data stated in code, nothing seeded, a table-driven test asserting its row count
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: each new check reads no wall time and sleeps never, driving time by the virtual clock alone
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=4
convention: each new regression test is named for its bug and documents it
home: test-strategy · 2-test-strategy
@task_standing n=5
convention: each new test failure prints no argument value and no typed text
home: unit-test-strategy · 4-unit-test-strategy
@task_item n=1 part=not_needed
The cold-agent pipe: its events, verdict, exit codes, isolation and wrong-call bar; the chunk runs no agent.
@task_item n=2 part=not_needed
The WPT runner, its CLI, statuses, outputs, pinned commit and reftests; the chunk runs no conformance suite.
@task_item n=3 part=not_needed
The session wire, session state directory, session errors and idle expiry; the chunk drives the stand in process and adds nothing to the session protocol.
@task_item n=4 part=not_needed
The MCP role: tools/list, protocol revision, tool results and parity with the command line.
@task_item n=5 part=not_needed
The agent-run verbs, selections, events and exit codes.
@task_item n=6 part=not_needed
Recorded checks and their recording script, replay and handler coverage by replay.
@task_item n=7 part=not_needed
The coverage-floor and apt-install scripts' exit codes and the coverage report artifact.
@task_item n=8 part=not_needed
Mutation testing runs and baselines on the dev host, and the cognitive-complexity ceiling.
@task_item n=9 part=not_needed
The driver's snapshot text budget, typed-password masking and owner-only session files.
@task_item n=10 part=not_needed
The network stand-ins and the load page's fetching providers counted by the harness, beyond booting the load page.
@task_item n=11 part=not_needed
The upstream buffered-render and mounted-flush tests, and the Dioxus 0.7 bridge render.
