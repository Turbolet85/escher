@task_prose part=chunk
Re-asserts three WCAG 2.2 success criteria at HEAD on the 7GUIs stand and the pages escher-session serves (Counter, Flight Booker, Timer, CRUD, Cells and the load page), after the 0.3.0 engine changes: SC 2.1.1 Keyboard, SC 2.4.3 Focus Order, SC 1.4.3 Contrast (Minimum) read with the bundled DejaVu Sans fonts.
The 0.3.0 surfaces it re-reads: checkbox and radio change, the multiple select's keys, row clicks and size, optgroup, the engine-painted select list (open, key match, Tab closing, its colours), inert and disabled by presence, focus after a control turns hidden, disabled or inert.
The existing keyboard and contrast stand harnesses and the a11y fixtures are re-read and extended where a 0.3.0 surface is unasserted; every re-assertion runs in the CI a11y leg, and a stand-facing one also in stand-repeat.
It pins the known SC 2.4.3 gap as it stands (a focused control hidden by attribute, style or CSS keeps focus and takes keys) and does not remove it; no engine behaviour is widened.
No driver verb, carrier, argument, field, log field or span is added, and no crate dependency.
@task_file n=1 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard harness check, SC 2.1.1 and SC 2.4.3 over the stand's screens and served pages — edits: its assertions, extended over the 0.3.0 surfaces left unasserted
@task_file n=2 path=tests/blitz-tests/tests/stand_contrast.rs mark=edits
what: the stand's contrast harness check, SC 1.4.3 over every text and background pair the stand paints — edits: its assertions, extended over the 0.3.0 surfaces and the bundled fonts
@task_file n=3 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select list check, the engine-painted list opened by click, Enter or Space, its key match and Tab closing it before focus moves
@task_file n=4 path=tests/blitz-tests/tests/select_popup_painted.rs mark=edits
what: the select list paint check, the open list's border, highlight and disabled-text colours
@task_file n=5 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction check, the multiple select's keys, row clicks, size and optgroup
@task_file n=6 path=tests/blitz-tests/tests/inert_subtree.rs mark=edits
what: the inert subtree check, focus and keys kept out of an inert subtree
@task_file n=7 path=tests/blitz-tests/tests/accessibility_states.rs mark=re-reads
what: the accessibility states check, disabled and other control states in the accessibility tree
@task_file n=8 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=re-reads
what: the accessibility hidden check, hidden controls left out of the accessibility tree
@task_file n=9 path=tests/blitz-tests/tests/accessibility_roles.rs mark=re-reads
what: the accessibility roles check, the roles of the controls an agent or person acts on
@task_file n=10 path=tests/blitz-tests/tests/accessibility_names.rs mark=re-reads
what: the accessibility names check, the accessible names of controls
@task_file n=11 path=tests/blitz-tests/tests/accessibility_label.rs mark=re-reads
what: the accessibility label check, label association for form controls
@task_file n=12 path=tests/blitz-tests/tests/accessibility_description.rs mark=re-reads
what: the accessibility description check, accessible descriptions of controls
@task_file n=13 path=tests/blitz-tests/tests/accessibility_merged_states.rs mark=re-reads
what: the accessibility merged states check, states merged onto one accessibility node
@task_file n=14 path=tests/blitz-tests/tests/accessibility_active_descendant_unread.rs mark=re-reads
what: the accessibility active descendant check, the active descendant left unread
@task_file n=15 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script, the a11y test leg and the stand-repeat run of stand checks over three rounds — edits: the a11y leg's --test list, the stand-repeat named list if a new non-stand witness joins it
@task_file n=16 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow pins, the legs and their test lists as CI runs them — edits: the legs' pins
@task_file n=17 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pins, every test target placed in a CI leg — edits: the legs' pins
@task_item n=1 part=needs
The re-assertion checks themselves: SC 2.1.1, SC 2.4.3 and SC 1.4.3 asserted on the stand and its served pages, each a test in the CI a11y leg, a stand-facing one also repeated in stand-repeat's three rounds.
@task_item n=2 part=needs
The CI per-leg log uploaded on failure covers the a11y leg the re-assertions run in; a failing re-assertion is read from that leg's log artifact.
@task_item n=3 part=needs
The headless stand runs with no telemetry sink; the keyboard and contrast harnesses drive it headless and install none.
@task_item n=4 part=needs
Silent checks: the re-assertion tests print nothing beyond the test harness's own lines, and share their stand-driving code as shared modules, not as test targets.
@task_item n=5 part=needs
The engine-painted select list: the open list's shot carries its row labels only as pixels; the contrast reader samples those pixels and the keyboard check reads the open list's row bounds from the snapshot and diff, never logging a row label.
@task_item n=6 part=needs
The multiple select's chosen set and the select list row labels are never logged; a check asserting on the multiple select's value or chosen rows keeps them in its assertion, never printed.
@task_item n=7 part=needs
Optgroup label as the group's name and its painted header: the contrast re-assertion covers the painted optgroup header and the focus-order re-assertion walks past it, the header not a focus stop.
@task_item n=8 part=needs
The select size attribute read on a multiple select: the keyboard re-assertion walks a sized multiple select's rows by its keys.
@task_item n=9 part=needs
Checkbox and radio change as page script events: operating a checkbox or radio by the keyboard alone fires its change once, which SC 2.1.1 reads.
@task_item n=10 part=needs
Clickable on the snapshot node and the snapshot text: the set of controls an agent can act on, which SC 2.1.1 asserts every member of is reached and operated by keys.
@task_item n=11 part=needs
The disabled refusal cause: a control disabled by presence refuses action, and focus moves to a meaningful place when a control turns disabled, hidden or inert.
@task_item n=12 part=needs
The fonts-unavailable refusal cause and the bundled DejaVu Sans fonts with system fonts off: the contrast reader's sampled text and the large-text threshold read with those fonts.
@task_item n=13 part=needs
Decoded screen text stays in the match: text the contrast or keyboard reader decodes from the painted screen is used inside its comparison and never printed.
@task_item n=14 part=needs
The known SC 2.4.3 gap, pinned as it stands: a focused control that no longer shows by hidden, style display none or CSS keeps focus and takes keys, the focus fixup reading inert alone; a check names it as a known gap and does not remove it.
@task_item n=15 part=needs
The bound on what the chunk adds: no driver verb, carrier, argument, answer field, log field or span, and no crate dependency; a fix found at HEAD stays inside an already ratified engine behaviour.
@task_item n=16 part=needs
A check that asserts an empty stderr from a stand or session run unsets RUST_LOG for that run.
@task_item n=17 part=needs
Session logs carry no page content: a served-page re-assertion that runs through escher-session reads no page text out of its logs.
@task_standing n=1
convention: each new test is silent, printing nothing beyond the harness's lines, and its shared code lives as shared modules
home: 3-observability-harness-contract
@task_standing n=2
convention: each new test joins a CI leg whose per-leg log is uploaded on failure
home: ci-integration
@task_standing n=3
convention: each new committed fixture or check holds no secrets and no private data
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: each new test that reads a failure from logs names a kind and a count, never the logged content
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The cold-agent pipe: its verdict fields and reasons, counted verbs, events, transcript, kept checks and server log; the chunk runs no cold agent.
@task_item n=2 part=not_needed
The agent-run harness events and its raw log; the chunk adds no agent run.
@task_item n=3 part=not_needed
The driver's command span and tool_call span field contracts and the closed span line layout; the chunk adds no span and no span field.
@task_item n=4 part=not_needed
escher's sink log line format, escaping, scrub set and resource keys, and the escher-telemetry startup and panic events; the chunk adds no log field and installs no sink.
@task_item n=5 part=not_needed
The escher-session MCP role, coverage role, exit codes, error object, host wire and recording file contracts; the chunk adds no verb, carrier or argument.
@task_item n=6 part=not_needed
Script error reporting: the window error event, window.onerror, script error source labels and the embedder's drain of script diagnostics.
@task_item n=7 part=not_needed
WPT reports and score publishing, and metric emission or exporters; the chunk measures no metric.
@task_item n=8 part=not_needed
Engine tracing behind its cargo feature and the choice of the tracing crate; the chunk changes no tracing.
@task_item n=9 part=not_needed
The WASM console panic hook and the exploration's panic catch.
@task_item n=10 part=not_needed
The package-install diagnostic line in CI.
@task_item n=11 part=not_needed
Recording and session state directory privacy and the session idle expiry; the chunk adds no recording.
