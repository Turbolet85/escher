# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · standing S5 · cargo fmt, clippy with -D warnings, rustdoc warnings as errors, and the locally allowed lints, all through ci-leg.sh
conventions#2.1 · no · the chunk adds no crate or dependency that a manifest rule would declare, and the task names none
conventions#3.1 · no · standing S4 · modules' //! docs, public items' ///, spec URLs beside implementations, and constants' rationale docs
conventions#4.1 · no · standing S6 · known gaps marked TODO or FIXME, the WARNING: DO NOT REORDER block, and the // === section markers
conventions#5.1 · yes · the scope names the choice writers crate-private and node::select holding the rules with no public type, the pub(crate) rule this entity records
conventions#6.1 · no · standing S7 · tracing gated per call site with a cfg fallback, and optional functionality behind cargo features, accessibility gating the snapshot modules
conventions#7.1 · no · the chunk adds no unsafe block and names no borrow workaround; the unsafe, borrow and let-chain habits are owed equally by every change of the kind
conventions#8.1 · no · standing S1 · unit-test homes and one-behaviour integration files (its layout-mode and shared-helper rules are S2 and S3)
conventions#9.1 · no · standing S1 · unit-test modules in #[path] child files, with cli, recording and mcp/schema tests moved out whole
conventions#9.2 · no · standing S3 · tests/blitz-tests/tests/fixtures/mod.rs holds the boot, hold and shared pages, and a page with one reader stays in its check's file
conventions#9.3 · no · the row-helper assertion style rules over mcp_tests.rs and stand_requirement_sweep.rs, neither reached by the chunk
conventions#10.1 · no · the example conventions (binary attributes, Android entries, Dioxus state and const CSS) are owed equally by every example change; the chunk adds no binary, app or example
conventions#11.1 · no · the chunk adds no HTML fixture and reaches no examples/assets file
