@account n=1 entity=design-philosophy#1.1 v=carried
why: it names `dioxus-native-dom` the core headless renderer for Dioxus, the bridge whose escher-side change bounds the «exposed with a cost» verdict
@account n=2 entity=design-philosophy#2.1 v=carried
why: it states `DioxusDocument` pushing UI events through `handle_ui_event` and flushing through `poll`, the surface the probe reads and the hazard's `poll` it re-verifies
@account n=3 entity=design-philosophy#4.1 v=carried
why: the scope runs each verdict in both layout modes where layout touches the reading, and this states the two modes must produce identical layouts
@account n=4 entity=design-philosophy#5.1 v=carried
why: the verdicts are measured headlessly through the harness, whose synthesized input routes through the real event-dispatch pipeline
@account n=5 entity=stack-and-technologies#1.1 v=not_in_scope
why: the language, edition and rust-version every crate keeps; the probe moves none of them, a re-check every change owes
@account n=6 entity=stack-and-technologies#1.2 v=not_in_scope
why: the workspace version and the in-repo crate pins; the probe adds no crate and moves no in-repo version, a re-check every change owes
@account n=7 entity=stack-and-technologies#1.5 v=not_in_scope
why: Taffy's layout layer and version; no link of the UI graph reads geometry and the chunk moves no Taffy dependency
@account n=8 entity=stack-and-technologies#1.9 v=carried
why: the hazard's task is spawned from a key handler, and probe code driving key input beside a `dioxus::prelude::*` glob-import must name the workspace's `::keyboard_types`
@account n=9 entity=stack-and-technologies#1.13 v=carried
why: the probe reads the Dioxus 0.7.3 runtime crates only, and Dioxus stays pinned at 0.7.3 with no patch, vendored copy or pin move
@account n=10 entity=stack-and-technologies#1.20 v=carried
why: a probe check in blitz-tests reads the stand and the driver session through the dev-dependencies this lists, and any new one is recorded there
@account n=11 entity=stack-and-technologies#1.22 v=not_in_scope
why: the CI workflows, scripts, actions and host tools; the probe adds no job, script, action or tool, a re-check every change owes
@account n=12 entity=stack-and-technologies#1.23 v=carried
why: the verdicts rest on stand runs, and any stand check the probe keeps joins the `stand-repeat` leg's zero-failure budget
