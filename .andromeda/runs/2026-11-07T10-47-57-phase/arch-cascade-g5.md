@carried n=1 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the buffered render must leave the stand's snapshot and ids as they read, so this stand count, re-read by the stand_* checks the scope names as witness, must hold in each layout mode.
quote: On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9
@carried n=1 run=2 label=architecture fact=fct-srb7slak
quote: `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=2 label=architecture/standard-contracts fact=fct-t7zq5feb
why: this stand_* check reads a not-settled render on a fixture that re-renders every time, so the buffered render must keep settle's `busy="render"` reading and the span's fields as they stand in both layout modes.
quote: `stand_act_spans` drives `press tab` on an in-file fixture that mounts a freshly keyed element on every render, in both layout modes, and its `command` line carries `settled=false` and `busy="render"`, no `passes` and no `cause`, six of the eight fields
@carried n=2 run=2 label=architecture fact=fct-gkkvjkmi
quote: `settle` drives render and layout to quiet, reports a load in flight and never waits on one or moves time.
@carried n=3 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope's nothing-crosses boundary requires the driver's closed sets (causes, verbs, spans and their fields) to stay as they are, the buffer adding no cause, field or logged content.
quote: `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18)
@carried n=3 run=2 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=3 run=3 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=4 label=architecture/standard-contracts fact=fct-t7zq5feb
why: stand_act_obstructed is a stand_* witness the chunk re-runs, so its covered reading must hold unchanged in both layout modes after the buffered render.
quote: a target lying beside a box clipped on one axis, along the axis it does not clip, which reads in view and is refused `covered` with the instance unchanged, in both layout modes
@carried n=5 label=architecture/standard-contracts fact=fct-6m5fkd33
why: the lint reads the DioxusDocument snapshot the buffered render builds, so that snapshot must read the same for the lint to keep its readings.
quote: it reads `harness.doc.snapshot()`, recording each node's id in the session's record of ids as `snapshot` does, and runs no step, settle, pump or time move, so it changes nothing.
@carried n=5 run=2 label=architecture fact=fct-elnqiowc
quote: `Session::lint` reads the screen and changes nothing
@carried n=6 label=architecture/standard-contracts fact=fct-6m5fkd33
why: stand_lint is a stand_* witness the chunk re-runs, so its pins must pass unchanged in both layout modes.
quote: a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same
@carried n=6 run=2 label=architecture/standard-contracts fact=fct-egxogxwi
quote: the snapshot gives a closed select's options its client rect and the lint's `overlaps` skips them, so on the stand all four lean tasks read 0 `overlap` at boot, `stand_lint` pinning `at_boot` as `[0, 0, 0, 0]` in both layout modes
@carried n=7 label=architecture/standard-contracts fact=fct-62w2i7iy
why: the load page re-renders after its fetch lands, so the buffered render must keep this stand_* reading inside the load wait in both layout modes.
quote: its `LoadNet` fetch is a counted load, so a `click` on `load-fetch` waits it out inside `LOAD_WAIT` and returns with `load-done` shown, in both layout modes
@carried n=7 run=2 label=architecture/standard-contracts fact=fct-g75i5t3p
quote: The crate-private `settle_waiting` settles the held harness through `Harness::settle` and, while the answer is `NotSettled { busy: Busy::Loads }`, settles again every `LOAD_POLL` (10 ms, private) until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed
@carried n=7 run=3 label=architecture fact=fct-elnqiowc
quote: `LOAD_WAIT` (5 s) bounds an acting step's wait for a load, moving no time.
@binding n=1
**standard-contracts (driver session readings) ↔ the Dioxus DOM bridge render path**: the driver's stand readings — the 68 of 77 clicks, the covered reading, the lint pins, the load page — all read `DioxusDocument::snapshot` and `element_ids` through the harness, so the buffered render in `initial_build` and `poll` must build the same document and ids for every one of them to hold in both layout modes (architecture/standard-contracts fct-t7zq5feb) (architecture fct-srb7slak)
@binding n=2
**standard-contracts (driver session) ↔ the test harness's settle**: the render-busy span reading and the load wait both run through `Harness::settle`, which drives the vdom render the chunk buffers, so the buffered `poll` must keep settle's quiet and `busy` answers as they read (architecture/standard-contracts fct-t7zq5feb) (architecture/standard-contracts fct-g75i5t3p) (architecture fct-gkkvjkmi)
@binding n=3
**standard-contracts (closed sets) ↔ the scope's nothing-crosses boundary**: no cause, verb, session error, span or span field is added and no log carries the buffer's mutations, the snapshot or a diff (architecture/standard-contracts fct-t7zq5feb) (architecture fct-ksgwc2yv) (architecture fct-tev7ieys)
