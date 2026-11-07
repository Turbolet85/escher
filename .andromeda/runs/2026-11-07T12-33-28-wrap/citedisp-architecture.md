@disposition n=1
row: held fct-rekexd3u examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_cont…»
disposition: holds — stands (lines 41-51 still fail with `.unwrap()` on the file read and the reqwest fetch)
@disposition n=2
row: held fct-m2wugvlj .github/workflows/publish-browser.yml:122-126 — refused at 2026-10-27T10-45-32-wrap, not touched since «- name: Ins…»
disposition: holds — stands (lines 122-126 are the "Install Dioxus CLI" step that provides the `dx` the bundle step at 152-157 runs)
@disposition n=3
row: stretched fct-klon5sda packages/dioxus-native-dom/src/dioxus_document.rs:160-168 → 164-175 — changed inside
disposition: holds (`initial_build` at 164-175 replays the buffered mutations and then calls `flush_queued_mounted_events`)
@disposition n=4
row: changed fct-apdputpm packages/dioxus-native-dom/src/dioxus_document.rs:295-297 → ? — the end rewritten «let mut inner …»
disposition: claim false — raised at Validate (`DioxusDocument::poll` no longer holds the document's `RefCell` across `render_immediate`: lines 302-309 render into `buffered_mutations` first and only then take `self.inner.borrow_mut()` to replay them, so the hazard described is fixed, not standing)
@disposition n=5
row: stretched fct-kaaobbkf packages/dioxus-native-dom/src/lib.rs:15-28 → 15-29 — changed inside
disposition: claim false — raised at Validate (the module list omits `mod mutation_buffer;`, now declared at lib.rs:21)
@disposition n=6
row: stretched fct-ktfizprz tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-607 → 198-631 — changed inside
disposition: claim false — raised at Validate (no test is `should_panic` any more: the MountedData tests at 606-631 assert the fix, with two spawned-task tests, one per mode, and one synchronous test looping both modes, so the hazard is not pinned as two `should_panic` tests)
@disposition n=7
row: stretched fct-6qyfixyh packages/dioxus-native-dom/src/dioxus_document.rs:259-331 → 266-341 — changed inside
disposition: holds (`impl Document for DioxusDocument` spans 266-341)
