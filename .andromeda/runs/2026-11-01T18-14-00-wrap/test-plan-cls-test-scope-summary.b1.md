# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · yes · stale · test_ci_workflows.py gains two tests, so the CI scripts count moves 108 to 110 and the ci-leg.sh:27 citation moves with the lines added to ci-leg.sh
test-scope-summary#2.1 · no · no file under apps/ is reached and examples/seven_guis has no diff, so the no-tests claim and its search still hold
test-scope-summary#2.2 · no · apps/browser/src/tab.rs is not reached and no file under apps/ was reached
test-scope-summary#2.3 · yes · stale · document.rs, mutator.rs and net.rs are merge surfaces, mutator.rs gains upstream's 39 restyle tests and document.rs gains the picture and inline-flag tests, so the 38 count and the cited lines move
test-scope-summary#2.4 · yes · add-to · F5: is_inline_with_only_block_children and its paint skip have no frame test and are held by 4 unit tests in document.rs, which joins this list of covered behaviours
test-scope-summary#3.1 · yes · stale · node.rs is a merge surface reached whole, so the cited hit_inner lines 1485-1490 may have moved
test-scope-summary#4.1 · no · the select_interaction target and node/select.rs are not reached and the take names no select behaviour change
test-scope-summary#5.1 · yes · stale · default.css is a merge surface and the UA sheet's picture rule changed, so the cited lines 161-199 may have moved
test-scope-summary#6.1 · no · escher-driver has no diff, so schema.rs:267-269 and the press residual still hold
test-scope-summary#7.1 · yes · contradict · the key-press panic is ended by deallocate_anonymous_block and select_popup no longer skips row 3 (one harness.press per row), so the claims that the panic is read by no check and that row 3 is skipped are false, F1 being the check that reads it
test-scope-summary#8.1 · no · the four stated limits of the windowed list name no reached file's hunk, and the row-3 change does not touch them
test-scope-summary#9.1 · no · list.rs, text_transform.rs, writing_mode.rs and scrolled_box_client_rect.rs are not reached and none is a merge surface
test-scope-summary#9.2 · yes · stale · classify_flow_children sits in layout/construct.rs, a merge surface reached whole, so the pseudo-element voting claim is re-read
test-scope-summary#9.3 · yes · stale · element.rs and node.rs are merge surfaces, so the cited lines 1084-1218 and 2097-2183 may have moved
test-scope-summary#9.4 · yes · stale · element.rs, mutator.rs and stylo.rs are merge surfaces, so the cited :open lines 543-549, 356-359, 502-505 and 513 may have moved
test-scope-summary#10.1 · no · select_list.rs is not reached and select_popup.rs changed only its row-3 skip, so the checks still read these functions
test-scope-summary#11.1 · yes · stale · the :open pins live in element.rs and mutator.rs, merge surfaces reached whole, so the open_state claims are re-read
test-scope-summary#12.1 · no · clock.rs, iframe.rs and the virtual_clock_engine target are not reached and the entity names no reached file
test-scope-summary#13.1 · yes · add-to · F4: no upstream or fork test paints a blended element to a frame, so mix-blend-mode and isolation painting rest on the 3 layers.rs unit tests (F12), and render.rs is a merge surface whose cited password-mask lines may move
test-scope-summary#14.1 · yes · stale · the blitz-test-harness library count moves 22 to 24, with contrast.rs's blended-ancestor test and settle.rs's failed-stylesheet test
test-scope-summary#15.1 · no · escher-telemetry has no diff and no log or tracing site was added
test-scope-summary#15.2 · yes · add-to · F2: a failed load now ends the driver's wait, which joins the session.rs load-wait group beside the load answered inside the bound
