@amendment marker=2026-11-08-handler-coverage
title: the ungated reached record on handle_event's path
section: §2 A11y Strategy (a11y-strategy.md: the feature-gate list); §5 Keyboard Navigation (keyboard-navigation.md: the reached record beside the registry arm)
change:
  - The feature-gate list's ungated clause names, beside the `signal_registry` module, dioxus-native-dom's `reached` module (`ReachedHandler`, re-exported from the crate root, `DioxusDocument::record_reached`, `DioxusDocument::reached_handlers`) and the crate-private listener reader `element_listener_ids` the record reads, `element_facts` staying behind the `accessibility` feature (was: `signal_registry` the one ungated module named).
  - New fact: beside the registry arm, `DioxusEventHandler::handle_event` writes an ungated reached record once `record_reached` has switched it on (a crate-private `ReachedRecord` context at `ScopeId::ROOT`, a second call keeping the one record); after the event-kind and dioxus-id early returns and before `runtime.handle_event`, each chain node whose current listeners hold the event's name inserts its stable id with that name; `reached_handlers` reads them in first-reached order, deduplicated by (id, event), empty when never switched on; the id is resolved before the run, so a handler whose run removes its element still counts; every other document pays one `has_context` lookup per dispatch.
why: the report's Symbols / APIs state the reached record ungated, compiled with and without `accessibility`, written on `handle_event`'s path before `runtime.handle_event`, and `element_listener_ids` crate-private and ungated; its deviation 1 states why the record reads `element_listener_ids`, not `element_facts`: `element_facts` is accessibility-gated and the wasm build compiles dioxus-native-dom without that feature.
kept: the registry-arm fact of §5 stands as written; the record sits beside it, the registry's dispatch unchanged.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: stand_handler_coverage's keyboard-activation check
section: §5 Keyboard Navigation (keyboard-navigation.md: the stand's keyboard checks)
change:
  - New fact: `stand_handler_coverage` (8 tests) asserts in both layout modes, through `escher_driver::handler_coverage`, that a refused step exercises nothing and a keyboard activation exercises its control's click (`a_refused_step_exercises_nothing_and_a_keyboard_activation_its_click`); it runs in the `stand-repeat` leg by its `stand_` prefix and is in no `a11y` leg list.
why: the report's Harness / gate surface (a new `stand_*` target of 8 tests, picked up by `run stand` and `stand-repeat` by its prefix, no leg list changed) and its Outcome (the test green in both modes; the `a11y` leg green, its fourteen files untouched).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: handler_coverage, a further reader of the UI graph
section: §7 Screen Reader Support (screen-reader-support.md: the UI graph's readers)
change:
  - New fact: `escher_driver::handler_coverage` is one more reader of `DioxusDocument::graph`: per check it reads the graph at boot and after each replayed step, `mounted` left out, deduplicated by (id, event), and lists as unexercised the graph's handlers no check reached; its answer holds ids, owner ids, event names and two counts (`checks`, `held`) only, no role or name among them.
why: the report's Symbols / APIs (`handler_coverage` reading the graph at boot and after each step, `mounted` left out) and its Coverage of new surfaces (ids and event names only; a11y n/a).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
