# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · fmt, clippy and rustdoc gate Rust code the chunk does not write, and no build, doc or test step reads README.md
conventions#2.1 · no · manifests and workspace dependency pins, which the chunk does not move
conventions#3.1 · no · Rust `//!` and `///` doc comments, which the chunk does not write; README.md is not a doc comment
conventions#4.1 · yes · TODO/FIXME sites in scrolling.rs, events/pointer.rs, dioxus-native-dom's mutation_writer.rs and blitz-paint's form_controls.rs record gaps in surfaces the statement re-reads, which it must not overclaim
conventions#5.1 · no · crate-internal visibility and naming, which the chunk changes no item of
conventions#6.1 · no · cargo-feature and tracing call-site gating; the chunk changes no feature, span or log field and the statement restates no sink
conventions#7.1 · no · unsafe blocks and borrow workarounds in engine code the chunk does not touch
conventions#8.1 · yes · names the witness checks stand_settle, stand_act_*, stand_mcp_tools, stand_screenshot_repeat and stand_requirement_sweep, which witness the settle, acting, MCP, screenshot and sweep surfaces the statement re-reads; its layout rules bind nothing
conventions#9.1 · yes · names the witness checks cli::tests, recording::tests and recording::off_unix_tests, which witness the command line, recording and its unix-only refusal the statement re-reads; its placement rules bind nothing
conventions#9.2 · no · the fixture module's boot-and-hold and HeldNet helper placement, which names no check and states no load or fixture behaviour the statement re-reads
conventions#10.1 · no · binary subsystem attributes and example layout, which the chunk does not write
conventions#11.1 · no · examples/assets HTML fixtures, which the chunk does not write
