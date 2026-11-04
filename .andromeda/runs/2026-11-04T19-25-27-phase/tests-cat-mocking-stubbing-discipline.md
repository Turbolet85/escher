@carried n=1 label=test-plan fact=fct-xvhacwho
why: the probe's verdicts must rest on the real Dioxus 0.7.3 runtime and bridge; it may add no stand-in for the VirtualDom, a signal, the listener map or the document's poll (where the MountedData mechanism lives), and any stand-in it uses must be one of the plan's hand-written kinds.
quote: hand-written stand-ins replace the shell, the network, a document's poll, navigation, event handlers, widgets, the renderer, time, timer instrumentation, the WPT JS harness, worker publication, notify and the GitHub API
@carried n=2 label=test-plan/mocking-stubbing-discipline fact=fct-bz6tsqol
why: link (2)'s answer for `click` must be read from the bridge's listener map; the names `RecordingHandler` records are a stand-in event handler's and may not stand as that evidence.
quote: `RecordingHandler` EventHandler installed by dispatch_recorded records each dispatched DOM event name
@carried n=2 run=2 label=test-plan fact=fct-vu43sciq
quote: dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as Dioxus VirtualDom forwarding
@carried n=3 label=test-plan/mocking-stubbing-discipline fact=fct-eeplrkyg
why: the probe's lean and served tasks must boot through the stand's own boots, offline but for `stand::boot_load`'s `LoadNet`, with no network stand-in of its own and no real fetch.
quote: `stand::boot_load` replaces its options' `net_provider` with `Some(Arc::new(LoadNet))` through `..options`, every other boot keeping `net_provider: None` (`the_stand_options_stay_offline`); each `fetch` answers the fixed sheet `LOADED_CSS` for `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); it does not override `is_noop`, so the harness counts its load — `the_load_page_click_leaves_a_load_in_flight` reads it in flight and `stand_load_page`'s click waits it out and shows the message; no `blitz_net`, print, env read, socket or listener
@carried n=4 label=test-plan/mocking-stubbing-discipline fact=fct-c3fx7sq3
why: a probe check in `tests/blitz-tests/` that needs a load held must reuse the shared `HeldNet` through `mod fixtures;`, never a fourth hand-written network stand-in.
quote: `HeldNet`, escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and `stand_boot`'s `RecordingNetProvider`: `fetch` keeps each request's url and handler, `held()` and `urls()` (request order) read them, and `answer(body)` answers the last held request through `NetHandler::bytes` and returns its url
@carried n=5 label=test-plan fact=fct-ywhfajcc
why: the probe's runs on the stand, in both layout modes, must keep the real pinned font with system fonts off and take no skip path.
quote: The headless-stand checks keep a real but pinned font — the bundled DejaVu Sans woff2, decoded through seven_guis' native `dioxus-native` `woff` feature, with system fonts off and no skip
@carried n=6 label=test-plan fact=fct-nrlinzvj
why: the probe must add no mocking library (and no workspace crate or dependency), its stand-ins, if any, hand-written.
quote: Observed absent, each by its search: a mocking library in CI, the build configuration and the apps, and mocks in slices s03, s06–s09 and s11
@binding n=1
**Mocking discipline ↔ the scope's «Measured, not read» and «No fork-level change»**: each link's verdict is run on the real runtime through the stand, no stand-in replacing the VirtualDom, a signal, the listener map or `DioxusDocument`'s `poll`, and the probe adds no mocking library (test-plan fct-xvhacwho) (test-plan fct-nrlinzvj)
@binding n=2
**Mocking discipline ↔ test-plan §5 Harness ↔ event pipeline**: `dispatch_recorded`'s `RecordingHandler` bypasses Dioxus forwarding, so link (2)'s `click` reading comes from the bridge's listener map, not from recorded names (test-plan/mocking-stubbing-discipline fct-bz6tsqol) (test-plan fct-vu43sciq)
@binding n=3
**Mocking discipline ↔ the scope's «Nothing crosses» and test-plan §7 fixtures**: the network stand-ins the probe may meet, `LoadNet` and `HeldNet`, hold no print, env read, socket or listener, matching the probe's ban on any listener, port, env var or log; a held load is `tests/blitz-tests/tests/fixtures/mod.rs`'s `HeldNet` (test-plan/mocking-stubbing-discipline fct-eeplrkyg) (test-plan/mocking-stubbing-discipline fct-c3fx7sq3)
@binding n=4
**Mocking discipline ↔ test-plan §9 Fonts**: the stand's bundled DejaVu Sans, system fonts off, is the probe's font in both layout modes (test-plan fct-ywhfajcc)
