@disposition n=1
row: held fct-dp2mr6q3 .github/workflows/wpt-post-results.yml:43-49 — refused at 2026-10-10T01-56-50-wrap, not touched since «- name: Pos…»
disposition: holds — stands (lines 43-49 are still the «Post results to PR description» step)
@disposition n=2
row: moved fct-kgita7a3 dioxus_document.rs:295 → 305 — listed, not written: suffix
disposition: holds — re-pointed by hand to 305 (the inner.borrow_mut() line, now taken after render_immediate returns)
@disposition n=3
row: changed fct-kgita7a3 dioxus_document.rs:297 → ? — rewritten · naked: bound by position «self.vdom.render_immediate(&mu…»
disposition: holds — re-pointed by hand to 304 (render_immediate now renders into buffered_mutations before the borrow, so the fact's held-across claim needs this wrap's amendment)
@disposition n=4
row: stretched fct-mealwlke tests/blitz-tests/tests/dioxus_runtime_probe.rs:580-584 → 606-609 — changed inside
disposition: claim false — raised at Validate (606-609 is now mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental, a plain test asserting focus acts; no should_panic pin of the panic remains)
@disposition n=5
row: stretched fct-mealwlke tests/blitz-tests/tests/dioxus_runtime_probe.rs:586-590 → 611-614 — changed inside
disposition: claim false — raised at Validate (611-614 is now mounted_data_in_a_task_a_key_handler_spawned_acts_incremental, a plain test asserting focus acts; no should_panic pin of the panic remains)
@disposition n=6
row: stretched fct-luwk6iun tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-607 → 198-631 — changed inside
disposition: claim false — raised at Validate (10 tests and 8 loops still hold, but the MountedData hazard is no longer pinned as two should_panic tests: they are the two _acts tests asserting focus moves)
