@answer n=1 entity=design-philosophy#1.1 v=yes
why: the probe reads dioxus-native-dom as the headless bridge over BaseDocument, a separate crate it drives with no window, and its examples run through the root virtual package's cargo run --example.
@answer n=2 entity=design-philosophy#2.1 v=yes
why: the DioxusDocument bullet (handle_ui_event pushes UI events, poll flushes changes, dioxus_document.rs:83-84) is the surface the probe drives and the poll the MountedData re-read checks.
@answer n=3 entity=design-philosophy#3.1 v=no
why: the chunk adds no web semantics, and its spec-cited behaviours (ARIA, CSS, paint, networking, touch-action, implicit submission) are used rather than changed, with no ordering of default actions named in this entity.
@answer n=4 entity=design-philosophy#4.1 v=yes
why: incremental and non-incremental layout must produce identical layouts after any mutation (incremental_oracle.rs), the oracle behind task item 8 that runs each layout-touching verdict in both modes.
@answer n=5 entity=design-philosophy#5.1 v=yes
why: the probe runs on the headless harness with synthesized input routed through the real dispatch and measures through its poll(None), with no window, GPU or compositor.
