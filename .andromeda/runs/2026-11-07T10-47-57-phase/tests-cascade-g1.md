@carried n=1 label=test-plan/test-scope-summary fact=fct-luwk6iun
why: the chunk inverts the two should_panic pins this row records, so the row must read 10 tests of which the two per-layout-mode pins now assert set_focus moves focus without a panic, the other eight looping both modes unchanged.
quote: a blitz-tests target of 10 tests, 8 looping `for incremental in [false, true]` and the MountedData hazard pinned as two `should_panic` tests, one per layout mode, with no `stand_` prefix, so `run all` and `run dioxus_runtime_probe` run it and `run stand` does not
@carried n=2 label=test-plan/test-scope-summary fact=fct-i5a6vv3t
why: the ported unit test lands in this crate, so the row's count of unit tests must move by the one test ported, its file named.
quote: - **dioxus-native-dom:** 82 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)
@carried n=2 run=2 label=test-plan fact=fct-jydhmppz
quote: - **dioxus-native-dom:** unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id, the actionable-key check, the bridge, the mutation writer and the document's events
@carried n=3 label=test-plan/test-scope-summary fact=fct-i5a6vv3t
why: the bridge test over the MutationWriter pins mutation_writer.rs byte-identical to upstream at 434e0661, which the scope keeps where the port allows, a change there named with its reason.
quote: — mapped to its element id and left on the writer's stack with its text, `mutation_writer.rs` itself byte-identical to upstream's at `434e0661`
@carried n=4 label=test-plan/test-scope-summary fact=fct-e2zfwpza
why: this is the chunk's witness, so the row must come to read that the fork's dioxus_document.rs test module holds the ported task_polled_during_render_can_access_document, passing on the 0.7 bridge.
quote: upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's dioxus_document.rs test module; it is the witness of the route entry «MountedData buffered render», which ports `286874cc1`'s buffered render onto the Dioxus 0.7 bridge and inverts `dioxus_runtime_probe`'s hazard pin
@carried n=4 run=2 label=test-plan fact=fct-7exig2pt
quote: - **Workspace (CI):** `cargo test --workspace --locked` runs on ubuntu with default features as the fast `test` leg
@carried n=4 run=3 label=test-plan fact=fct-7m2wyfwg
quote: - **Native platforms:** the CI matrix runs `test --all --tests --locked` on windows and macos
@carried n=5 label=test-plan/test-scope-summary fact=fct-ybmlkzsq
why: the chunk ports 286874cc1, with which this test returns, so the plan must state whether it is ported on 0.7 and what it asserts there, or why it stays out, the flush guard keeping or gaining its test.
quote: upstream's `element_removed_before_its_mounted_event_is_flushed` is left out, because it reads 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own
@carried n=6 label=test-plan fact=fct-e36kow75
why: the chunk rewrites poll, so the buffered poll must return the answer the unbuffered one did, keeping settle's quiet-pass reading and its Render, Loads and Layout outcomes where they stand.
quote: `settle` repeats that pass and keeps poll's answer — a pass is quiet when poll answered `false`, the hover node did not change and no load finished in it
@carried n=7 label=test-plan fact=fct-65u33zgx
why: the inverted pins and every stand check reach the buffered initial_build and poll through the Dioxus-backed constructors, which pump once.
quote: from_html, from_html_with, from_component, from_vdom, wrap; constructors pump once, wrap does not (packages/blitz-test-harness/src/harness.rs:88-136)
@carried n=8 label=test-plan fact=fct-dncmjjxa
why: the inverted pins press Tab and the handler's key through these helpers, each pumping into the buffered poll, and then read focus to assert it moved.
quote: click, click_at, mouse_down_at, mouse_up_at, move_mouse_to, drag, tap, tap_at, touch_down, touch_move, touch_up, wheel_at, press, press_with, type_text, apple_keybinding, scroll_into_view, ime — each pumps after dispatch
@carried n=8 run=2 label=test-plan fact=fct-542cugs7
quote: query, node, query_all, layout_rect, layout_rect_of, center_of, text_content, attr, hit, hit_node, focused, hovered, dom_string
@carried n=9 label=test-plan fact=fct-i47d76zr
why: the buffered replay must keep a re-render's removal and mapping clearing, so these element_id tests stay green unchanged.
quote: assert that a re-render-removed node no longer resolves, reads no id and stands in no pair of `element_ids()`
@carried n=10 label=test-plan fact=fct-i47d76zr
why: a buffer living on vdom_state changes what the bridge tests construct over, so they must keep building their handler over doc.vdom and doc.vdom_state and stay green.
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep
@carried n=11 label=test-plan fact=fct-glytb3w3
why: the inverted pins change their outcome, not the count, so run all's reading must stay 743 passed, 0 failed, 12 ignored unless the chunk measures otherwise and restates it.
quote: `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change
@carried n=12 label=test-plan fact=fct-xxpbx2ay
why: the buffer holds ids, names and text in process only, so a hosted call's log lines must stay one command span line per call and no further line at any level.
quote: a host that runs calls adds one `command` span line per call from `info` up, and the workspace-built host one engine line more, `blitz_dom::document` at INFO, with no further line at `debug` or `trace` on a snapshot · click · type flow
@binding n=1
**dioxus_runtime_probe's row ↔ the agent-run proof**: the two pins inverted turn from should_panic into passing tests in both layout modes (test-plan/test-scope-summary fct-luwk6iun), so run all keeps 743 · 0 · 12 with the probe's 10 tests still counted (test-plan fct-glytb3w3).
@binding n=2
**the ported witness ↔ the dioxus-native-dom unit-test row ↔ the coverage floor**: the absent upstream test becomes present (test-plan/test-scope-summary fct-e2zfwpza), the crate's count of 82 moving by one (test-plan/test-scope-summary fct-i5a6vv3t), and the new buffer module counts toward the crate's 83 % line floor (test-plan fct-4t3fuckc).
@binding n=3
**the buffer's home ↔ the bridge tests ↔ mutation_writer.rs byte-identity**: if the buffer lives on DioxusState as upstream's does, mutation_writer.rs stops being byte-identical to 434e0661 and the bridge tests' construction over doc.vdom_state must be re-read; either way the change is named with its reason (test-plan/test-scope-summary fct-i5a6vv3t) (test-plan fct-i47d76zr).
@binding n=4
**the buffered poll ↔ the harness's pump and settle**: pump polls with no waker context and settle keeps poll's answer, so the buffered poll's answer equals the direct one's and the stand checks' settle readings do not move (test-plan fct-e36kow75) (test-plan fct-65u33zgx).
@binding n=5
**the second absent upstream test ↔ flush_queued_mounted_events unchanged**: porting 286874cc1 reopens element_removed_before_its_mounted_event_is_flushed, which reads 1 mounted event on 0.7; the plan decides it, the mounted events' order kept as the scope bounds it (test-plan/test-scope-summary fct-ybmlkzsq).
