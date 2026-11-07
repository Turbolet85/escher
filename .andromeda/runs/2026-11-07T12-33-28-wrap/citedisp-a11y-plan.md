@disposition n=1
row: held fct-keyqn7iz apps/browser/Cargo.toml:14 — refused at 2026-10-24T15-00-29-cites-backlog, not touched since «default = […»
disposition: holds — stands (line 14 is the default feature set, and `accessibility` is not in it)
@disposition n=2
row: changed fct-gmhtpdel packages/dioxus-native-dom/src/dioxus_document.rs:295-297 → ? — the end rewritten «let mut inner …»
disposition: claim false — raised at Validate (`DioxusDocument::poll` no longer holds `inner.borrow_mut()` across `vdom.render_immediate`: it now renders into `buffered_mutations` first and borrows `inner` only to replay them, at lines 302-307, so a task polled during render does not panic)
@disposition n=3
row: stretched fct-gmhtpdel tests/blitz-tests/tests/dioxus_runtime_probe.rs:557-607 → 556-631 — changed inside
disposition: claim false — raised at Validate (the two `should_panic` pins are gone: 556-631 now holds `spawned_focus_acts` and the tests `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `_incremental`, which assert that the spawned task's focus request moves focus, so no `RefCell already borrowed` panic is measured there)
@disposition n=4
row: held fct-ge4n5uhw tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…»
disposition: holds — stands (the module doc at 1-5 states the stand snapshot's nodes carry their accessibility node's role and name, in both layout modes)
