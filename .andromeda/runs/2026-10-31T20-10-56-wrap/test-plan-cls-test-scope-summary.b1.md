# Classified — test-plan · test-scope-summary.b1

## Answers

test-scope-summary#1.1 · no · the Python test count (108 in eight files) is unmoved since the session adds no Python test; test_ci_workflows.py's cited ranges (:218-235, :383-497) sit outside the edited assertion at :367 and no shift is reported there, ci-leg.sh:27 lies above the added 59-60, and the capabilities pin reads the edited docs/capabilities.md with its counts unmoved
test-scope-summary#2.1 · yes · stale · the session adds one test each to cli_commands, host_log and mcp_log, so the counts stated here (cli_commands 13, host_log 4, mcp_log 2) each move up by one
test-scope-summary#2.2 · no · apps/browser is unreached and untouched, so its tab test and its citation stand
test-scope-summary#2.3 · yes · stale · document.rs gains added ranges above its test lines (761-1089 among them), so the cited document.rs:3543 and :3652-3660 move; the test counts themselves are not moved
test-scope-summary#2.4 · yes · contradict · the session paints the list, opens it on Enter and Space (the renamed select_interaction test no longer reads open nothing), reads a multiple select's size and paints group labels, and select_popup asserts the label click, so the limits stated here are no longer true
test-scope-summary#3.1 · no · node/node.rs is unedited and hit_non_invertible_transform is unreached, so the non-invertible-transform claim stands
test-scope-summary#4.1 · yes · add-to · F2 joins its multiple and group checks: select_popup, 25 tests in both layout modes, reads the list's engine behaviour with size on a multiple select and its groups among them
test-scope-summary#5.1 · yes · stale · default.css is reached whole by this session and the report does not show the chosen-row pair's rule at default.css:161-199 unmoved, and the select_interaction test at :707-728 is reached too
test-scope-summary#6.1 · yes · add-to · F7 and F6 join its unreached list: the list's unmeasured limits (windowed list unrun, assistive reading unrunnable-here, highlight contrast unread, highlight absent from snapshot and tree, thumb not draggable, larger font clipping) and select_popup's skipped key press, whose hidden-ancestor panic no check reads
test-scope-summary#7.1 · no · none of the files it names (layout/list.rs, layout/text_transform.rs, layout/writing_mode.rs, scrolled_box_client_rect.rs) is reached, and the session adds no layout test
test-scope-summary#7.2 · no · pseudo_element_flow_classification and its layout files are unreached and unchanged
test-scope-summary#7.3 · no · scrollbar.rs, element.rs, node.rs, range.rs and select.rs are unreached, and node/mod.rs, though reached, is named by none of its claims
test-scope-summary#7.4 · no · stylo.rs is reached but its change lands at 1124-1139, below the cited :510, and the :open files element.rs and mutator.rs are unreached
test-scope-summary#8.1 · no · open_state.rs is unreached and its :open checks are not changed
test-scope-summary#9.1 · no · clock.rs, iframe.rs and the clock checks are unreached and unchanged
test-scope-summary#10.1 · yes · add-to · F3 joins its frame-read paint checks: select_popup_painted, 6 tests reading the list on frames, with blitz-paint's render/select_list.rs holding no unit test of its own; render.rs:969-1052 also moves by the added 7 and 246-247
test-scope-summary#11.1 · no · blitz-test-harness is unreached and its API and test counts are unedited
test-scope-summary#12.1 · no · escher-telemetry is unreached; no log field or span was added and its sink is unedited
test-scope-summary#12.2 · yes · add-to · F4 joins its stand_act_* files as stand_act_popup, 12 tests driving the list through a held session, and the count of fifteen stand_act_* files stated here moves to sixteen with it
test-scope-summary#13.1 · no · blitz-vibey-script is unreached and no file of it changed
