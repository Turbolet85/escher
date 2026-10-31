# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · yes · stale · test_ci_workflows.py is reached and its stand-repeat assertion at :367 moves, so the cited install-bounding ranges in that file (:218-235, :383-497) are not shown to hold
test-scope-summary#2.1 · yes · stale · cli_commands, host_log and mcp_log each gain one test, and the host_log.rs and mcp_log.rs citations shift
test-scope-summary#2.2 · no · apps/browser is unedited and its tab.rs test is outside the session
test-scope-summary#2.3 · yes · stale · document.rs gains lines above the cited :3543 and :3652-3660 (761-1089 among them), so both citations move
test-scope-summary#2.4 · yes · contradict · a closed single select now paints a list, Enter and Space open it, a group's label paints and a multiple select's size is read, so the no-popup, opens-nothing, unpainted and size-unread limits are false
test-scope-summary#3.1 · no · its hit rule and test sit in node.rs and hit_non_invertible_transform.rs, neither reached
test-scope-summary#4.1 · yes · add-to · F2: select_popup, 25 engine tests of the list with multiple-select size and groups among them, joins this group of select checks
test-scope-summary#5.1 · yes · stale · default.css is reached whole and the cited chosen-row rule (:161-199) and its ratio test are not shown to hold on the changed file
test-scope-summary#6.1 · yes · add-to · F7: the list's unmeasured limits (windowed list not run, highlighted row contrast unread, thumb not draggable, larger font clipping) join what no driver check reaches
test-scope-summary#7.1 · no · its bounds and flow rules sit in layout/ and scrolled_box_client_rect, and no layout/ file is among the reached files
test-scope-summary#7.2 · no · its flow-classification rule lives in layout code and pseudo_element_flow_classification.rs, neither reached
test-scope-summary#7.3 · no · its cited in-file tests sit in scrollbar.rs, element.rs, node.rs, range.rs and select.rs, none reached, and node/mod.rs gains only the select_list module line
test-scope-summary#7.4 · yes · stale · stylo.rs is reached whole and the :open match at stylo.rs:510 is this entity's rule, which the report does not show holds
test-scope-summary#8.1 · yes · stale · the summary's Enter flip is a keyboard path, and events/keyboard.rs is reached whole with the Enter-on-select change
test-scope-summary#9.1 · no · its clock rules sit in clock.rs, iframe.rs and virtual_clock_engine, none reached
test-scope-summary#10.1 · yes · add-to · F3: select_popup_painted, six frame checks of the list, joins the blitz-paint inventory whose render/select_list.rs holds no unit test; the render.rs citation at :969-1052 also moves
test-scope-summary#11.1 · no · the harness settle, screenshot and contrast files are unedited and no harness API moved
test-scope-summary#12.1 · no · escher-telemetry's format.rs and telemetry_* checks are unedited, with no log field or span added
test-scope-summary#12.2 · yes · stale · the fifteen stand_act_* files become sixteen with stand_act_popup (F4), moving the count at test-scope-summary:94
test-scope-summary#13.1 · no · blitz-vibey-script is unedited and no reached file is in it
