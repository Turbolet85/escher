@answer n=1 entity=design-philosophy#1.1 v=no
why: its crate-root citation (lib.rs:3) and Cargo.toml:6 sit above every added line and the change adds only a private mod line.
@answer n=2 entity=design-philosophy#2.1 v=yes mode=stale
why: the bridge file gains lines from line 7 on, so its citations at dioxus_document.rs:58 and 84-85 have moved.
@answer n=3 entity=design-philosophy#3.1 v=no
why: its citations are all in blitz-dom, blitz-paint, blitz-net and other crates the change does not reach.
@answer n=4 entity=design-philosophy#4.1 v=no
why: its incremental pipeline claims cite blitz-dom style, layout and tree files the change does not reach.
@answer n=5 entity=design-philosophy#5.1 v=no
why: its headless harness and tests-directory claims cite blitz-tests and harness files the change does not reach, and the harness API is unchanged.
