# Classified — arch · existing-scopes.b2

## Answers
existing-scopes#1.26 · no · the driver's session, verbs and executor are the Not-needed driver surface; no reached file lies in packages/escher-driver and the chunk changes none of it
existing-scopes#1.27 · no · the browser app is Not needed; no reached file lies in apps/browser
existing-scopes#1.28 · no · browser-persistence's HistoryStore sits in apps/browser, which is Not needed; nothing reached there
existing-scopes#1.29 · no · rdme in apps/readme is Not needed; no reached file lies there
existing-scopes#1.30 · no · the WPT runner is Not needed and no reached file lies in wpt/runner
existing-scopes#1.31 · yes · the blitz-tests unit holds stand_keyboard, stand_contrast, inert_subtree, select_popup and select_popup_painted, the a11y leg and the one-file-one-target rule the chunk re-asserts, and its focus, hit and contrast rules meet the engine-debts and select-list merge surfaces
existing-scopes#1.32 · no · the seven_guis unit's escher-session binary, its integration tests and its stand boot are not reached, and the Tasks list it names lives in tasks/mod.rs, not in the reached cells.rs
existing-scopes#1.33 · no · the counter, todomvc, transparent, wasm_hello and wgpu_texture example crates are not reached
existing-scopes#1.34 · no · the root examples named here are not reached, and the examples' command lines are Not needed
existing-scopes#1.35 · no · the HTML fixture pages under examples/assets are not reached, and the google and graphite forms are Not needed
existing-scopes#2.1 · yes · stand_cells_served's 47,949-byte, 740-line screen and its Cells values, and stand_load_page's checks, are stand checks the chunk re-asserts and re-measures in both layout modes
existing-scopes#3.1 · no · the escher-driver's cli and recording unit-test children sit in the driver, which is not reached
existing-scopes#3.2 · no · standing S3 · Shared fixture pages, boot and hold live once in tests/blitz-tests/tests/fixtures/mod.rs
existing-scopes#3.3 · no · the inside_stand_repeat predicate in stand_requirement_sweep reads the stand-repeat leg's membership, which the chunk does not change; its file is not reached
existing-scopes#3.4 · yes · the runtime child-module tree and the BOOTSTRAP_JS concatenation list in bootstrap.rs (lines 12-36), a conflict surface, and events.rs bind the document_may_load gate and report_js_error
existing-scopes#3.5 · yes · pseudo_element_flow_classification.rs is a whole-file conflict surface, and its all.rs mod line is the upstream test-file rule the take applies
existing-scopes#4.1 · yes · visible_region_transformed, hit_non_invertible_transform and checkable_change_event are the engine-debts merge surface the take re-reads row by row (visible_region through transforms, non-invertible no-hit, checkbox and radio change)
